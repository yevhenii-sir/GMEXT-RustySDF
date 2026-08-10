use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use unicode_bidi::BidiInfo;

use crate::font_manager::{get_font, resolve_char_with_fallback, Handle};

static NEXT_SHAPE_HANDLE: LazyLock<Mutex<Handle>> = LazyLock::new(|| Mutex::new(1));

pub static SHAPE_HANDLES: LazyLock<Mutex<HashMap<Handle, ShapeResult>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Cached rustybuzz face: Arc keeps font bytes alive; Face lifetime is extended to 'static.
struct CachedFace {
    _data: Arc<Vec<u8>>,
    face: rustybuzz::Face<'static>,
    units_per_em: i32,
}

static FACE_CACHE: LazyLock<Mutex<HashMap<Handle, CachedFace>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const SHAPE_CACHE_MAX_ENTRIES: usize = 64;
const SHAPE_CACHE_MAX_GLYPHS: usize = 32_768;

#[derive(Clone, Hash, PartialEq, Eq)]
struct ShapeCacheKey {
    font_handle: Handle,
    size_bits: u64,
    text: String,
}

struct ShapeCache {
    map: HashMap<ShapeCacheKey, Arc<ShapeResult>>,
    order: VecDeque<ShapeCacheKey>,
    total_glyphs: usize,
}

impl ShapeCache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            total_glyphs: 0,
        }
    }

    fn get(&mut self, key: &ShapeCacheKey) -> Option<Arc<ShapeResult>> {
        if let Some(v) = self.map.get(key) {
            // move to MRU
            if let Some(i) = self.order.iter().position(|k| k == key) {
                if let Some(k) = self.order.remove(i) {
                    self.order.push_back(k);
                }
            }
            return Some(Arc::clone(v));
        }
        None
    }

    fn insert(&mut self, key: ShapeCacheKey, value: Arc<ShapeResult>) {
        let add = value.glyphs.len();
        if let Some(old) = self.map.insert(key.clone(), Arc::clone(&value)) {
            self.total_glyphs = self.total_glyphs.saturating_sub(old.glyphs.len());
            self.order.retain(|k| k != &key);
        }
        self.total_glyphs += add;
        self.order.push_back(key);

        while self.map.len() > SHAPE_CACHE_MAX_ENTRIES
            || self.total_glyphs > SHAPE_CACHE_MAX_GLYPHS
        {
            let Some(old_key) = self.order.pop_front() else {
                break;
            };
            if let Some(old) = self.map.remove(&old_key) {
                self.total_glyphs = self.total_glyphs.saturating_sub(old.glyphs.len());
            }
        }
    }

    fn invalidate_font(&mut self, font_handle: Handle) {
        let keys: Vec<_> = self
            .map
            .keys()
            .filter(|k| k.font_handle == font_handle)
            .cloned()
            .collect();
        for k in keys {
            if let Some(old) = self.map.remove(&k) {
                self.total_glyphs = self.total_glyphs.saturating_sub(old.glyphs.len());
            }
            self.order.retain(|x| x != &k);
        }
    }
}

static SHAPE_CACHE: LazyLock<Mutex<ShapeCache>> = LazyLock::new(|| Mutex::new(ShapeCache::new()));

// 0 = Auto (unicode-bidi), 1 = Force LTR, 2 = Force RTL
static BIDI_MODE: AtomicU32 = AtomicU32::new(0);

pub fn set_bidi_mode(mode: u32) {
    BIDI_MODE.store(mode, Ordering::SeqCst);
}

#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub font_handle: Handle,
    pub glyph_id: u32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub x_advance: f32,
    pub y_advance: f32,
    pub cluster: u32,
    pub char_code: u32,
}

#[derive(Debug, Clone)]
pub struct ShapeResult {
    pub width: f64,
    pub height: f64,
    pub glyphs: Vec<GlyphInfo>,
}

fn alloc_shape_handle() -> Handle {
    let mut next = NEXT_SHAPE_HANDLE.lock().unwrap_or_else(|e| e.into_inner());
    let h = *next;
    *next += 1;
    h
}

fn with_cached_face<R>(font_handle: Handle, f: impl FnOnce(&rustybuzz::Face<'static>, i32) -> R) -> Option<R> {
    // Fast path: already cached
    {
        let cache = FACE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cached) = cache.get(&font_handle) {
            return Some(f(&cached.face, cached.units_per_em));
        }
    }

    let font_arc = get_font(font_handle)?;
    let face = rustybuzz::Face::from_slice(&font_arc.font_data, 0)?;
    let units_per_em = face.units_per_em();
    // SAFETY: CachedFace keeps Arc<Vec<u8>> alive for as long as Face exists in the cache.
    let face_static: rustybuzz::Face<'static> = unsafe { std::mem::transmute(face) };

    let mut cache = FACE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let entry = cache.entry(font_handle).or_insert_with(|| CachedFace {
        _data: Arc::clone(&font_arc.font_data),
        face: face_static,
        units_per_em,
    });
    Some(f(&entry.face, entry.units_per_em))
}

/// Drop cached face + shaped runs when font is freed.
pub fn invalidate_face_cache(font_handle: Handle) {
    FACE_CACHE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&font_handle);
    SHAPE_CACHE
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .invalidate_font(font_handle);
}

fn text_needs_bidi(text: &str) -> bool {
    for ch in text.chars() {
        let u = ch as u32;
        // Rough: any RTL / complex script → full bidi. Pure Latin/Cyrillic/etc. skip.
        if (0x0590..=0x08FF).contains(&u) // Hebrew + Arabic blocks
            || (0xFB1D..=0xFDFF).contains(&u)
            || (0xFE70..=0xFEFF).contains(&u)
            || unicode_bidi::bidi_class(ch) == unicode_bidi::BidiClass::R
            || unicode_bidi::bidi_class(ch) == unicode_bidi::BidiClass::AL
        {
            return true;
        }
    }
    false
}

/// Shape text and return the result directly (no handle map).
/// Prefer this for internal rich/plain layout. Results are LRU-cached.
pub fn shape_text_result(font_handle: Handle, text: &str, font_size: f64) -> Arc<ShapeResult> {
    if text.is_empty() || font_handle == 0 {
        return Arc::new(ShapeResult {
            width: 0.0,
            height: font_size,
            glyphs: Vec::new(),
        });
    }

    let key = ShapeCacheKey {
        font_handle,
        size_bits: font_size.to_bits(),
        text: text.to_string(),
    };

    {
        let mut cache = SHAPE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(hit) = cache.get(&key) {
            return hit;
        }
    }

    let result = Arc::new(shape_text_uncached(font_handle, text, font_size));
    {
        let mut cache = SHAPE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        cache.insert(key, Arc::clone(&result));
    }
    result
}

fn shape_text_uncached(font_handle: Handle, text: &str, font_size: f64) -> ShapeResult {
    let mut glyphs = Vec::new();
    let mut total_width: f32 = 0.0;
    let mut max_height: f32 = 0.0;

    let bidi_mode = BIDI_MODE.load(Ordering::SeqCst);
    let mut visual_runs_data = Vec::new();

    if bidi_mode == 0 {
        if text_needs_bidi(text) {
            let bidi_info = BidiInfo::new(text, None);
            for para in &bidi_info.paragraphs {
                let (levels, runs) = bidi_info.visual_runs(para, para.range.clone());
                for run in runs {
                    let is_rtl = levels[run.start].is_rtl();
                    visual_runs_data.push((run, is_rtl));
                }
            }
        } else {
            visual_runs_data.push((0..text.len(), false));
        }
    } else {
        let is_rtl = bidi_mode == 2;
        visual_runs_data.push((0..text.len(), is_rtl));
    }

    for (run, is_rtl) in visual_runs_data {
        let run_text = &text[run.clone()];

        let mut font_runs = Vec::new();
        let mut current_font = font_handle;
        let mut current_start = 0;

        let chars: Vec<(usize, char)> = run_text.char_indices().collect();
        for (i, &(byte_idx, ch)) in chars.iter().enumerate() {
            let mut char_font = current_font;

            if !ch.is_whitespace() && !ch.is_control() {
                if let Some(resolved) = resolve_char_with_fallback(font_handle, ch) {
                    char_font = resolved.font_handle;
                }
            }

            if i == 0 {
                current_font = char_font;
            } else if char_font != current_font {
                font_runs.push((current_font, current_start..byte_idx));
                current_font = char_font;
                current_start = byte_idx;
            }
        }
        if current_start < run_text.len() {
            font_runs.push((current_font, current_start..run_text.len()));
        }

        for (run_font, byte_range) in font_runs {
            let sub_text = &run_text[byte_range.clone()];

            let shaped = with_cached_face(run_font, |face, units_per_em| {
                let scale = if units_per_em > 0 {
                    font_size as f32 / units_per_em as f32
                } else {
                    1.0
                };

                let mut is_arabic = false;
                let mut is_hebrew = false;
                for ch in sub_text.chars() {
                    let u = ch as u32;
                    if (u >= 0x0600 && u <= 0x06FF)
                        || (u >= 0x0750 && u <= 0x077F)
                        || (u >= 0x08A0 && u <= 0x08FF)
                        || (u >= 0xFB50 && u <= 0xFDFF)
                        || (u >= 0xFE70 && u <= 0xFEFF)
                    {
                        is_arabic = true;
                        break;
                    }
                    if u >= 0x0590 && u <= 0x05FF {
                        is_hebrew = true;
                        break;
                    }
                }

                let mut buffer = rustybuzz::UnicodeBuffer::new();
                buffer.push_str(sub_text);

                if is_rtl {
                    buffer.set_direction(rustybuzz::Direction::RightToLeft);
                    if is_arabic {
                        buffer.set_script(rustybuzz::script::ARABIC);
                        if let Ok(lang) = rustybuzz::Language::from_str("ar") {
                            buffer.set_language(lang);
                        }
                    } else if is_hebrew {
                        buffer.set_script(rustybuzz::script::HEBREW);
                        if let Ok(lang) = rustybuzz::Language::from_str("he") {
                            buffer.set_language(lang);
                        }
                    } else {
                        buffer.guess_segment_properties();
                    }
                } else {
                    buffer.set_direction(rustybuzz::Direction::LeftToRight);
                    buffer.guess_segment_properties();
                }

                let glyph_buffer = rustybuzz::shape(face, &[], buffer);
                let glyph_infos = glyph_buffer.glyph_infos();
                let glyph_positions = glyph_buffer.glyph_positions();

                let mut out = Vec::with_capacity(glyph_infos.len());
                for (info, pos) in glyph_infos.iter().zip(glyph_positions.iter()) {
                    let mut glyph_id = info.glyph_id;
                    let mut g_font_handle = run_font;

                    let cluster = (run.start + byte_range.start + info.cluster as usize) as u32;
                    let char_code = if (cluster as usize) < text.len() {
                        text[cluster as usize..]
                            .chars()
                            .next()
                            .map(|c| c as u32)
                            .unwrap_or(0)
                    } else {
                        0
                    };

                    if glyph_id == 0 {
                        if let Some(ch) = std::char::from_u32(char_code) {
                            if let Some(resolved) = resolve_char_with_fallback(font_handle, ch) {
                                glyph_id = resolved.glyph_id;
                                g_font_handle = resolved.font_handle;
                            }
                        }
                    }

                    let x_offset = pos.x_offset as f32 * scale;
                    let y_offset = pos.y_offset as f32 * scale;
                    let mut x_advance = pos.x_advance as f32 * scale;
                    let y_advance = pos.y_advance as f32 * scale;

                    if glyph_id != 0 && g_font_handle != run_font {
                        if let Some(fallback_arc) = get_font(g_font_handle) {
                            if let Some(ch) = std::char::from_u32(char_code) {
                                if let Ok(fb_face) = fdsm_ttf_parser::ttf_parser::Face::parse(
                                    &fallback_arc.font_data,
                                    0,
                                ) {
                                    if let Some(g_id) = fb_face.glyph_index(ch) {
                                        if let Some(advance) = fb_face.glyph_hor_advance(g_id) {
                                            let upe = fb_face.units_per_em() as f32;
                                            let fb_scale = if upe > 0.0 {
                                                font_size as f32 / upe
                                            } else {
                                                1.0
                                            };
                                            x_advance = advance as f32 * fb_scale;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    out.push((
                        GlyphInfo {
                            font_handle: g_font_handle,
                            glyph_id,
                            x_offset,
                            y_offset,
                            x_advance,
                            y_advance,
                            cluster,
                            char_code,
                        },
                        x_advance,
                        y_advance,
                    ));
                }
                out
            });

            if let Some(out) = shaped {
                for (g, xa, ya) in out {
                    max_height = max_height.max(ya);
                    total_width += xa;
                    glyphs.push(g);
                }
            }
        }
    }

    ShapeResult {
        width: total_width as f64,
        height: max_height.max(font_size as f32) as f64,
        glyphs,
    }
}

/// GML-facing API: shape into the global handle map.
pub fn shape_text(font_handle: Handle, text: &str, font_size: f64) -> Handle {
    let result = (*shape_text_result(font_handle, text, font_size)).clone();
    let handle = alloc_shape_handle();
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(handle, result);
    handle
}

pub fn free_shape(handle: Handle) -> bool {
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&handle)
        .is_some()
}

pub fn get_shape(handle: Handle) -> Option<ShapeResult> {
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)
        .cloned()
}

pub fn get_shape_glyphs_json(handle: Handle) -> Option<String> {
    let shape = SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)?
        .clone();
    let mut json = String::with_capacity(shape.glyphs.len() * 128);
    json.push('[');
    for (i, g) in shape.glyphs.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            "{{\"font_handle\":{},\"glyph_id\":{},\"x_offset\":{},\"y_offset\":{},\"x_advance\":{},\"y_advance\":{},\"cluster\":{},\"char_code\":{}}}",
            g.font_handle, g.glyph_id, g.x_offset, g.y_offset, g.x_advance, g.y_advance, g.cluster, g.char_code
        ));
    }
    json.push(']');
    Some(json)
}

#[allow(dead_code)]
pub fn write_shape_glyphs_buffer(
    handle: Handle,
    writer: &mut crate::gm_buffer::GMBufferWriter,
) -> Option<()> {
    let shape = SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)?
        .clone();
    let count = shape.glyphs.len() as u16;
    let elem_count = count * 8;

    writer.write_type(crate::gm_buffer::GMType::TypedArray);
    writer.data.extend_from_slice(&elem_count.to_le_bytes());
    writer.data.push(crate::gm_buffer::GMType::F64 as u8);

    for g in &shape.glyphs {
        writer
            .data
            .extend_from_slice(&(g.font_handle as f64).to_le_bytes());
        writer
            .data
            .extend_from_slice(&(g.glyph_id as f64).to_le_bytes());
        writer.data.extend_from_slice(&g.x_offset.to_le_bytes());
        writer.data.extend_from_slice(&g.y_offset.to_le_bytes());
        writer.data.extend_from_slice(&g.x_advance.to_le_bytes());
        writer.data.extend_from_slice(&g.y_advance.to_le_bytes());
        writer
            .data
            .extend_from_slice(&(g.cluster as f64).to_le_bytes());
        writer
            .data
            .extend_from_slice(&(g.char_code as f64).to_le_bytes());
    }

    Some(())
}

pub fn get_shape_glyph_count(handle: Handle) -> Option<u32> {
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)
        .map(|s| s.glyphs.len() as u32)
}

pub fn get_shape_glyph_info(handle: Handle, index: u32) -> Option<GlyphInfo> {
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)
        .and_then(|s| s.glyphs.get(index as usize).cloned())
}
