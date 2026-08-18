use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use unicode_bidi::BidiInfo;

use crate::font_manager::{get_font, Handle};

static NEXT_SHAPE_HANDLE: LazyLock<Mutex<Handle>> = LazyLock::new(|| Mutex::new(1));

pub static SHAPE_HANDLES: LazyLock<Mutex<HashMap<Handle, ShapeResult>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Cached rustybuzz face: Arc keeps font bytes alive; Face lifetime is extended to 'static.
struct CachedFace {
    _data: Arc<Vec<u8>>,
    face: rustybuzz::Face<'static>,
    units_per_em: i32,
}

static FACE_CACHE: LazyLock<Mutex<HashMap<Handle, Arc<CachedFace>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const SHAPE_CACHE_MAX_ENTRIES: usize = 64;
const SHAPE_CACHE_MAX_GLYPHS: usize = 32_768;

#[derive(Clone, Hash, PartialEq, Eq)]
struct ShapeCacheKey {
    font_handle: Handle,
    size_bits: u64,
    bidi_mode: u32,
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

thread_local! {
    /// Reused across shape calls on this worker (UnicodeBuffer <-> GlyphBuffer cycle).
    static TLS_UNICODE_BUF: RefCell<Option<rustybuzz::UnicodeBuffer>> = const { RefCell::new(None) };
}

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

/// Fetch (or build) a cached rustybuzz face. Lock is released before return.
fn get_cached_face(font_handle: Handle) -> Option<Arc<CachedFace>> {
    {
        let cache = FACE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cached) = cache.get(&font_handle) {
            return Some(Arc::clone(cached));
        }
    }

    let font_arc = get_font(font_handle)?;
    let face = rustybuzz::Face::from_slice(&font_arc.font_data, 0)?;
    let units_per_em = face.units_per_em();
    // SAFETY: CachedFace keeps Arc<Vec<u8>> alive for as long as Face exists in the cache.
    let face_static: rustybuzz::Face<'static> = unsafe { std::mem::transmute(face) };

    let entry = Arc::new(CachedFace {
        _data: Arc::clone(&font_arc.font_data),
        face: face_static,
        units_per_em,
    });

    let mut cache = FACE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let slot = cache.entry(font_handle).or_insert_with(|| Arc::clone(&entry));
    Some(Arc::clone(slot))
}

/// Glyph index via cached face (no per-call `Face::parse`).
pub(crate) fn cached_glyph_index(font_handle: Handle, ch: char) -> Option<u32> {
    let cached = get_cached_face(font_handle)?;
    Some(cached.face.glyph_index(ch)?.0 as u32)
}

/// Horizontal advance in font units + units_per_em via cached face.
pub(crate) fn cached_hor_advance(font_handle: Handle, glyph_id: u32) -> Option<(u16, i32)> {
    let cached = get_cached_face(font_handle)?;
    let gid = rustybuzz::ttf_parser::GlyphId(u16::try_from(glyph_id).ok()?);
    let advance = cached.face.glyph_hor_advance(gid)?;
    Some((advance, cached.units_per_em))
}

pub(crate) fn cached_glyph_count(font_handle: Handle) -> Option<u32> {
    let cached = get_cached_face(font_handle)?;
    Some(cached.face.number_of_glyphs() as u32)
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
        if (0x0590..=0x08FF).contains(&u)
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

fn take_unicode_buffer() -> rustybuzz::UnicodeBuffer {
    TLS_UNICODE_BUF.with(|cell| {
        cell.borrow_mut()
            .take()
            .unwrap_or_else(rustybuzz::UnicodeBuffer::new)
    })
}

fn store_unicode_buffer(buf: rustybuzz::UnicodeBuffer) {
    TLS_UNICODE_BUF.with(|cell| {
        *cell.borrow_mut() = Some(buf);
    });
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

    let bidi_mode = BIDI_MODE.load(Ordering::Relaxed);
    let key = ShapeCacheKey {
        font_handle,
        size_bits: font_size.to_bits(),
        bidi_mode,
        text: text.to_string(),
    };

    {
        let mut cache = SHAPE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(hit) = cache.get(&key) {
            return hit;
        }
    }

    let result = Arc::new(shape_text_uncached(font_handle, text, font_size, bidi_mode));
    {
        let mut cache = SHAPE_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        cache.insert(key, Arc::clone(&result));
    }
    result
}

fn shape_text_uncached(
    font_handle: Handle,
    text: &str,
    font_size: f64,
    bidi_mode: u32,
) -> ShapeResult {
    let mut glyphs = Vec::new();
    let mut total_width: f32 = 0.0;
    let mut max_height: f32 = 0.0;

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

        // Split into contiguous same-font sub-runs without allocating a char vec.
        let mut font_runs: Vec<(Handle, std::ops::Range<usize>)> = Vec::new();
        let mut current_font = font_handle;
        let mut current_start = 0usize;
        let mut started = false;

        for (byte_idx, ch) in run_text.char_indices() {
            let mut char_font = current_font;
            if !ch.is_whitespace() && !ch.is_control() {
                if let Some(resolved) = crate::font_manager::resolve_char_with_fallback(font_handle, ch)
                {
                    char_font = resolved.font_handle;
                }
            }

            if !started {
                current_font = char_font;
                current_start = byte_idx;
                started = true;
            } else if char_font != current_font {
                font_runs.push((current_font, current_start..byte_idx));
                current_font = char_font;
                current_start = byte_idx;
            }
        }
        if started && current_start < run_text.len() {
            font_runs.push((current_font, current_start..run_text.len()));
        }

        for (run_font, byte_range) in font_runs {
            let sub_text = &run_text[byte_range.clone()];
            let Some(cached) = get_cached_face(run_font) else {
                continue;
            };

            let scale = if cached.units_per_em > 0 {
                font_size as f32 / cached.units_per_em as f32
            } else {
                1.0
            };

            let mut is_arabic = false;
            let mut is_hebrew = false;
            for ch in sub_text.chars() {
                let u = ch as u32;
                if (0x0600..=0x06FF).contains(&u)
                    || (0x0750..=0x077F).contains(&u)
                    || (0x08A0..=0x08FF).contains(&u)
                    || (0xFB50..=0xFDFF).contains(&u)
                    || (0xFE70..=0xFEFF).contains(&u)
                {
                    is_arabic = true;
                    break;
                }
                if (0x0590..=0x05FF).contains(&u) {
                    is_hebrew = true;
                    break;
                }
            }

            let mut buffer = take_unicode_buffer();
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

            // Shape without holding FACE_CACHE lock (cached Arc keeps data alive).
            let glyph_buffer = rustybuzz::shape(&cached.face, &[], buffer);
            let glyph_infos = glyph_buffer.glyph_infos();
            let glyph_positions = glyph_buffer.glyph_positions();

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
                        if let Some(resolved) =
                            crate::font_manager::resolve_char_with_fallback(font_handle, ch)
                        {
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
                    if let Some((advance, upe)) = cached_hor_advance(g_font_handle, glyph_id) {
                        let fb_scale = if upe > 0 {
                            font_size as f32 / upe as f32
                        } else {
                            1.0
                        };
                        x_advance = advance as f32 * fb_scale;
                    }
                }

                max_height = max_height.max(y_advance);
                total_width += x_advance;
                glyphs.push(GlyphInfo {
                    font_handle: g_font_handle,
                    glyph_id,
                    x_offset,
                    y_offset,
                    x_advance,
                    y_advance,
                    cluster,
                    char_code,
                });
            }

            // Recycle buffer for the next sub-run.
            store_unicode_buffer(glyph_buffer.clear());
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

pub fn get_shape_glyph_count(handle: Handle) -> Option<u32> {
    SHAPE_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)
        .map(|s| s.glyphs.len() as u32)
}
