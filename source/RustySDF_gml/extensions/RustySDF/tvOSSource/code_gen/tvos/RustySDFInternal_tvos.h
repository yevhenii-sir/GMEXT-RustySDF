// ##### extgen :: Auto-generated file do not edit!! #####

#import <Foundation/Foundation.h>

@interface RustySDFInternal : NSObject
- (double)__EXT_NATIVE__rusty_sdf_load_font:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_free_font:(double)font_handle;
- (double)__EXT_NATIVE__rusty_sdf_add_fallback:(double)font_handle arg1:(double)fallback_handle;
- (double)__EXT_NATIVE__rusty_sdf_get_font_glyph_count:(double)font_handle;
- (double)__EXT_NATIVE__rusty_sdf_shape_text:(double)font_handle arg1:(char*)text arg2:(double)font_size;
- (double)__EXT_NATIVE__rusty_sdf_free_shape:(double)shape_handle;
- (double)__EXT_NATIVE__rusty_sdf_get_shape_width:(double)shape_handle;
- (double)__EXT_NATIVE__rusty_sdf_get_shape_height:(double)shape_handle;
- (double)__EXT_NATIVE__rusty_sdf_get_shape_glyph_count:(double)shape_handle;
- (double)__EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer:(double)shape_handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (double)__EXT_NATIVE__rusty_sdf_set_bidi_mode:(double)mode;
- (double)__EXT_NATIVE__rusty_sdf_set_buffer:(char*)buffer_ptr arg1:(double)buf_w arg2:(double)buf_h;
- (double)__EXT_NATIVE__rusty_sdf_set_params:(double)padding arg1:(double)spread;
- (double)__EXT_NATIVE__rusty_sdf_set_mode:(double)mode;
- (double)__EXT_NATIVE__rusty_sdf_get_mode;
- (double)__EXT_NATIVE__rusty_sdf_get_buffer_bpp;
- (double)__EXT_NATIVE__rusty_sdf_get_glyph_bounds:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_render_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)font_size;
- (double)__EXT_NATIVE__rusty_sdf_render_char:(double)font_handle arg1:(double)char_code arg2:(double)font_size;
- (double)__EXT_NATIVE__rusty_sdf_request_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)font_size arg3:(double)padding arg4:(double)spread arg5:(double)mode;
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph:(char*)__ret_buffer arg1:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph_pixels:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_measure_text:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length;
- (char*)__EXT_NATIVE__rusty_sdf_ping;
- (char*)__EXT_NATIVE__rusty_sdf_get_last_error;
- (double)__EXT_NATIVE__rusty_sdf_atlas_init:(double)width arg1:(double)height arg2:(double)padding;
- (double)__EXT_NATIVE__rusty_sdf_atlas_reset;
- (double)__EXT_NATIVE__rusty_sdf_atlas_clear;
- (double)__EXT_NATIVE__rusty_sdf_atlas_get_version;
- (double)__EXT_NATIVE__rusty_sdf_atlas_page_count;
- (double)__EXT_NATIVE__rusty_sdf_atlas_ensure_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)base_size arg3:(double)spread arg4:(double)mode arg5:(double)async_flag;
- (double)__EXT_NATIVE__rusty_sdf_atlas_lookup:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_atlas_commit_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)base_size arg3:(double)spread arg4:(double)width arg5:(double)height arg6:(double)raw_w arg7:(double)raw_h arg8:(double)x_min arg9:(double)y_max;
- (double)__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__rusty_sdf_rich_create;
- (double)__EXT_NATIVE__rusty_sdf_rich_free:(double)handle;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_text:(double)handle arg1:(char*)text;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_font:(double)handle arg1:(double)font_handle arg2:(double)font_size arg3:(double)base_size arg4:(double)spread;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_layout:(double)handle arg1:(double)max_width arg2:(double)line_height arg3:(double)letter_spacing arg4:(double)halign arg5:(double)valign;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_default_style:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_async:(double)handle arg1:(double)enabled;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_plain:(double)handle arg1:(double)enabled;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_config:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (double)__EXT_NATIVE__rusty_sdf_rich_register_image:(double)handle arg1:(char*)name;
- (double)__EXT_NATIVE__rusty_sdf_rich_set_image_metrics:(double)handle arg1:(double)spr_w arg2:(double)spr_h arg3:(double)xoff arg4:(double)yoff;
- (double)__EXT_NATIVE__rusty_sdf_rich_clear_images:(double)handle;
- (double)__EXT_NATIVE__rusty_sdf_rich_build:(double)handle;
- (double)__EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (double)__EXT_NATIVE__rusty_sdf_rich_get_page_byte_size:(double)handle arg1:(double)page;
- (double)__EXT_NATIVE__rusty_sdf_rich_write_page_vertices:(double)handle arg1:(double)page arg2:(char*)buffer_ptr arg3:(double)buffer_len;
- (double)__EXT_NATIVE__rusty_sdf_rich_get_images_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (char*)__EXT_NATIVE__rusty_sdf_rich_get_image_name:(double)handle arg1:(double)index;
- (double)__EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len;
- (char*)__EXT_NATIVE__rusty_sdf_rich_get_plain_text:(double)handle;
- (double)__EXT_NATIVE__RustySDF_queue_buffer:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
@end

