package ${YYAndroidPackageName};
import static com.gamemaker.ExtensionCore.ExtBridge.RustySDFBridge.*;
import java.lang.String;
import java.nio.ByteBuffer;

public class RustySDFInternal extends RunnerSocial {
    public double __EXT_NATIVE__rusty_sdf_load_font(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_load_font(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_free_font(double font_handle)
    {
        return __EXT_JNI__rusty_sdf_free_font(font_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_add_fallback(double font_handle, double fallback_handle)
    {
        return __EXT_JNI__rusty_sdf_add_fallback(font_handle, fallback_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_get_font_glyph_count(double font_handle)
    {
        return __EXT_JNI__rusty_sdf_get_font_glyph_count(font_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_shape_text(double font_handle, String text, double font_size)
    {
        return __EXT_JNI__rusty_sdf_shape_text(font_handle, text, font_size);
    }
    public double __EXT_NATIVE__rusty_sdf_free_shape(double shape_handle)
    {
        return __EXT_JNI__rusty_sdf_free_shape(shape_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_get_shape_width(double shape_handle)
    {
        return __EXT_JNI__rusty_sdf_get_shape_width(shape_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_get_shape_height(double shape_handle)
    {
        return __EXT_JNI__rusty_sdf_get_shape_height(shape_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_get_shape_glyph_count(double shape_handle)
    {
        return __EXT_JNI__rusty_sdf_get_shape_glyph_count(shape_handle);
    }
    public String __EXT_NATIVE__rusty_sdf_get_shape_glyph_info(double shape_handle, double index)
    {
        return __EXT_JNI__rusty_sdf_get_shape_glyph_info(shape_handle, index);
    }
    public String __EXT_NATIVE__rusty_sdf_get_shape_glyphs_json(double shape_handle)
    {
        return __EXT_JNI__rusty_sdf_get_shape_glyphs_json(shape_handle);
    }
    public double __EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer(double shape_handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_get_shape_glyphs_buffer(shape_handle, buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_set_bidi_mode(double mode)
    {
        return __EXT_JNI__rusty_sdf_set_bidi_mode(mode);
    }
    public double __EXT_NATIVE__rusty_sdf_set_buffer(ByteBuffer buffer_ptr, double buf_w, double buf_h)
    {
        return __EXT_JNI__rusty_sdf_set_buffer(buffer_ptr, buf_w, buf_h);
    }
    public double __EXT_NATIVE__rusty_sdf_set_params(double padding, double spread)
    {
        return __EXT_JNI__rusty_sdf_set_params(padding, spread);
    }
    public double __EXT_NATIVE__rusty_sdf_set_mode(double mode)
    {
        return __EXT_JNI__rusty_sdf_set_mode(mode);
    }
    public double __EXT_NATIVE__rusty_sdf_get_mode()
    {
        return __EXT_JNI__rusty_sdf_get_mode();
    }
    public double __EXT_NATIVE__rusty_sdf_get_buffer_bpp()
    {
        return __EXT_JNI__rusty_sdf_get_buffer_bpp();
    }
    public String __EXT_NATIVE__rusty_sdf_get_glyph_bounds(double font_handle, double glyph_id, double font_size)
    {
        return __EXT_JNI__rusty_sdf_get_glyph_bounds(font_handle, glyph_id, font_size);
    }
    public double __EXT_NATIVE__rusty_sdf_get_glyph_bounds_buffer(double font_handle, double glyph_id, double font_size, ByteBuffer buffer_ptr)
    {
        return __EXT_JNI__rusty_sdf_get_glyph_bounds_buffer(font_handle, glyph_id, font_size, buffer_ptr);
    }
    public double __EXT_NATIVE__rusty_sdf_render_glyph(double font_handle, double glyph_id, double font_size)
    {
        return __EXT_JNI__rusty_sdf_render_glyph(font_handle, glyph_id, font_size);
    }
    public double __EXT_NATIVE__rusty_sdf_render_char(double font_handle, double char_code, double font_size)
    {
        return __EXT_JNI__rusty_sdf_render_char(font_handle, char_code, font_size);
    }
    public double __EXT_NATIVE__rusty_sdf_request_glyph(double font_handle, double glyph_id, double font_size, double padding, double spread, double mode)
    {
        return __EXT_JNI__rusty_sdf_request_glyph(font_handle, glyph_id, font_size, padding, spread, mode);
    }
    public String __EXT_NATIVE__rusty_sdf_poll_glyph()
    {
        return __EXT_JNI__rusty_sdf_poll_glyph();
    }
    public double __EXT_NATIVE__rusty_sdf_poll_glyph_buffer(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_poll_glyph_buffer(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_poll_glyph_pixels(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_poll_glyph_pixels(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided(ByteBuffer buffer_ptr, double buffer_len, double stride_w, double stride_h)
    {
        return __EXT_JNI__rusty_sdf_poll_glyph_pixels_strided(buffer_ptr, buffer_len, stride_w, stride_h);
    }
    public String __EXT_NATIVE__rusty_sdf_measure_text(double font_handle, String text, double font_size)
    {
        return __EXT_JNI__rusty_sdf_measure_text(font_handle, text, font_size);
    }
    public String __EXT_NATIVE__rusty_sdf_ping()
    {
        return __EXT_JNI__rusty_sdf_ping();
    }
    public String __EXT_NATIVE__rusty_sdf_get_last_error()
    {
        return __EXT_JNI__rusty_sdf_get_last_error();
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_init(double width, double height, double padding)
    {
        return __EXT_JNI__rusty_sdf_atlas_init(width, height, padding);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_reset()
    {
        return __EXT_JNI__rusty_sdf_atlas_reset();
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_clear()
    {
        return __EXT_JNI__rusty_sdf_atlas_clear();
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_get_version()
    {
        return __EXT_JNI__rusty_sdf_atlas_get_version();
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_page_count()
    {
        return __EXT_JNI__rusty_sdf_atlas_page_count();
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_ensure_glyph(double font_handle, double glyph_id, double base_size, double spread, double mode, double async_flag)
    {
        return __EXT_JNI__rusty_sdf_atlas_ensure_glyph(font_handle, glyph_id, base_size, spread, mode, async_flag);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_prepare_lookup(double font_handle, double glyph_id, double base_size, double spread)
    {
        return __EXT_JNI__rusty_sdf_atlas_prepare_lookup(font_handle, glyph_id, base_size, spread);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_lookup_buffer(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_atlas_lookup_buffer(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_commit_glyph(double font_handle, double glyph_id, double base_size, double spread, double width, double height, double raw_w, double raw_h, double x_min, double y_max)
    {
        return __EXT_JNI__rusty_sdf_atlas_commit_glyph(font_handle, glyph_id, base_size, spread, width, height, raw_w, raw_h, x_min, y_max);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_atlas_poll_dirty_meta(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels(ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_atlas_poll_dirty_pixels(buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_create()
    {
        return __EXT_JNI__rusty_sdf_rich_create();
    }
    public double __EXT_NATIVE__rusty_sdf_rich_free(double handle)
    {
        return __EXT_JNI__rusty_sdf_rich_free(handle);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_text(double handle, String text)
    {
        return __EXT_JNI__rusty_sdf_rich_set_text(handle, text);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_font(double handle, double font_handle, double font_size, double base_size, double spread)
    {
        return __EXT_JNI__rusty_sdf_rich_set_font(handle, font_handle, font_size, base_size, spread);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_layout(double handle, double max_width, double line_height, double letter_spacing, double halign, double valign)
    {
        return __EXT_JNI__rusty_sdf_rich_set_layout(handle, max_width, line_height, letter_spacing, halign, valign);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_default_style(double handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_set_default_style(handle, buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_async(double handle, double enabled)
    {
        return __EXT_JNI__rusty_sdf_rich_set_async(handle, enabled);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_plain(double handle, double enabled)
    {
        return __EXT_JNI__rusty_sdf_rich_set_plain(handle, enabled);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_config(double handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_set_config(handle, buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_register_image(double handle, String name)
    {
        return __EXT_JNI__rusty_sdf_rich_register_image(handle, name);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_set_image_metrics(double handle, double spr_w, double spr_h, double xoff, double yoff)
    {
        return __EXT_JNI__rusty_sdf_rich_set_image_metrics(handle, spr_w, spr_h, xoff, yoff);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_clear_images(double handle)
    {
        return __EXT_JNI__rusty_sdf_rich_clear_images(handle);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_build(double handle)
    {
        return __EXT_JNI__rusty_sdf_rich_build(handle);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_get_metrics_buffer(handle, buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_get_page_byte_size(double handle, double page)
    {
        return __EXT_JNI__rusty_sdf_rich_get_page_byte_size(handle, page);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_write_page_vertices(double handle, double page, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_write_page_vertices(handle, page, buffer_ptr, buffer_len);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_get_images_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_get_images_buffer(handle, buffer_ptr, buffer_len);
    }
    public String __EXT_NATIVE__rusty_sdf_rich_get_image_name(double handle, double index)
    {
        return __EXT_JNI__rusty_sdf_rich_get_image_name(handle, index);
    }
    public double __EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer(double handle, ByteBuffer buffer_ptr, double buffer_len)
    {
        return __EXT_JNI__rusty_sdf_rich_get_glyph_meta_buffer(handle, buffer_ptr, buffer_len);
    }
    public String __EXT_NATIVE__rusty_sdf_rich_get_plain_text(double handle)
    {
        return __EXT_JNI__rusty_sdf_rich_get_plain_text(handle);
    }
}