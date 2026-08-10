use std::sync::Mutex;

use crate::error::set_last_error;
use crate::font_manager::{get_font, resolve_char_with_fallback, Handle};
use fdsm::bezier::scanline::FillRule;
use fdsm::correct_error::{correct_error_msdf, ErrorCorrectionConfig};
use fdsm::generate::{generate_msdf, generate_sdf};
use fdsm::render::{correct_sign_msdf, correct_sign_sdf};
use fdsm::shape::Shape;
use fdsm::transform::Transform;
use fdsm_ttf_parser::load_shape_from_face;
use fdsm_ttf_parser::ttf_parser::{Face, GlyphId};
use image::{ImageBuffer, Luma, Rgb};
use nalgebra::{Affine2, Similarity2, Vector2};

// ─── Thread-local render state ────────────────────────────────────────────────

thread_local! {
    pub static TLS_RENDER_BUFFER: Mutex<RenderBuffer> = Mutex::new(RenderBuffer::default());
    pub static TLS_RENDER_PARAMS: Mutex<RenderParams> = Mutex::new(RenderParams::default());
    pub static TLS_RENDER_MODE: Mutex<RenderMode> = Mutex::new(RenderMode::Sdf);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderMode {
    Sdf = 0,
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

    let font_arc = match get_font(font_handle) {
        Some(f) => f,
        None => {
            set_last_error("invalid font handle");
            return false;
        }
    };

    let font_state = &*font_arc;

    let glyph_id_u16 = match u16::try_from(glyph_id) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("glyph id out of ttf range");
            return false;
        }
    };

    let face = match Face::parse(&font_state.font_data[..], 0) {
        Ok(f) => f,
        Err(_) => {
            set_last_error("failed to parse font for msdf generation");
            return false;
        }
    };

    let mut shape = match load_shape_from_face(&face, GlyphId(glyph_id_u16)) {
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
        let glyph_w_px = ((glyph_bbox.x_max as f64 - glyph_bbox.x_min as f64).max(1.0) * px_scale)
            .ceil()
            .max(1.0) as u32;
        let glyph_h_px = ((glyph_bbox.y_max as f64 - glyph_bbox.y_min as f64).max(1.0) * px_scale)
            .ceil()
            .max(1.0) as u32;

        let render_w = (glyph_w_px + padding.saturating_mul(2)).min(buf_w).max(1);
        let render_h = (glyph_h_px + padding.saturating_mul(2)).min(buf_h).max(1);
        let range = spread.max(1) as f64 * 2.0;

        let transformation: Affine2<f64> = nalgebra::convert(Similarity2::new(
            Vector2::new(
                padding as f64 - glyph_bbox.x_min as f64 * px_scale,
                padding as f64 - glyph_bbox.y_min as f64 * px_scale,
            ),
            0.0,
            px_scale,
        ));
        shape.transform(&transformation);

        match mode {
            RenderMode::Sdf | RenderMode::Psdf => {
                let prepared = shape.prepare();

                let mut sdf_bitmap = ImageBuffer::<Luma<f32>, Vec<f32>>::new(render_w, render_h);
                generate_sdf(&prepared, range, &mut sdf_bitmap);
                correct_sign_sdf(&mut sdf_bitmap, &prepared, FillRule::Nonzero);

                for y in 0..render_h as usize {
                    for x in 0..render_w as usize {
                        // fdsm output uses bottom-up orientation for font coordinates.
                        let src_y = render_h - 1 - y as u32;
                        let d = sdf_bitmap.get_pixel(x as u32, src_y).0[0];
                        let v = f32_to_u8(d);

                        // Universal RGBA8: replicate distance into R, zero G/B, full A.
                        let dst_idx = (y * width + x) * 4;
                        buf[dst_idx] = v;
                        buf[dst_idx + 1] = 0;
                        buf[dst_idx + 2] = 0;
                        buf[dst_idx + 3] = 255;
                    }
                }
            }
            RenderMode::Msdf | RenderMode::Mtsdf => {
                let sdf_prepared = if mode == RenderMode::Mtsdf {
                    Some(shape.prepare())
                } else {
                    None
                };

                let colored = Shape::edge_coloring_simple(shape, 0.03, 0);
                let prepared = colored.prepare();

                let mut msdf_bitmap = ImageBuffer::<Rgb<f32>, Vec<f32>>::new(render_w, render_h);
                generate_msdf(&prepared, range, &mut msdf_bitmap);
                correct_sign_msdf(&mut msdf_bitmap, &prepared, FillRule::Nonzero);
                correct_error_msdf(
                    &mut msdf_bitmap,
                    &colored,
                    &prepared,
                    range,
                    &ErrorCorrectionConfig::default(),
                );

                let mut mtsdf_alpha: Option<ImageBuffer<Luma<f32>, Vec<f32>>> = None;
                if let Some(prepared_sdf) = sdf_prepared.as_ref() {
                    let mut sdf_bitmap =
                        ImageBuffer::<Luma<f32>, Vec<f32>>::new(render_w, render_h);
                    generate_sdf(prepared_sdf, range, &mut sdf_bitmap);
                    correct_sign_sdf(&mut sdf_bitmap, prepared_sdf, FillRule::Nonzero);
                    mtsdf_alpha = Some(sdf_bitmap);
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
            }
        }
    }

    true
}

/// Compute exact glyph pixel bounds (width, height) and offsets for a given font size.
/// Returns zero-size bounds for whitespace / non-outline glyphs.
pub fn get_glyph_bounds(
    font_handle: Handle,
    glyph_id: u32,
    font_size: f64,
) -> Option<(u32, u32, f64, f64)> {
    let font_arc = match get_font(font_handle) {
        Some(f) => f,
        None => {
            set_last_error("get_glyph_bounds: invalid font handle");
            return None;
        }
    };
    let font_state = &*font_arc;

    let glyph_id_u16 = match u16::try_from(glyph_id) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("get_glyph_bounds: glyph id out of ttf range");
            return None;
        }
    };

    let face = match Face::parse(&font_state.font_data[..], 0) {
        Ok(f) => f,
        Err(e) => {
            set_last_error(&format!("get_glyph_bounds: failed to parse font: {:?}", e));
            return None;
        }
    };

    let glyph_bbox = match face.glyph_bounding_box(GlyphId(glyph_id_u16)) {
        Some(v) => v,
        None => {
            // Whitespace / non-outline glyph: valid zero-size bounds
            return Some((0, 0, 0.0, 0.0));
        }
    };

    let upem = face.units_per_em() as f64;
    let px_scale = (font_size / upem).max(0.0001);

    // Cast to f64 BEFORE subtraction to prevent i16 overflow on some fonts
    let w = ((glyph_bbox.x_max as f64 - glyph_bbox.x_min as f64).max(1.0) * px_scale)
        .ceil()
        .max(1.0) as u32;
    let h = ((glyph_bbox.y_max as f64 - glyph_bbox.y_min as f64).max(1.0) * px_scale)
        .ceil()
        .max(1.0) as u32;

    let x_min = glyph_bbox.x_min as f64 * px_scale;
    let y_max = glyph_bbox.y_max as f64 * px_scale;

    Some((w, h, x_min, y_max))
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
    let font_arc = get_font(font_handle)?;
    let face = Face::parse(&font_arc.font_data, 0).ok()?;

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

    Some((total_width as f64, line_height.max(font_size as f32) as f64))
}
