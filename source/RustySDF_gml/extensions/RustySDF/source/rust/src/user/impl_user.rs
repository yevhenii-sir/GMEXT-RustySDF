//! MANUAL: RustySDF business logic adapted into extgen user stubs.
//! Generated stubs were replaced once; extgen will NOT overwrite this file (IfMissing).
//! Pointer args are `*mut c_char` (extgen ABI) — cast to `*mut u8` for buffer APIs.
//! Bodies adapted from live rust/src/lib.rs via rust_generated user layer.

#![allow(unused_variables)]

use std::ffi::c_char;
use std::sync::Mutex;

use crate::async_renderer::{poll_glyph_result, request_glyph_async, GlyphResult};
use crate::atlas;
use crate::error::set_last_error;
use crate::font_manager::{
    add_fallback as fm_add_fallback, free_font as fm_free_font,
    get_font_glyph_count as fm_get_font_glyph_count, load_font as fm_load_font, Handle,
};
use crate::gm_buffer;
use crate::rich_layout::ImageInfo;
use crate::rich_parse::RichStyle;
use crate::rich_text;
use crate::rich_vertex::RICH_VERTEX_STRIDE;
use crate::sdf_renderer::{
    clear_render_buffer, get_glyph_bounds as sdf_get_glyph_bounds, get_render_bytes_per_pixel,
    get_render_mode, measure_text as sdf_measure_text, render_char_sdf, render_glyph_sdf,
    set_render_buffer, set_render_mode, set_render_params,
};
use crate::shaper::{
    free_shape as sh_free_shape, get_shape, get_shape_glyph_count as sh_get_shape_glyph_count,
    set_bidi_mode as sh_set_bidi_mode, shape_text as sh_shape_text,
};
use gm_ext_wire::{get_last_error_ptr, GMBuffer, StructStream};

thread_local! {
    static TLS_ASYNC_RESULT: Mutex<Option<GlyphResult>> = Mutex::new(None);
    static TLS_RICH_LAST_IMAGE: Mutex<Option<(Handle, String)>> = Mutex::new(None);
}

fn as_u8_mut(ptr: *mut c_char) -> *mut u8 {
    ptr as *mut u8
}

// ─── Font ───────────────────────────────────────────────────────────────────

pub fn rusty_sdf_load_font(data: GMBuffer) -> f64 {
    let slice = data.as_slice();
    if slice.is_empty() {
        set_last_error("empty font buffer");
        return -1.0;
    }
    let handle = fm_load_font(slice.to_vec());
    if handle == 0 {
        return -1.0;
    }
    handle as f64
}

pub fn rusty_sdf_free_font(font_handle: f64) -> f64 {
    if fm_free_font(font_handle as Handle) {
        0.0
    } else {
        set_last_error("invalid font handle");
        -1.0
    }
}

pub fn rusty_sdf_add_fallback(font_handle: f64, fallback_handle: f64) -> f64 {
    if fm_add_fallback(font_handle as Handle, fallback_handle as Handle) {
        0.0
    } else {
        -1.0
    }
}

pub fn rusty_sdf_get_font_glyph_count(font_handle: f64) -> f64 {
    match fm_get_font_glyph_count(font_handle as Handle) {
        Some(count) => count as f64,
        None => {
            set_last_error("invalid font handle");
            -1.0
        }
    }
}

// ─── Shape ──────────────────────────────────────────────────────────────────

pub fn rusty_sdf_shape_text(font_handle: f64, text: &str, font_size: f64) -> f64 {
    let handle = sh_shape_text(font_handle as Handle, text, font_size);
    if handle == 0 {
        return -1.0;
    }
    handle as f64
}

pub fn rusty_sdf_free_shape(shape_handle: f64) -> f64 {
    if sh_free_shape(shape_handle as Handle) {
        0.0
    } else {
        set_last_error("invalid shape handle");
        -1.0
    }
}

pub fn rusty_sdf_get_shape_width(shape_handle: f64) -> f64 {
    match get_shape(shape_handle as Handle) {
        Some(result) => result.width,
        None => {
            set_last_error("invalid shape handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_get_shape_height(shape_handle: f64) -> f64 {
    match get_shape(shape_handle as Handle) {
        Some(result) => result.height,
        None => {
            set_last_error("invalid shape handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_get_shape_glyph_count(shape_handle: f64) -> f64 {
    match sh_get_shape_glyph_count(shape_handle as Handle) {
        Some(count) => count as f64,
        None => {
            set_last_error("invalid shape handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_get_shape_glyphs_buffer(
    shape_handle: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    if buffer_ptr.is_null() {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    let len = buffer_len as usize;
    if len == 0 {
        set_last_error("zero buffer length");
        return -1.0;
    }
    let shape = match get_shape(shape_handle as Handle) {
        Some(s) => s,
        None => {
            set_last_error("invalid shape handle");
            return -1.0;
        }
    };

    let mut values = Vec::with_capacity(shape.glyphs.len() * 8);
    for g in &shape.glyphs {
        values.push(g.font_handle as f64);
        values.push(g.glyph_id as f64);
        values.push(g.x_offset as f64);
        values.push(g.y_offset as f64);
        values.push(g.x_advance as f64);
        values.push(g.y_advance as f64);
        values.push(g.cluster as f64);
        values.push(g.char_code as f64);
    }

    let mut writer = gm_buffer::GMBufferWriter::new(values.len() * 8 + 4);
    writer.write_f64_typed_array(&values);
    let data_len = writer.data.len();
    if data_len > len {
        set_last_error("buffer too small for glyph data");
        return -2.0;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(writer.data.as_ptr(), as_u8_mut(buffer_ptr), data_len);
    }
    data_len as f64
}

pub fn rusty_sdf_set_bidi_mode(mode: f64) -> f64 {
    sh_set_bidi_mode(mode as u32);
    0.0
}

// ─── Render state ───────────────────────────────────────────────────────────

pub fn rusty_sdf_set_buffer(buffer_ptr: *mut c_char, buf_w: f64, buf_h: f64) -> f64 {
    if buffer_ptr.is_null() {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    set_render_buffer(as_u8_mut(buffer_ptr), buf_w as u32, buf_h as u32);
    0.0
}

pub fn rusty_sdf_set_params(padding: f64, spread: f64) -> f64 {
    set_render_params(padding as u32, spread as u32);
    0.0
}

pub fn rusty_sdf_set_mode(mode: f64) -> f64 {
    if set_render_mode(mode as u32) {
        0.0
    } else {
        -1.0
    }
}

pub fn rusty_sdf_get_mode() -> f64 {
    get_render_mode() as f64
}

pub fn rusty_sdf_get_buffer_bpp() -> f64 {
    get_render_bytes_per_pixel() as f64
}

pub fn rusty_sdf_get_glyph_bounds(
    font_handle: f64,
    glyph_id: f64,
    font_size: f64,
) -> StructStream {
    let mut s = StructStream::new();
    match sdf_get_glyph_bounds(font_handle as Handle, glyph_id as u32, font_size) {
        Some((w, h, x_min, y_max)) => {
            s.add_f64("width", w as f64);
            s.add_f64("height", h as f64);
            s.add_f64("x_min", x_min);
            s.add_f64("y_max", y_max);
        }
        None => {
            set_last_error("invalid font handle, glyph id, or glyph has no outline");
        }
    }
    s
}

pub fn rusty_sdf_render_glyph(font_handle: f64, glyph_id: f64, font_size: f64) -> f64 {
    let ok = render_glyph_sdf(font_handle as Handle, glyph_id as u32, font_size);
    clear_render_buffer();
    if ok {
        0.0
    } else {
        -1.0
    }
}

pub fn rusty_sdf_render_char(font_handle: f64, char_code: f64, font_size: f64) -> f64 {
    let ok = render_char_sdf(font_handle as Handle, char_code as u32, font_size);
    clear_render_buffer();
    if ok {
        0.0
    } else {
        -1.0
    }
}

// ─── Async ──────────────────────────────────────────────────────────────────

pub fn rusty_sdf_request_glyph(
    font_handle: f64,
    glyph_id: f64,
    font_size: f64,
    padding: f64,
    spread: f64,
    mode: f64,
) -> f64 {
    let ok = request_glyph_async(
        font_handle as Handle,
        glyph_id as u32,
        font_size,
        padding as u32,
        spread as u32,
        mode as u32,
    );
    if ok {
        1.0
    } else {
        0.0
    }
}

pub fn rusty_sdf_poll_glyph() -> StructStream {
    let mut s = StructStream::new();
    match poll_glyph_result() {
        Some(result) => {
            s.add_f64("font_handle", result.font_handle as f64);
            s.add_f64("glyph_id", result.glyph_id as f64);
            s.add_f64("font_size", result.font_size);
            s.add_f64("padding", result.padding as f64);
            s.add_f64("spread", result.spread as f64);
            s.add_f64("width", result.width as f64);
            s.add_f64("height", result.height as f64);
            s.add_f64("raw_w", result.raw_w as f64);
            s.add_f64("raw_h", result.raw_h as f64);
            s.add_f64("x_min", result.x_min);
            s.add_f64("y_max", result.y_max);
            TLS_ASYNC_RESULT.with(|cell| {
                *cell.lock().unwrap_or_else(|e| e.into_inner()) = Some(result);
            });
        }
        None => {}
    }
    s
}

pub fn rusty_sdf_poll_glyph_pixels(dst: GMBuffer) -> f64 {
    let len = dst.len as usize;
    if dst.ptr.is_null() || len == 0 {
        set_last_error("empty destination buffer");
        return -1.0;
    }
    let mut copied_len = -1.0;
    TLS_ASYNC_RESULT.with(|cell| {
        let mut guard = cell.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(result) = guard.take() {
            let pixel_len = result.pixels.len();
            if pixel_len == 0 {
                copied_len = 0.0;
            } else {
                let copy_len = pixel_len.min(len);
                unsafe {
                    std::ptr::copy_nonoverlapping(result.pixels.as_ptr(), dst.ptr, copy_len);
                }
                copied_len = copy_len as f64;
            }
        } else {
            set_last_error("no async result waiting");
        }
    });
    copied_len
}

/// Copy last async glyph into a destination buffer laid out for `buffer_set_surface`.
/// Destination is `stride_w * stride_h` RGBA8 pixels (row-major). Glyph is top-left;
/// the full destination is zeroed first so padding matches the surface.
pub fn rusty_sdf_poll_glyph_pixels_strided(dst: GMBuffer, stride_w: f64, stride_h: f64) -> f64 {
    if dst.ptr.is_null() {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    if stride_w < 1.0 || stride_h < 1.0 {
        set_last_error("invalid stride dimensions");
        return -1.0;
    }

    let stride_w = stride_w as usize;
    let stride_h = stride_h as usize;
    let need = stride_w
        .checked_mul(stride_h)
        .and_then(|px| px.checked_mul(4))
        .unwrap_or(0);
    let len = dst.len as usize;
    if need == 0 || len < need {
        set_last_error("buffer too small for strided glyph upload");
        return -1.0;
    }

    let mut copied_len = -1.0;
    TLS_ASYNC_RESULT.with(|cell| {
        let mut guard = cell.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(result) = guard.take() {
            let gw = result.width as usize;
            let gh = result.height as usize;
            if gw == 0 || gh == 0 || result.pixels.is_empty() {
                unsafe {
                    std::ptr::write_bytes(dst.ptr, 0, need);
                }
                copied_len = 0.0;
                return;
            }
            if gw > stride_w || gh > stride_h {
                set_last_error("glyph larger than upload stride");
                *guard = Some(result);
                copied_len = -2.0;
                return;
            }

            let src_stride = gw * 4;
            let dst_stride = stride_w * 4;
            let expected = src_stride.saturating_mul(gh);
            if result.pixels.len() < expected {
                set_last_error("async pixel buffer shorter than glyph size");
                copied_len = -1.0;
                return;
            }

            let out = unsafe { std::slice::from_raw_parts_mut(dst.ptr, need) };
            out.fill(0);
            for y in 0..gh {
                let src_off = y * src_stride;
                let dst_off = y * dst_stride;
                out[dst_off..dst_off + src_stride]
                    .copy_from_slice(&result.pixels[src_off..src_off + src_stride]);
            }
            copied_len = need as f64;
        } else {
            set_last_error("no async result waiting");
        }
    });
    copied_len
}

// ─── Utility ────────────────────────────────────────────────────────────────

pub fn rusty_sdf_measure_text(font_handle: f64, text: String, font_size: f64) -> StructStream {
    let mut s = StructStream::new();
    match sdf_measure_text(font_handle as Handle, &text, font_size) {
        Some((width, height)) => {
            s.add_f64("width", width);
            s.add_f64("height", height);
        }
        None => {
            set_last_error("invalid font handle");
        }
    }
    s
}

pub fn rusty_sdf_ping() -> String {
    format!("RustySDF v{}", env!("CARGO_PKG_VERSION"))
}

pub fn rusty_sdf_get_last_error() -> String {
    let ptr = get_last_error_ptr();
    if ptr.is_null() {
        return String::new();
    }
    unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned()
}

fn write_f64_array_to_ptr(values: &[f64], buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    if buffer_ptr.is_null() {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    let len = buffer_len as usize;
    if len == 0 {
        set_last_error("zero buffer length");
        return -1.0;
    }
    let mut writer = gm_buffer::GMBufferWriter::new(values.len() * 8 + 4);
    writer.write_f64_typed_array(values);
    let data_len = writer.data.len();
    if data_len > len {
        set_last_error("buffer too small");
        return -2.0;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(writer.data.as_ptr(), as_u8_mut(buffer_ptr), data_len);
    }
    data_len as f64
}

fn write_f64_array_to_gmbuffer(values: &[f64], dst: GMBuffer) -> f64 {
    write_f64_array_to_ptr(values, dst.ptr as *mut c_char, dst.len as f64)
}

// ─── Atlas ──────────────────────────────────────────────────────────────────

pub fn rusty_sdf_atlas_init(width: f64, height: f64, padding: f64) -> f64 {
    atlas::init(
        width.max(1.0) as u32,
        height.max(1.0) as u32,
        padding.max(0.0) as u32,
    );
    0.0
}

pub fn rusty_sdf_atlas_reset() -> f64 {
    atlas::reset();
    0.0
}

pub fn rusty_sdf_atlas_clear() -> f64 {
    atlas::clear_pages();
    0.0
}

pub fn rusty_sdf_atlas_get_version() -> f64 {
    atlas::get_version() as f64
}

pub fn rusty_sdf_atlas_page_count() -> f64 {
    atlas::page_count() as f64
}

pub fn rusty_sdf_atlas_ensure_glyph(
    font_handle: f64,
    glyph_id: f64,
    base_size: f64,
    spread: f64,
    mode: f64,
    async_flag: f64,
) -> f64 {
    let fh = font_handle as Handle;
    let gid = glyph_id as u32;
    if let Some(e) = atlas::lookup(fh, gid, base_size, spread) {
        if e.async_pending {
            return 0.0;
        }
        return 1.0;
    }
    if async_flag != 0.0 {
        let pad = atlas::get_padding().max((spread.round() as u32).saturating_add(1));
        request_glyph_async(fh, gid, base_size, pad, spread.round() as u32, mode as u32);
        atlas::mark_pending(fh, gid, base_size, spread);
        return 0.0;
    }
    atlas::ensure_glyph_sync(fh, gid, base_size, spread, mode as u32) as f64
}

pub fn rusty_sdf_atlas_lookup(
    font_handle: f64,
    glyph_id: f64,
    base_size: f64,
    spread: f64,
) -> StructStream {
    let mut s = StructStream::new();
    match atlas::lookup(
        font_handle as Handle,
        glyph_id as u32,
        base_size,
        spread,
    ) {
        Some(e) => {
            let vals = atlas::entry_to_f64s(&e);
            s.add_f64("found", vals[0]);
            s.add_f64("page_index", vals[1]);
            s.add_f64("atlas_x", vals[2]);
            s.add_f64("atlas_y", vals[3]);
            s.add_f64("w", vals[4]);
            s.add_f64("h", vals[5]);
            s.add_f64("raw_w", vals[6]);
            s.add_f64("raw_h", vals[7]);
            s.add_f64("u1", vals[8]);
            s.add_f64("v1", vals[9]);
            s.add_f64("u2", vals[10]);
            s.add_f64("v2", vals[11]);
            s.add_f64("x_min", vals[12]);
            s.add_f64("y_max", vals[13]);
            s.add_f64("async_pending", vals[14]);
        }
        None => {
            s.add_f64("found", 0.0);
        }
    }
    s
}

pub fn rusty_sdf_atlas_commit_glyph(
    font_handle: f64,
    glyph_id: f64,
    base_size: f64,
    spread: f64,
    width: f64,
    height: f64,
    raw_w: f64,
    raw_h: f64,
    x_min: f64,
    y_max: f64,
) -> f64 {
    match atlas::commit_glyph(
        font_handle as Handle,
        glyph_id as u32,
        base_size,
        spread,
        width.max(0.0) as u32,
        height.max(0.0) as u32,
        raw_w.max(0.0) as u32,
        raw_h.max(0.0) as u32,
        x_min,
        y_max,
    ) {
        Some(_) => 1.0,
        None => -1.0,
    }
}

pub fn rusty_sdf_atlas_poll_dirty_meta(dst: GMBuffer) -> f64 {
    match atlas::poll_dirty_meta() {
        Some((page, x, y, w, h)) => {
            write_f64_array_to_gmbuffer(&[page as f64, x as f64, y as f64, w as f64, h as f64], dst)
        }
        None => 0.0,
    }
}

pub fn rusty_sdf_atlas_poll_dirty_pixels(dst: GMBuffer) -> f64 {
    if dst.ptr.is_null() || dst.len == 0 {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    let slice = unsafe { std::slice::from_raw_parts_mut(dst.ptr, dst.len as usize) };
    match atlas::poll_dirty_pixels(slice) {
        Some(n) => n as f64,
        None => 0.0,
    }
}

// ─── Rich text ──────────────────────────────────────────────────────────────

pub fn rusty_sdf_rich_create() -> f64 {
    rich_text::create() as f64
}

pub fn rusty_sdf_rich_free(handle: f64) -> f64 {
    if rich_text::free(handle as Handle) {
        0.0
    } else {
        set_last_error("invalid rich text handle");
        -1.0
    }
}

pub fn rusty_sdf_rich_set_text(handle: f64, text: &str) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        if st.text != text {
            st.text = text.to_string();
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_rich_set_font(
    handle: f64,
    font_handle: f64,
    font_size: f64,
    base_size: f64,
    spread: f64,
) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        let fh = font_handle as Handle;
        if st.font_handle != fh
            || st.font_size != font_size
            || st.base_size != base_size
            || st.spread != spread
        {
            st.font_handle = fh;
            st.font_size = font_size;
            st.base_size = base_size;
            st.spread = spread;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_rich_set_layout(
    handle: f64,
    max_width: f64,
    line_height: f64,
    letter_spacing: f64,
    halign: f64,
    valign: f64,
) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        let mw = max_width as f32;
        let lh = line_height as f32;
        let ls = letter_spacing as f32;
        let ha = halign as i32;
        let va = valign as i32;
        if st.max_width != mw
            || st.line_height != lh
            || st.letter_spacing != ls
            || st.halign != ha
            || st.valign != va
        {
            st.max_width = mw;
            st.line_height = lh;
            st.letter_spacing = ls;
            st.halign = ha;
            st.valign = va;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

/// Typed f64 array: c, a, b, oc, oa, ow, gc, ga, gr, ul, st (11 values). Colors are GM packed ints.
pub fn rusty_sdf_rich_set_default_style(
    handle: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    if buffer_ptr.is_null() || buffer_len < 4.0 {
        set_last_error("null/short style buffer");
        return -1.0;
    }
    let slice = unsafe { std::slice::from_raw_parts(as_u8_mut(buffer_ptr), buffer_len as usize) };
    // Accept either raw f64[11] or typed-array wrapper. Prefer typed unpack if tag present.
    let values: Vec<f64> = if slice.first() == Some(&250) {
        // typed array: tag, u16 count, u8 elem, f64...
        if slice.len() < 4 {
            return -1.0;
        }
        let count = u16::from_le_bytes([slice[1], slice[2]]) as usize;
        let mut out = Vec::with_capacity(count);
        let mut off = 4usize;
        for _ in 0..count {
            if off + 8 > slice.len() {
                break;
            }
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[off..off + 8]);
            out.push(f64::from_le_bytes(b));
            off += 8;
        }
        out
    } else {
        let n = (slice.len() / 8).min(11);
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[i * 8..(i + 1) * 8]);
            out.push(f64::from_le_bytes(b));
        }
        out
    };
    if values.len() < 9 {
        set_last_error("style buffer needs at least 9 floats");
        return -1.0;
    }
    let style = RichStyle {
        c: values[0] as u32,
        a: values[1] as f32,
        b: values[2] as f32,
        oc: values[3] as u32,
        oa: values[4] as f32,
        ow: values[5] as f32,
        gc: values[6] as u32,
        ga: values[7] as f32,
        gr: values[8] as f32,
        ul: values.get(9).copied().unwrap_or(0.0) != 0.0,
        st: values.get(10).copied().unwrap_or(0.0) != 0.0,
    };
    match rich_text::with_mut(handle as Handle, |st| {
        if st.default_style.c != style.c
            || st.default_style.a != style.a
            || st.default_style.b != style.b
            || st.default_style.oc != style.oc
            || st.default_style.oa != style.oa
            || st.default_style.ow != style.ow
            || st.default_style.gc != style.gc
            || st.default_style.ga != style.ga
            || st.default_style.gr != style.gr
            || st.default_style.ul != style.ul
            || st.default_style.st != style.st
        {
            st.default_style = style;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_rich_set_async(handle: f64, enabled: f64) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        let on = enabled != 0.0;
        if st.async_enabled != on {
            st.async_enabled = on;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => -1.0,
    }
}

pub fn rusty_sdf_rich_set_plain(handle: f64, enabled: f64) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        let on = enabled != 0.0;
        if st.plain_mode != on {
            st.plain_mode = on;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => -1.0,
    }
}

/// Packed f64 config (raw or typed-array):
/// 0 font_handle, 1 font_size, 2 base_size, 3 spread,
/// 4 max_width, 5 line_height, 6 letter_spacing, 7 halign, 8 valign,
/// 9 plain, 10 async,
/// 11..21 style: c,a,b,oc,oa,ow,gc,ga,gr,ul,st
pub fn rusty_sdf_rich_set_config(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    if buffer_ptr.is_null() || buffer_len < 4.0 {
        set_last_error("null/short config buffer");
        return -1.0;
    }
    let slice = unsafe { std::slice::from_raw_parts(as_u8_mut(buffer_ptr), buffer_len as usize) };
    let values: Vec<f64> = if slice.first() == Some(&250) {
        if slice.len() < 4 {
            return -1.0;
        }
        let count = u16::from_le_bytes([slice[1], slice[2]]) as usize;
        let mut out = Vec::with_capacity(count);
        let mut off = 4usize;
        for _ in 0..count {
            if off + 8 > slice.len() {
                break;
            }
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[off..off + 8]);
            out.push(f64::from_le_bytes(b));
            off += 8;
        }
        out
    } else {
        let n = slice.len() / 8;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let mut b = [0u8; 8];
            b.copy_from_slice(&slice[i * 8..(i + 1) * 8]);
            out.push(f64::from_le_bytes(b));
        }
        out
    };
    if values.len() < 22 {
        set_last_error("config buffer needs 22 floats");
        return -1.0;
    }
    let style = RichStyle {
        c: values[11] as u32,
        a: values[12] as f32,
        b: values[13] as f32,
        oc: values[14] as u32,
        oa: values[15] as f32,
        ow: values[16] as f32,
        gc: values[17] as u32,
        ga: values[18] as f32,
        gr: values[19] as f32,
        ul: values[20] != 0.0,
        st: values[21] != 0.0,
    };
    match rich_text::with_mut(handle as Handle, |st| {
        let fh = values[0] as Handle;
        let font_size = values[1];
        let base_size = values[2];
        let spread = values[3];
        let mw = values[4] as f32;
        let lh = values[5] as f32;
        let ls = values[6] as f32;
        let ha = values[7] as i32;
        let va = values[8] as i32;
        let plain = values[9] != 0.0;
        let async_on = values[10] != 0.0;
        let changed = st.font_handle != fh
            || st.font_size != font_size
            || st.base_size != base_size
            || st.spread != spread
            || st.max_width != mw
            || st.line_height != lh
            || st.letter_spacing != ls
            || st.halign != ha
            || st.valign != va
            || st.plain_mode != plain
            || st.async_enabled != async_on
            || st.default_style.c != style.c
            || st.default_style.a != style.a
            || st.default_style.b != style.b
            || st.default_style.oc != style.oc
            || st.default_style.oa != style.oa
            || st.default_style.ow != style.ow
            || st.default_style.gc != style.gc
            || st.default_style.ga != style.ga
            || st.default_style.gr != style.gr
            || st.default_style.ul != style.ul
            || st.default_style.st != style.st;
        if changed {
            st.font_handle = fh;
            st.font_size = font_size;
            st.base_size = base_size;
            st.spread = spread;
            st.max_width = mw;
            st.line_height = lh;
            st.letter_spacing = ls;
            st.halign = ha;
            st.valign = va;
            st.plain_mode = plain;
            st.async_enabled = async_on;
            st.default_style = style;
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_rich_register_image(handle: f64, name: &str) -> f64 {
    let h = handle as Handle;
    match rich_text::with_mut(h, |st| {
        st.images.insert(
            name.to_string(),
            ImageInfo {
                width: 0.0,
                height: 1.0,
                xoffset: 0.0,
                yoffset: 0.0,
            },
        );
        st.built = None;
    }) {
        Some(()) => {
            TLS_RICH_LAST_IMAGE.with(|cell| {
                *cell.lock().unwrap_or_else(|e| e.into_inner()) = Some((h, name.to_string()));
            });
            0.0
        }
        None => -1.0,
    }
}

pub fn rusty_sdf_rich_set_image_metrics(
    handle: f64,
    spr_w: f64,
    spr_h: f64,
    xoff: f64,
    yoff: f64,
) -> f64 {
    let h = handle as Handle;
    let name = TLS_RICH_LAST_IMAGE.with(|cell| {
        cell.lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .filter(|(hh, _)| *hh == h)
            .map(|(_, n)| n.clone())
    });
    let Some(name) = name else {
        set_last_error("rich_register_image not called");
        return -1.0;
    };
    match rich_text::with_mut(h, |st| {
        if let Some(info) = st.images.get_mut(&name) {
            info.width = spr_w as f32;
            info.height = spr_h.max(1.0) as f32;
            info.xoffset = xoff as f32;
            info.yoffset = yoff as f32;
        }
        st.built = None;
    }) {
        Some(()) => 0.0,
        None => -1.0,
    }
}

pub fn rusty_sdf_rich_clear_images(handle: f64) -> f64 {
    match rich_text::with_mut(handle as Handle, |st| {
        if !st.images.is_empty() {
            st.images.clear();
            st.built = None;
        }
    }) {
        Some(()) => 0.0,
        None => -1.0,
    }
}

pub fn rusty_sdf_rich_build(handle: f64) -> f64 {
    match rich_text::build(handle as Handle) {
        Some((code, _)) => code as f64,
        None => {
            set_last_error("invalid rich text handle");
            -1.0
        }
    }
}

pub fn rusty_sdf_rich_get_metrics_buffer(
    handle: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    let vals = match rich_text::with_ref(handle as Handle, |st| {
        let b = st.built.as_ref();
        [
            b.map(|x| x.total_w as f64).unwrap_or(0.0),
            b.map(|x| x.total_h as f64).unwrap_or(0.0),
            b.map(|x| x.page_buffers.len() as f64).unwrap_or(0.0),
            b.map(|x| x.images.len() as f64).unwrap_or(0.0),
            b.map(|x| x.glyph_meta.len() as f64).unwrap_or(0.0),
            b.map(|x| x.atlas_version as f64)
                .unwrap_or(atlas::get_version() as f64),
            b.map(|x| if x.pending { 1.0 } else { 0.0 }).unwrap_or(0.0),
            b.map(|x| x.last_line_w as f64).unwrap_or(0.0),
            b.map(|x| x.visual_line_count as f64).unwrap_or(0.0),
            b.map(|x| x.vertex_stride as f64)
                .unwrap_or(RICH_VERTEX_STRIDE as f64),
            // native build_rich timings (µs): total, parse, shape, ensure, wrap, layout, verts, refresh
            b.map(|x| x.timing.total as f64).unwrap_or(0.0),
            b.map(|x| x.timing.parse as f64).unwrap_or(0.0),
            b.map(|x| x.timing.shape as f64).unwrap_or(0.0),
            b.map(|x| x.timing.ensure as f64).unwrap_or(0.0),
            b.map(|x| x.timing.wrap as f64).unwrap_or(0.0),
            b.map(|x| x.timing.layout as f64).unwrap_or(0.0),
            b.map(|x| x.timing.verts as f64).unwrap_or(0.0),
            b.map(|x| x.timing.refresh as f64).unwrap_or(0.0),
        ]
    }) {
        Some(v) => v,
        None => {
            set_last_error("invalid rich text handle");
            return -1.0;
        }
    };
    write_f64_array_to_ptr(&vals, buffer_ptr, buffer_len)
}

pub fn rusty_sdf_rich_get_page_byte_size(handle: f64, page: f64) -> f64 {
    match rich_text::with_ref(handle as Handle, |st| {
        st.built
            .as_ref()
            .and_then(|b| b.page_buffers.get(page as usize))
            .map(|p| p.len() as f64)
            .unwrap_or(0.0)
    }) {
        Some(v) => v,
        None => -1.0,
    }
}

pub fn rusty_sdf_rich_write_page_vertices(
    handle: f64,
    page: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    if buffer_ptr.is_null() {
        set_last_error("null buffer pointer");
        return -1.0;
    }
    let (page_data, stride) = match rich_text::with_ref(handle as Handle, |st| {
        st.built.as_ref().map(|b| {
            (
                b.page_buffers
                    .get(page as usize)
                    .cloned()
                    .unwrap_or_default(),
                b.vertex_stride.max(1),
            )
        })
    }) {
        Some(Some(v)) => v,
        Some(None) => return 0.0,
        None => {
            set_last_error("invalid rich text handle");
            return -1.0;
        }
    };
    let len = buffer_len as usize;
    if page_data.len() > len {
        set_last_error("buffer too small for page vertices");
        return -2.0;
    }
    if page_data.is_empty() {
        return 0.0;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(page_data.as_ptr(), as_u8_mut(buffer_ptr), page_data.len());
    }
    (page_data.len() / stride) as f64
}

pub fn rusty_sdf_rich_get_images_buffer(
    handle: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    // Per image: x, y, scale, sub, tint, c, a, b, oc, oa, ow, gc, ga, gr, ul, st (16 f64)
    let values = match rich_text::with_ref(handle as Handle, |st| {
        let mut v = Vec::new();
        if let Some(b) = &st.built {
            for img in &b.images {
                v.push(img.x as f64);
                v.push(img.y as f64);
                v.push(img.scale as f64);
                v.push(img.sub as f64);
                v.push(img.tint as f64);
                v.push(img.style.c as f64);
                v.push(img.style.a as f64);
                v.push(img.style.b as f64);
                v.push(img.style.oc as f64);
                v.push(img.style.oa as f64);
                v.push(img.style.ow as f64);
                v.push(img.style.gc as f64);
                v.push(img.style.ga as f64);
                v.push(img.style.gr as f64);
                v.push(if img.style.ul { 1.0 } else { 0.0 });
                v.push(if img.style.st { 1.0 } else { 0.0 });
            }
        }
        v
    }) {
        Some(v) => v,
        None => return -1.0,
    };
    write_f64_array_to_ptr(&values, buffer_ptr, buffer_len)
}

pub fn rusty_sdf_rich_get_image_name(handle: f64, index: f64) -> String {
    rich_text::with_ref(handle as Handle, |st| {
        st.built
            .as_ref()
            .and_then(|b| b.images.get(index as usize))
            .map(|i| i.name.clone())
            .unwrap_or_default()
    })
    .unwrap_or_default()
}

pub fn rusty_sdf_rich_get_glyph_meta_buffer(
    handle: f64,
    buffer_ptr: *mut c_char,
    buffer_len: f64,
) -> f64 {
    // Per glyph: is_img, page, offset, img_index, is_sdf, c, a, b, oc, oa, ow, gc, ga, gr, ul, st (16)
    let values = match rich_text::with_ref(handle as Handle, |st| {
        let mut v = Vec::new();
        if let Some(b) = &st.built {
            for m in &b.glyph_meta {
                v.push(if m.is_img { 1.0 } else { 0.0 });
                v.push(m.page as f64);
                v.push(m.offset as f64);
                v.push(m.img_index as f64);
                v.push(m.is_sdf as f64);
                v.push(m.style.c as f64);
                v.push(m.style.a as f64);
                v.push(m.style.b as f64);
                v.push(m.style.oc as f64);
                v.push(m.style.oa as f64);
                v.push(m.style.ow as f64);
                v.push(m.style.gc as f64);
                v.push(m.style.ga as f64);
                v.push(m.style.gr as f64);
                v.push(if m.style.ul { 1.0 } else { 0.0 });
                v.push(if m.style.st { 1.0 } else { 0.0 });
            }
        }
        v
    }) {
        Some(v) => v,
        None => return -1.0,
    };
    write_f64_array_to_ptr(&values, buffer_ptr, buffer_len)
}

pub fn rusty_sdf_rich_get_plain_text(handle: f64) -> String {
    rich_text::with_ref(handle as Handle, |st| {
        st.built
            .as_ref()
            .map(|b| b.plain_text.clone())
            .unwrap_or_default()
    })
    .unwrap_or_default()
}
