package com.gamemaker.ExtensionCore.ExtBridge;
import java.lang.String;
import java.nio.ByteBuffer;
import ${YYAndroidPackageName}.GMExtUtils;

public final class RustySDFBridge {
    static {
        // this is the extension lib name
        System.loadLibrary("RustySDF");
        nativeRegister();
    }
    // this registers the native functions on the C++ layer
    private static native void nativeRegister();

    public static String __EXT_JAVA__GetExtensionOption(String extName, String optName)
    {
        return GMExtUtils.GetExtensionOption(extName, optName);
    }

    public static native double __EXT_JNI__RustySDF_queue_buffer(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_load_font(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_free_font(double font_handle);
    public static native double __EXT_JNI__rusty_sdf_add_fallback(double font_handle, double fallback_handle);
    public static native double __EXT_JNI__rusty_sdf_get_font_glyph_count(double font_handle);
    public static native double __EXT_JNI__rusty_sdf_shape_text(double font_handle, String text, double font_size);
    public static native double __EXT_JNI__rusty_sdf_free_shape(double shape_handle);
    public static native double __EXT_JNI__rusty_sdf_get_shape_width(double shape_handle);
    public static native double __EXT_JNI__rusty_sdf_get_shape_height(double shape_handle);
    public static native double __EXT_JNI__rusty_sdf_get_shape_glyph_count(double shape_handle);
    public static native double __EXT_JNI__rusty_sdf_get_shape_glyphs_buffer(double shape_handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native double __EXT_JNI__rusty_sdf_set_bidi_mode(double mode);
    public static native double __EXT_JNI__rusty_sdf_set_buffer(ByteBuffer buffer_ptr, double buf_w, double buf_h);
    public static native double __EXT_JNI__rusty_sdf_set_params(double padding, double spread);
    public static native double __EXT_JNI__rusty_sdf_set_mode(double mode);
    public static native double __EXT_JNI__rusty_sdf_get_mode();
    public static native double __EXT_JNI__rusty_sdf_get_buffer_bpp();
    public static native double __EXT_JNI__rusty_sdf_get_glyph_bounds(ByteBuffer __arg_buffer, double __arg_buffer_length, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_render_glyph(double font_handle, double glyph_id, double font_size);
    public static native double __EXT_JNI__rusty_sdf_render_char(double font_handle, double char_code, double font_size);
    public static native double __EXT_JNI__rusty_sdf_request_glyph(double font_handle, double glyph_id, double font_size, double padding, double spread, double mode);
    public static native double __EXT_JNI__rusty_sdf_poll_glyph(ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_poll_glyph_pixels(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_poll_glyph_pixels_strided(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_measure_text(ByteBuffer __arg_buffer, double __arg_buffer_length, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native String __EXT_JNI__rusty_sdf_ping();
    public static native String __EXT_JNI__rusty_sdf_get_last_error();
    public static native double __EXT_JNI__rusty_sdf_atlas_init(double width, double height, double padding);
    public static native double __EXT_JNI__rusty_sdf_atlas_reset();
    public static native double __EXT_JNI__rusty_sdf_atlas_clear();
    public static native double __EXT_JNI__rusty_sdf_atlas_get_version();
    public static native double __EXT_JNI__rusty_sdf_atlas_page_count();
    public static native double __EXT_JNI__rusty_sdf_atlas_ensure_glyph(double font_handle, double glyph_id, double base_size, double spread, double mode, double async_flag);
    public static native double __EXT_JNI__rusty_sdf_atlas_lookup(ByteBuffer __arg_buffer, double __arg_buffer_length, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_atlas_commit_glyph(double font_handle, double glyph_id, double base_size, double spread, double width, double height, double raw_w, double raw_h, double x_min, double y_max);
    public static native double __EXT_JNI__rusty_sdf_atlas_poll_dirty_meta(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_atlas_poll_dirty_pixels(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__rusty_sdf_rich_create();
    public static native double __EXT_JNI__rusty_sdf_rich_free(double handle);
    public static native double __EXT_JNI__rusty_sdf_rich_set_text(double handle, String text);
    public static native double __EXT_JNI__rusty_sdf_rich_set_font(double handle, double font_handle, double font_size, double base_size, double spread);
    public static native double __EXT_JNI__rusty_sdf_rich_set_layout(double handle, double max_width, double line_height, double letter_spacing, double halign, double valign);
    public static native double __EXT_JNI__rusty_sdf_rich_set_default_style(double handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native double __EXT_JNI__rusty_sdf_rich_set_async(double handle, double enabled);
    public static native double __EXT_JNI__rusty_sdf_rich_set_plain(double handle, double enabled);
    public static native double __EXT_JNI__rusty_sdf_rich_set_config(double handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native double __EXT_JNI__rusty_sdf_rich_register_image(double handle, String name);
    public static native double __EXT_JNI__rusty_sdf_rich_set_image_metrics(double handle, double spr_w, double spr_h, double xoff, double yoff);
    public static native double __EXT_JNI__rusty_sdf_rich_clear_images(double handle);
    public static native double __EXT_JNI__rusty_sdf_rich_build(double handle);
    public static native double __EXT_JNI__rusty_sdf_rich_get_metrics_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native double __EXT_JNI__rusty_sdf_rich_get_page_byte_size(double handle, double page);
    public static native double __EXT_JNI__rusty_sdf_rich_write_page_vertices(double handle, double page, ByteBuffer buffer_ptr, double buffer_len);
    public static native double __EXT_JNI__rusty_sdf_rich_get_images_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native String __EXT_JNI__rusty_sdf_rich_get_image_name(double handle, double index);
    public static native double __EXT_JNI__rusty_sdf_rich_get_glyph_meta_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len);
    public static native String __EXT_JNI__rusty_sdf_rich_get_plain_text(double handle);
}