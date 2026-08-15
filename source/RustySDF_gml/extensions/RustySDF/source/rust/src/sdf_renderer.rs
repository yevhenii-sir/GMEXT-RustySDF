use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

use crate::error::set_last_error;
use crate::font_manager::{get_font, resolve_char_with_fallback, Handle};
use fdsm::bezier::scanline::FillRule;
use fdsm::correct_error::{correct_error_msdf, ErrorCorrectionConfig};
use fdsm::generate::{generate_msdf, generate_sdf, generate_sdf_banded_rgba8};
use fdsm::render::{correct_sign_msdf, correct_sign_sdf};
use fdsm::shape::Shape;
use fdsm::transform::Transform;
use fdsm_ttf_parser::load_shape_from_face;
use fdsm_ttf_parser::ttf_parser::{Face, GlyphId, Rect};
use image::{ImageBuffer, Luma, Rgb};
use nalgebra::{Affine2, Similarity2, Vector2};

// ─── Thread-local render state ────────────────────────────────────────────────

thread_local! {
    pub static TLS_RENDER_BUFFER: Mutex<RenderBuffer> = Mutex::new(RenderBuffer::default());
    pub static TLS_RENDER_PARAMS: Mutex<RenderParams> = Mutex::new(RenderParams::default());
    pub static TLS_RENDER_MODE: Mutex<RenderMode> = Mutex::new(RenderMode::Sdf);
    /// Reused f32 scratch for SDF/MSDF ImageBuffer generation (avoids alloc per glyph).
    static TLS_SDF_SCRATCH: RefCell<Vec<f32>> = RefCell::new(Vec::new());
    static TLS_MSDF_SCRATCH: RefCell<Vec<f32>> = RefCell::new(Vec::new());
    /// Per-worker cached ttf-parser faces (avoids Face::parse every glyph).
    static TLS_TTF_FACES: RefCell<HashMap<Handle, CachedTtfFace>> = RefCell::new(HashMap::new());
}

/// Cached ttf-parser face: Arc keeps font bytes alive; Face lifetime is extended to 'static.
struct CachedTtfFace {
    _data: Arc<Vec<u8>>,
    face: Face<'static>,
}

/// Fonts that were freed; TLS face caches drop entries on next use.
static FREED_TTF_FACES: LazyLock<Mutex<HashSet<Handle>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

fn with_cached_ttf_face<R>(font_handle: Handle, f: impl FnOnce(&Face<'static>) -> R) -> Option<R> {
    TLS_TTF_FACES.with(|cell| {
        let mut map = cell.borrow_mut();

        if FREED_TTF_FACES
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .contains(&font_handle)
        {
            map.remove(&font_handle);
            return None;
        }

        if !map.contains_key(&font_handle) {
            let font_arc = get_font(font_handle)?;
            let face = Face::parse(&font_arc.font_data[..], 0).ok()?;
            // SAFETY: CachedTtfFace keeps Arc<Vec<u8>> alive for as long as Face exists in TLS.
            let face_static: Face<'static> = unsafe { std::mem::transmute(face) };
            map.insert(
                font_handle,
                CachedTtfFace {
                    _data: Arc::clone(&font_arc.font_data),
                    face: face_static,
                },
            );
        }

        let face = &map.get(&font_handle).expect("face just inserted").face;
        // SAFETY: Face borrows font bytes held by CachedTtfFace in this map. We only
        // remove entries when the handle is freed (see invalidate) or on miss rebuild.
        // The callback must not call with_cached_ttf_face in a way that removes this entry.
        Some(f(face))
    })
}

/// Drop cached ttf-parser faces when a font is freed.
pub fn invalidate_ttf_face_cache(font_handle: Handle) {
    FREED_TTF_FACES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(font_handle);
    // Best-effort cleanup on the freeing thread.
    TLS_TTF_FACES.with(|cell| {
        cell.borrow_mut().remove(&font_handle);
    });
}

fn with_luma_scratch<R>(
    width: u32,
    height: u32,
    f: impl FnOnce(&mut ImageBuffer<Luma<f32>, Vec<f32>>) -> R,
) -> R {
    TLS_SDF_SCRATCH.with(|cell| {
        let mut storage = std::mem::take(&mut *cell.borrow_mut());
        let needed = (width as usize).saturating_mul(height as usize);
        storage.resize(needed, 0.0);
        storage.fill(0.0);
        let mut img = ImageBuffer::from_raw(width, height, storage)
            .expect("scratch length matches width*height");
        let result = f(&mut img);
        *cell.borrow_mut() = img.into_raw();
        result
    })
}

fn with_rgb_scratch<R>(
    width: u32,
    height: u32,
    f: impl FnOnce(&mut ImageBuffer<Rgb<f32>, Vec<f32>>) -> R,
) -> R {
    TLS_MSDF_SCRATCH.with(|cell| {
        let mut storage = std::mem::take(&mut *cell.borrow_mut());
        let needed = (width as usize)
            .saturating_mul(height as usize)
            .saturating_mul(3);
        storage.resize(needed, 0.0);
        storage.fill(0.0);
        let mut img = ImageBuffer::from_raw(width, height, storage)
            .expect("scratch length matches width*height*3");
        let result = f(&mut img);
        *cell.borrow_mut() = img.into_raw();
        result
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderMode {
    Sdf = 0,
    /// Alias of SDF (no separate PSDF generator).
    Psdf = 1,
    Msdf = 2,
    Mtsdf = 3,
}

impl RenderMode {
    fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::Sdf),
            1 => Some(Self::Psdf),
            2 => Some(Self::Msdf),
            3 => Some(Self::Mtsdf),
            _ => None,
        }
    }

    fn bytes_per_pixel(self) -> usize {
        // Universal RGBA8: all modes write 4 bytes per pixel for broad GPU compatibility.
        4
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderBuffer {
    pub ptr: *mut u8,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Default)]
pub struct RenderParams {
    pub padding: u32,
    pub spread: u32,
}

/// Set the render buffer (pointer + dimensions).
pub fn set_render_buffer(ptr: *mut u8, width: u32, height: u32) {
    TLS_RENDER_BUFFER.with(|cell| {
        let mut buf = cell.lock().unwrap_or_else(|e| e.into_inner());
        buf.ptr = ptr;
        buf.width = width;
        buf.height = height;
    });
}

/// Clear the render buffer pointer to prevent stale-pointer writes.
pub fn clear_render_buffer() {
    TLS_RENDER_BUFFER.with(|cell| {
        let mut buf = cell.lock().unwrap_or_else(|e| e.into_inner());
        buf.ptr = std::ptr::null_mut();
        buf.width = 0;
        buf.height = 0;
    });
}

/// Set render parameters (padding + spread).
pub fn set_render_params(padding: u32, spread: u32) {
    TLS_RENDER_PARAMS.with(|cell| {
        let mut params = cell.lock().unwrap_or_else(|e| e.into_inner());
        params.padding = padding;
        params.spread = spread;
    });
}

pub fn set_render_mode(mode: u32) -> bool {
    let parsed = match RenderMode::from_u32(mode) {
        Some(v) => v,
        None => {
            set_last_error("invalid render mode");
            return false;
        }
    };

    TLS_RENDER_MODE.with(|cell| {
        *cell.lock().unwrap_or_else(|e| e.into_inner()) = parsed;
    });
    true
}

pub fn get_render_mode() -> u32 {
    TLS_RENDER_MODE.with(|cell| *cell.lock().unwrap_or_else(|e| e.into_inner()) as u32)
}

pub fn get_render_bytes_per_pixel() -> u32 {
    TLS_RENDER_MODE.with(|cell| {
        cell.lock()
            .unwrap_or_else(|e| e.into_inner())
            .bytes_per_pixel() as u32
    })
}

/// Render a glyph SDF using the current thread-local buffer and params.
pub fn render_glyph_sdf(font_handle: Handle, glyph_id: u32, font_size: f64) -> bool {
    let mode = TLS_RENDER_MODE.with(|cell| *cell.lock().unwrap_or_else(|e| e.into_inner()));

    let (buffer, buf_w, buf_h) = TLS_RENDER_BUFFER.with(|cell| {
        let buf = cell.lock().unwrap_or_else(|e| e.into_inner());
        (buf.ptr, buf.width, buf.height)
    });

    let (padding, spread) = TLS_RENDER_PARAMS.with(|cell| {
        let params = cell.lock().unwrap_or_else(|e| e.into_inner());
        (params.padding, params.spread)
    });

    if buffer.is_null() {
        set_last_error("render buffer not set");
        return false;
    }

    if buf_w == 0 || buf_h == 0 {
        set_last_error("zero buffer dimensions");
        return false;
    }

    let glyph_id_u16 = match u16::try_from(glyph_id) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("glyph id out of ttf range");
            return false;
        }
    };

    let rendered = with_cached_ttf_face(font_handle, |face| {
        let mut shape = match load_shape_from_face(face, GlyphId(glyph_id_u16)) {
            Some(s) => s,
            None => {
                // Whitespace and non-outline glyphs are valid no-op renders.
                return true;
            }
        };

        let glyph_bbox = match face.glyph_bounding_box(GlyphId(glyph_id_u16)) {
            Some(v) => v,
            None => return true,
        };

        let width = buf_w as usize;
        let height = buf_h as usize;
        let bpp = mode.bytes_per_pixel();
        let total_bytes = width.saturating_mul(height).saturating_mul(bpp);

        if total_bytes == 0 {
            set_last_error("zero buffer size");
            return false;
        }

        unsafe {
            let buf = std::slice::from_raw_parts_mut(buffer, total_bytes);
            buf.fill(0);

            let upem = face.units_per_em() as f64;
            let px_scale = (font_size / upem).max(0.0001);
            let (glyph_w_px, glyph_h_px, _, _, y_slack) =
                glyph_tile_metrics(glyph_bbox, px_scale);

            let render_w = (glyph_w_px + padding.saturating_mul(2)).min(buf_w).max(1);
            let render_h = (glyph_h_px + padding.saturating_mul(2)).min(buf_h).max(1);
            let range = spread.max(1) as f64 * 2.0;

            let transformation: Affine2<f64> = nalgebra::convert(Similarity2::new(
                Vector2::new(
                    padding as f64 - glyph_bbox.x_min as f64 * px_scale,
                    padding as f64 + y_slack - glyph_bbox.y_min as f64 * px_scale,
                ),
                0.0,
                px_scale,
            ));
            shape.transform(&transformation);

            match mode {
                RenderMode::Sdf | RenderMode::Psdf => {
                    let prepared = shape.prepare();
                    // One-pass: banded analytic + sign + Y-flip + RGBA8 (parallel rows).
                    generate_sdf_banded_rgba8(
                        &prepared,
                        range,
                        buf,
                        buf_w,
                        render_w,
                        render_h,
                        true,
                    );
                }
                RenderMode::Msdf | RenderMode::Mtsdf => {
                    let sdf_prepared = if mode == RenderMode::Mtsdf {
                        Some(shape.prepare())
                    } else {
                        None
                    };

                    let colored = Shape::edge_coloring_simple(shape, 0.03, 0);
                    let prepared = colored.prepare();

                    with_rgb_scratch(render_w, render_h, |msdf_bitmap| {
                        generate_msdf(&prepared, range, msdf_bitmap);
                        correct_sign_msdf(msdf_bitmap, &prepared, FillRule::Nonzero);
                        correct_error_msdf(
                            msdf_bitmap,
                            &colored,
                            &prepared,
                            range,
                            &ErrorCorrectionConfig::default(),
                        );

                        let mut mtsdf_alpha: Option<ImageBuffer<Luma<f32>, Vec<f32>>> = None;
                        if let Some(prepared_sdf) = sdf_prepared.as_ref() {
                            with_luma_scratch(render_w, render_h, |sdf_bitmap| {
                                // MTSDF alpha uses full analytic SDF (MSDF path deferred).
                                generate_sdf(prepared_sdf, range, sdf_bitmap);
                                correct_sign_sdf(sdf_bitmap, prepared_sdf, FillRule::Nonzero);
                                mtsdf_alpha = Some(sdf_bitmap.clone());
                            });
                        }

                        for y in 0..render_h as usize {
                            for x in 0..render_w as usize {
                                let src_y = render_h - 1 - y as u32;
                                let ms = msdf_bitmap.get_pixel(x as u32, src_y).0;

                                let dst_idx = (y * width + x) * 4;
                                buf[dst_idx] = f32_to_u8(ms[0]);
                                buf[dst_idx + 1] = f32_to_u8(ms[1]);
                                buf[dst_idx + 2] = f32_to_u8(ms[2]);

                                let alpha = match &mtsdf_alpha {
                                    Some(sdf_bitmap) => {
                                        f32_to_u8(sdf_bitmap.get_pixel(x as u32, src_y).0[0])
                                    }
                                    None => 255,
                                };
                                buf[dst_idx + 3] = alpha;
                            }
                        }
                    });
                }
            }
        }

        true
    });

    match rendered {
        Some(ok) => ok,
        None => {
            set_last_error("invalid font handle or failed to parse font");
            false
        }
    }
}

fn glyph_tile_metrics(bbox: Rect, px_scale: f64) -> (u32, u32, f64, f64, f64) {
    let w_exact = (bbox.x_max as f64 - bbox.x_min as f64).max(1.0) * px_scale;
    let h_exact = (bbox.y_max as f64 - bbox.y_min as f64).max(1.0) * px_scale;
    let w = w_exact.ceil().max(1.0) as u32;
    let h = h_exact.ceil().max(1.0) as u32;
    let x_min = bbox.x_min as f64 * px_scale;
    let y_max = bbox.y_max as f64 * px_scale;
    let y_slack = h as f64 - h_exact;
    (w, h, x_min, y_max, y_slack)
}

/// Compute exact glyph pixel bounds (width, height) and offsets for a given font size.
/// Returns zero-size bounds for whitespace / non-outline glyphs.
pub fn get_glyph_bounds(
    font_handle: Handle,
    glyph_id: u32,
    font_size: f64,
) -> Option<(u32, u32, f64, f64)> {
    let glyph_id_u16 = match u16::try_from(glyph_id) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("get_glyph_bounds: glyph id out of ttf range");
            return None;
        }
    };

    match with_cached_ttf_face(font_handle, |face| {
        let glyph_bbox = match face.glyph_bounding_box(GlyphId(glyph_id_u16)) {
            Some(v) => v,
            None => {
                // Whitespace / non-outline glyph: valid zero-size bounds
                return (0, 0, 0.0, 0.0);
            }
        };

        let upem = face.units_per_em() as f64;
        let px_scale = (font_size / upem).max(0.0001);
        let (w, h, x_min, y_max, _) = glyph_tile_metrics(glyph_bbox, px_scale);
        (w, h, x_min, y_max)
    }) {
        Some(v) => Some(v),
        None => {
            set_last_error("get_glyph_bounds: invalid font handle");
            None
        }
    }
}

#[inline]
fn f32_to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Render a character SDF using the current thread-local buffer and params.
pub fn render_char_sdf(font_handle: Handle, char_code: u32, font_size: f64) -> bool {
    let ch = match std::char::from_u32(char_code) {
        Some(c) => c,
        None => {
            set_last_error("invalid character code");
            return false;
        }
    };

    let resolved = match resolve_char_with_fallback(font_handle, ch) {
        Some(v) => v,
        None => {
            set_last_error("glyph not found in primary font or fallback chain");
            return false;
        }
    };

    render_glyph_sdf(resolved.font_handle, resolved.glyph_id, font_size)
}

/// Quick measure text without creating a shape handle.
pub fn measure_text(font_handle: Handle, text: &str, font_size: f64) -> Option<(f64, f64)> {
    with_cached_ttf_face(font_handle, |face| {
        let units_per_em = face.units_per_em() as f32;
        let scale = if units_per_em > 0.0 {
            font_size as f32 / units_per_em
        } else {
            1.0
        };

        let mut total_width: f32 = 0.0;

        for ch in text.chars() {
            let g_id = face
                .glyph_index(ch)
                .unwrap_or(fdsm_ttf_parser::ttf_parser::GlyphId(0));
            if let Some(advance) = face.glyph_hor_advance(g_id) {
                total_width += advance as f32 * scale;
            }
        }

        // Line height from font metrics (ascender - descender, descender is negative)
        let ascender = face.ascender() as f32 * scale;
        let descender = face.descender() as f32 * scale;
        let line_height = ascender - descender;

        (total_width as f64, line_height.max(font_size as f32) as f64)
    })
}
