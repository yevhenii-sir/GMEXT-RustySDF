// ##### extgen :: Auto-generated Android JNI bridge (Rust) #####
#![allow(non_snake_case)]

use jni::objects::{JByteBuffer, JClass, JObject, JString, JValue};
use jni::sys::{jdouble, jint, jstring, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM, NativeMethod};
use std::ffi::c_void;
use std::os::raw::c_char;

use crate::generated::ffi;

fn direct_buf_ptr(env: &mut JNIEnv<'_>, buf: JObject<'_>) -> Option<*mut c_char> {
    let bb = unsafe { JByteBuffer::from_raw(buf.as_raw()) };
    env.get_direct_buffer_address(&bb).ok().map(|p| p as *mut c_char)
}

extern "system" fn jni_wrap_queue_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble,
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    unsafe { ffi::__EXT_NATIVE__RustySDF_queue_buffer(__arg_buffer_ptr, __arg_buffer_length) }
}

extern "system" fn jni_wrap_rusty_sdf_load_font(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_load_font(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_free_font(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_free_font(font_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_add_fallback(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    fallback_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_add_fallback(font_handle, fallback_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_font_glyph_count(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_font_glyph_count(font_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_shape_text(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    text: JString<'_>,
    font_size: jdouble
) -> jdouble {
    let text_c = env.get_string(&text).ok();
    let text_ptr = text_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_shape_text(font_handle, text_ptr, font_size) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_free_shape(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    shape_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_free_shape(shape_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_shape_width(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    shape_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_shape_width(shape_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_shape_height(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    shape_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_shape_height(shape_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_shape_glyph_count(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    shape_handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_shape_glyph_count(shape_handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_shape_glyphs_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    shape_handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer(shape_handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_set_bidi_mode(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    mode: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_set_bidi_mode(mode) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_set_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    buffer_ptr: JObject<'_>,
    buf_w: jdouble,
    buf_h: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_set_buffer(buffer_ptr_ptr, buf_w, buf_h) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_set_params(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    padding: jdouble,
    spread: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_set_params(padding, spread) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_set_mode(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    mode: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_set_mode(mode) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_mode(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_mode() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_buffer_bpp(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_buffer_bpp() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_get_glyph_bounds(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_glyph_bounds(__arg_buffer_ptr, __arg_buffer_length, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_render_glyph(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    glyph_id: jdouble,
    font_size: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_render_glyph(font_handle, glyph_id, font_size) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_render_char(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    char_code: jdouble,
    font_size: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_render_char(font_handle, char_code, font_size) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_request_glyph(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    glyph_id: jdouble,
    font_size: jdouble,
    padding: jdouble,
    spread: jdouble,
    mode: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_request_glyph(font_handle, glyph_id, font_size, padding, spread, mode) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_poll_glyph(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_poll_glyph(__ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_poll_glyph_pixels(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_poll_glyph_pixels(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_poll_glyph_pixels_strided(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_measure_text(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_measure_text(__arg_buffer_ptr, __arg_buffer_length, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_ping(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_ping() };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

extern "system" fn jni_wrap_rusty_sdf_get_last_error(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_get_last_error() };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

extern "system" fn jni_wrap_rusty_sdf_atlas_init(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    width: jdouble,
    height: jdouble,
    padding: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_init(width, height, padding) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_reset(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_reset() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_clear(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_clear() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_get_version(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_get_version() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_page_count(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_page_count() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_ensure_glyph(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    glyph_id: jdouble,
    base_size: jdouble,
    spread: jdouble,
    mode: jdouble,
    async_flag: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_ensure_glyph(font_handle, glyph_id, base_size, spread, mode, async_flag) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_lookup(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_lookup(__arg_buffer_ptr, __arg_buffer_length, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_commit_glyph(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    font_handle: jdouble,
    glyph_id: jdouble,
    base_size: jdouble,
    spread: jdouble,
    width: jdouble,
    height: jdouble,
    raw_w: jdouble,
    raw_h: jdouble,
    x_min: jdouble,
    y_max: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_commit_glyph(font_handle, glyph_id, base_size, spread, width, height, raw_w, raw_h, x_min, y_max) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_poll_dirty_meta(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_atlas_poll_dirty_pixels(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_create(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_create() };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_free(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_free(handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_text(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    text: JString<'_>
) -> jdouble {
    let text_c = env.get_string(&text).ok();
    let text_ptr = text_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_text(handle, text_ptr) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_font(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    font_handle: jdouble,
    font_size: jdouble,
    base_size: jdouble,
    spread: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_font(handle, font_handle, font_size, base_size, spread) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_layout(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    max_width: jdouble,
    line_height: jdouble,
    letter_spacing: jdouble,
    halign: jdouble,
    valign: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_layout(handle, max_width, line_height, letter_spacing, halign, valign) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_default_style(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_default_style(handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_async(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    enabled: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_async(handle, enabled) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_plain(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    enabled: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_plain(handle, enabled) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_config(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_config(handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_register_image(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    name: JString<'_>
) -> jdouble {
    let name_c = env.get_string(&name).ok();
    let name_ptr = name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_register_image(handle, name_ptr) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_set_image_metrics(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    spr_w: jdouble,
    spr_h: jdouble,
    xoff: jdouble,
    yoff: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_set_image_metrics(handle, spr_w, spr_h, xoff, yoff) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_clear_images(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_clear_images(handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_build(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_build(handle) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_metrics_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer(handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_page_byte_size(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    page: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_page_byte_size(handle, page) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_write_page_vertices(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    page: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_write_page_vertices(handle, page, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_images_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_images_buffer(handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_image_name(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    index: jdouble
) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_image_name(handle, index) };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_glyph_meta_buffer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    buffer_ptr: JObject<'_>,
    buffer_len: jdouble
) -> jdouble {
    let buffer_ptr_ptr = match direct_buf_ptr(&mut env, buffer_ptr) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer(handle, buffer_ptr_ptr, buffer_len) };
    result
}

extern "system" fn jni_wrap_rusty_sdf_rich_get_plain_text(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__rusty_sdf_rich_get_plain_text(handle) };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_gamemaker_ExtensionCore_ExtBridge_RustySDFBridge_nativeRegister(mut env: JNIEnv<'_>, class: JClass<'_>) {
    let methods = [
        NativeMethod { name: "__EXT_JNI__RustySDF_queue_buffer".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_queue_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_load_font".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_load_font as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_free_font".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_free_font as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_add_fallback".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_add_fallback as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_font_glyph_count".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_font_glyph_count as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_shape_text".into(), sig: "(DLjava/lang/String;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_shape_text as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_free_shape".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_free_shape as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_shape_width".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_shape_width as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_shape_height".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_shape_height as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_shape_glyph_count".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_shape_glyph_count as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_shape_glyphs_buffer".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_shape_glyphs_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_set_bidi_mode".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_set_bidi_mode as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_set_buffer".into(), sig: "(Ljava/nio/ByteBuffer;DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_set_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_set_params".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_set_params as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_set_mode".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_set_mode as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_mode".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_get_mode as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_buffer_bpp".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_get_buffer_bpp as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_glyph_bounds".into(), sig: "(Ljava/nio/ByteBuffer;DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_get_glyph_bounds as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_render_glyph".into(), sig: "(DDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_render_glyph as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_render_char".into(), sig: "(DDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_render_char as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_request_glyph".into(), sig: "(DDDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_request_glyph as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_poll_glyph".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_poll_glyph as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_poll_glyph_pixels".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_poll_glyph_pixels as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_poll_glyph_pixels_strided".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_poll_glyph_pixels_strided as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_measure_text".into(), sig: "(Ljava/nio/ByteBuffer;DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_measure_text as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_ping".into(), sig: "()Ljava/lang/String;".into(), fn_ptr: jni_wrap_rusty_sdf_ping as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_get_last_error".into(), sig: "()Ljava/lang/String;".into(), fn_ptr: jni_wrap_rusty_sdf_get_last_error as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_init".into(), sig: "(DDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_init as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_reset".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_reset as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_clear".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_clear as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_get_version".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_get_version as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_page_count".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_page_count as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_ensure_glyph".into(), sig: "(DDDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_ensure_glyph as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_lookup".into(), sig: "(Ljava/nio/ByteBuffer;DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_lookup as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_commit_glyph".into(), sig: "(DDDDDDDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_commit_glyph as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_poll_dirty_meta".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_poll_dirty_meta as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_atlas_poll_dirty_pixels".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_atlas_poll_dirty_pixels as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_create".into(), sig: "()D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_create as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_free".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_free as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_text".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_text as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_font".into(), sig: "(DDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_font as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_layout".into(), sig: "(DDDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_layout as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_default_style".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_default_style as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_async".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_async as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_plain".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_plain as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_config".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_config as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_register_image".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_register_image as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_set_image_metrics".into(), sig: "(DDDDD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_set_image_metrics as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_clear_images".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_clear_images as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_build".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_build as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_metrics_buffer".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_metrics_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_page_byte_size".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_page_byte_size as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_write_page_vertices".into(), sig: "(DDLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_write_page_vertices as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_images_buffer".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_images_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_image_name".into(), sig: "(DD)Ljava/lang/String;".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_image_name as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_glyph_meta_buffer".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_glyph_meta_buffer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__rusty_sdf_rich_get_plain_text".into(), sig: "(D)Ljava/lang/String;".into(), fn_ptr: jni_wrap_rusty_sdf_rich_get_plain_text as *mut c_void },
    ];
    let _ = env.register_native_methods(class, &methods);
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _reserved: *mut c_void) -> jint {
    JNI_VERSION_1_6
}

