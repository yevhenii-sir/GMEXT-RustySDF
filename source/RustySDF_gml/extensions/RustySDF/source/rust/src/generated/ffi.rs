// ##### extgen :: Auto-generated file do not edit!! #####

#![allow(non_upper_case_globals)]

use std::ffi::c_char;
use std::panic::catch_unwind;
use gm_ext_wire::{clear_last_error, get_last_error_ptr, set_last_error};
use gm_ext_wire::store_tls_string;
use gm_ext_wire::{GMBufferReader, GMSliceWriter, BufferQueue, GMBuffer};
use crate::user;

static __buffer_queue: BufferQueue = BufferQueue::new();

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__RustySDF_queue_buffer(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __buff = GMBuffer::new(__arg_buffer as *mut u8, __arg_buffer_length as u64);
        __buffer_queue.push(__buff);
        1.0
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__RustySDF_queue_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__RustySDF_get_last_error() -> *const c_char {
    get_last_error_ptr()
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_load_font(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let data = __buffer_queue.pop_front()?;
            Some(user::rusty_sdf_load_font(data) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_load_font");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_free_font(font_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_free_font(font_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_free_font");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_add_fallback(font_handle: f64, fallback_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_add_fallback(font_handle, fallback_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_add_fallback");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_font_glyph_count(font_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_font_glyph_count(font_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_font_glyph_count");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_shape_text(font_handle: f64, text: *const c_char, font_size: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let text_str = if text.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(text) }.to_str().unwrap_or("") };
        user::rusty_sdf_shape_text(font_handle, text_str, font_size)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_shape_text");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_free_shape(shape_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_free_shape(shape_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_free_shape");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_shape_width(shape_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_shape_width(shape_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_shape_width");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_shape_height(shape_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_shape_height(shape_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_shape_height");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_shape_glyph_count(shape_handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_shape_glyph_count(shape_handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_shape_glyph_count");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer(shape_handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_shape_glyphs_buffer(shape_handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_set_bidi_mode(mode: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_set_bidi_mode(mode)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_set_bidi_mode");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_set_buffer(buffer_ptr: *mut c_char, buf_w: f64, buf_h: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_set_buffer(buffer_ptr, buf_w, buf_h)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_set_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_set_params(padding: f64, spread: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_set_params(padding, spread)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_set_params");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_set_mode(mode: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_set_mode(mode)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_set_mode");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_mode() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_mode()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_mode");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_buffer_bpp() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_get_buffer_bpp()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_buffer_bpp");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_glyph_bounds(__arg_buffer: *mut c_char, __arg_buffer_length: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let font_handle = __br.read_f64()?;
            let glyph_id = __br.read_f64()?;
            let font_size = __br.read_f64()?;
            let __result = user::rusty_sdf_get_glyph_bounds(font_handle, glyph_id, font_size);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_glyph_bounds");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_render_glyph(font_handle: f64, glyph_id: f64, font_size: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_render_glyph(font_handle, glyph_id, font_size)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_render_glyph");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_render_char(font_handle: f64, char_code: f64, font_size: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_render_char(font_handle, char_code, font_size)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_render_char");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_request_glyph(font_handle: f64, glyph_id: f64, font_size: f64, padding: f64, spread: f64, mode: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_request_glyph(font_handle, glyph_id, font_size, padding, spread, mode)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_request_glyph");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_poll_glyph(__ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let __result = user::rusty_sdf_poll_glyph();
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_poll_glyph");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_poll_glyph_pixels(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let dst = __buffer_queue.pop_front()?;
            Some(user::rusty_sdf_poll_glyph_pixels(dst) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_poll_glyph_pixels");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let dst = __buffer_queue.pop_front()?;
            let stride_w = __br.read_f64()?;
            let stride_h = __br.read_f64()?;
            Some(user::rusty_sdf_poll_glyph_pixels_strided(dst, stride_w, stride_h) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_measure_text(__arg_buffer: *mut c_char, __arg_buffer_length: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let font_handle = __br.read_f64()?;
            let text = __br.read_idl_string()?.to_string();
            let font_size = __br.read_f64()?;
            let __result = user::rusty_sdf_measure_text(font_handle, text, font_size);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_measure_text");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_ping() -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::rusty_sdf_ping();
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_ping");
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_get_last_error() -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::rusty_sdf_get_last_error();
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_get_last_error");
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_init(width: f64, height: f64, padding: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_init(width, height, padding)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_init");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_reset() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_reset()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_reset");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_clear() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_clear()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_clear");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_get_version() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_get_version()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_get_version");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_page_count() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_page_count()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_page_count");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_ensure_glyph(font_handle: f64, glyph_id: f64, base_size: f64, spread: f64, mode: f64, async_flag: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_ensure_glyph(font_handle, glyph_id, base_size, spread, mode, async_flag)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_ensure_glyph");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_lookup(__arg_buffer: *mut c_char, __arg_buffer_length: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let font_handle = __br.read_f64()?;
            let glyph_id = __br.read_f64()?;
            let base_size = __br.read_f64()?;
            let spread = __br.read_f64()?;
            let __result = user::rusty_sdf_atlas_lookup(font_handle, glyph_id, base_size, spread);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_lookup");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_commit_glyph(font_handle: f64, glyph_id: f64, base_size: f64, spread: f64, width: f64, height: f64, raw_w: f64, raw_h: f64, x_min: f64, y_max: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_atlas_commit_glyph(font_handle, glyph_id, base_size, spread, width, height, raw_w, raw_h, x_min, y_max)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_commit_glyph");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let dst = __buffer_queue.pop_front()?;
            Some(user::rusty_sdf_atlas_poll_dirty_meta(dst) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let dst = __buffer_queue.pop_front()?;
            Some(user::rusty_sdf_atlas_poll_dirty_pixels(dst) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_create() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_create()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_create");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_free(handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_free(handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_free");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_text(handle: f64, text: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let text_str = if text.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(text) }.to_str().unwrap_or("") };
        user::rusty_sdf_rich_set_text(handle, text_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_text");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_font(handle: f64, font_handle: f64, font_size: f64, base_size: f64, spread: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_font(handle, font_handle, font_size, base_size, spread)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_font");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_layout(handle: f64, max_width: f64, line_height: f64, letter_spacing: f64, halign: f64, valign: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_layout(handle, max_width, line_height, letter_spacing, halign, valign)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_layout");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_default_style(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_default_style(handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_default_style");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_async(handle: f64, enabled: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_async(handle, enabled)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_async");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_plain(handle: f64, enabled: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_plain(handle, enabled)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_plain");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_config(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_config(handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_config");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_register_image(handle: f64, name: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let name_str = if name.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(name) }.to_str().unwrap_or("") };
        user::rusty_sdf_rich_register_image(handle, name_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_register_image");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_set_image_metrics(handle: f64, spr_w: f64, spr_h: f64, xoff: f64, yoff: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_set_image_metrics(handle, spr_w, spr_h, xoff, yoff)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_set_image_metrics");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_clear_images(handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_clear_images(handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_clear_images");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_build(handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_build(handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_build");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_get_metrics_buffer(handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_page_byte_size(handle: f64, page: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_get_page_byte_size(handle, page)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_page_byte_size");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_write_page_vertices(handle: f64, page: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_write_page_vertices(handle, page, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_write_page_vertices");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_images_buffer(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_get_images_buffer(handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_images_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_image_name(handle: f64, index: f64) -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::rusty_sdf_rich_get_image_name(handle, index);
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_image_name");
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer(handle: f64, buffer_ptr: *mut c_char, buffer_len: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::rusty_sdf_rich_get_glyph_meta_buffer(handle, buffer_ptr, buffer_len)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__rusty_sdf_rich_get_plain_text(handle: f64) -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::rusty_sdf_rich_get_plain_text(handle);
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__rusty_sdf_rich_get_plain_text");
            std::ptr::null()
        }
    }
}

