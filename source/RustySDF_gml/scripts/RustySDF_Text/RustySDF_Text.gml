// ============================================================================
// VERTEX-CACHED TEXT MODEL — native layout + plain 20-byte vertices (Rust)
// ============================================================================

/// @func RustySDF_Text(font_handle, [font_size], [base_font_size], [spread])
function RustySDF_Text(_font, _font_size = 32, _base_size = global.rusty_sdf_default_base_size, _spread = global.rusty_sdf_default_spread) constructor {
    font_handle = _font;
    base_size = _base_size;
    spread = _spread;

    text = "";
    font_size = _font_size;
    alpha = 1.0;

    max_width = 0;
    fit_width = -1;
    fit_height = -1;
    halign = fa_left;
    valign = fa_top;
    line_height = 0;
    letter_spacing = 0;

    style = undefined;

    vbuffers = [];
    shadow_buffers = [];
    shadow_sizes = [];
    atlas_version = -1;
    is_dirty = true;
    shapes_array = [];

    total_text_width = 0;
    total_text_height = 0;
    last_line_width = 0;
    visual_line_count = 0;

    native_handle = rusty_sdf_rich_create();
    _io_buf = buffer_create(4096, buffer_grow, 1);
    if (native_handle > 0) {
        rusty_sdf_rich_set_plain(native_handle, 1);
    }

    static set_font = function(f) { if (font_handle != f) { font_handle = f; is_dirty = true; } return self; }
    static set_text = function(t) { if (text != t) { text = t; is_dirty = true; } return self; }
    static set_font_size = function(sz) { if (font_size != sz) { font_size = sz; is_dirty = true; } return self; }
    static set_wrap = function(w) { if (max_width != w) { max_width = w; is_dirty = true; } return self; }
    static set_fit_size = function(fw, fh) { fit_width = fw; fit_height = fh; return self; }
    static set_align = function(ha, va) { if (halign != ha || valign != va) { halign = ha; valign = va; is_dirty = true; } return self; }
    static set_spacing = function(lh, ls) { if (line_height != lh || letter_spacing != ls) { line_height = lh; letter_spacing = ls; is_dirty = true; } return self; }
    static set_base_size = function(sz) { if (base_size != sz) { base_size = sz; is_dirty = true; } return self; }
    static set_spread = function(sp) { if (spread != sp) { spread = sp; is_dirty = true; } return self; }
    static set_alpha = function(a) { alpha = a; return self; }

    static set_style = function(c_col, c_a, o_col, o_a, o_w, g_col, g_a, g_rad, bld) {
        if (is_undefined(style)) style = {};
        style.core_color = c_col; style.core_alpha = c_a;
        style.outline_color = o_col; style.outline_alpha = o_a; style.outline_width = o_w;
        style.glow_color = g_col; style.glow_alpha = g_a; style.glow_radius = g_rad;
        style.boldness = bld; style.use_outline = (o_w > 0); style.use_glow = (g_a > 0);
        return self;
    }

    static set_style_struct = function(_s) { style = _s; return self; }
    static clear_style = function() { style = undefined; return self; }

    static _halign_to_native = function(ha) {
        if (ha == fa_center) return 1;
        if (ha == fa_right) return 2;
        return 0;
    }
    static _valign_to_native = function(va) {
        if (va == fa_middle) return 1;
        if (va == fa_bottom) return 2;
        return 0;
    }

    static free = function() {
        for (var i = 0; i < array_length(vbuffers); i++) {
            if (!is_undefined(vbuffers[i]) && vertex_buffer_exists(vbuffers[i])) vertex_delete_buffer(vbuffers[i]);
        }
        for (var i = 0; i < array_length(shadow_buffers); i++) {
            if (!is_undefined(shadow_buffers[i]) && buffer_exists(shadow_buffers[i])) buffer_delete(shadow_buffers[i]);
        }
        vbuffers = [];
        shadow_buffers = [];
        shadow_sizes = [];
        shapes_array = [];
    }

    static destroy = function() {
        free();
        if (native_handle > 0) {
            rusty_sdf_rich_free(native_handle);
            native_handle = -1;
        }
        if (buffer_exists(_io_buf)) buffer_delete(_io_buf);
    }

    static build = function() {
        if (font_handle < 0 || text == "" || native_handle <= 0) return;
        var atlas_only = !is_dirty && atlas_version >= 0 && atlas_version != global.rusty_sdf_atlas.version;

        free();

        if (!atlas_only) {
            rusty_sdf_rich_set_text(native_handle, text);
            var need = 22 * 8;
            if (buffer_get_size(_io_buf) < need) buffer_resize(_io_buf, need);
            buffer_seek(_io_buf, buffer_seek_start, 0);
            buffer_write(_io_buf, buffer_f64, font_handle);
            buffer_write(_io_buf, buffer_f64, font_size);
            buffer_write(_io_buf, buffer_f64, base_size);
            buffer_write(_io_buf, buffer_f64, spread);
            buffer_write(_io_buf, buffer_f64, max_width);
            buffer_write(_io_buf, buffer_f64, line_height);
            buffer_write(_io_buf, buffer_f64, letter_spacing);
            buffer_write(_io_buf, buffer_f64, _halign_to_native(halign));
            buffer_write(_io_buf, buffer_f64, _valign_to_native(valign));
            buffer_write(_io_buf, buffer_f64, 1); // plain
            buffer_write(_io_buf, buffer_f64, global.rusty_sdf_async_enabled ? 1 : 0);
            // default style (unused in plain verts, but required by config)
            buffer_write(_io_buf, buffer_f64, c_white);
            buffer_write(_io_buf, buffer_f64, 1);
            buffer_write(_io_buf, buffer_f64, 0.36);
            buffer_write(_io_buf, buffer_f64, c_black);
            buffer_write(_io_buf, buffer_f64, 1);
            buffer_write(_io_buf, buffer_f64, 0);
            buffer_write(_io_buf, buffer_f64, c_aqua);
            buffer_write(_io_buf, buffer_f64, 0);
            buffer_write(_io_buf, buffer_f64, 0.2);
            buffer_write(_io_buf, buffer_f64, 0);
            buffer_write(_io_buf, buffer_f64, 0);
            rusty_sdf_rich_set_config(native_handle, buffer_get_address(_io_buf), need);
        }

        rusty_sdf_rich_build(native_handle);

        RustySDF_AtlasFlushDirty();
        __RustySDF_AtlasSyncPageSurfaces();

        if (buffer_get_size(_io_buf) < 256) buffer_resize(_io_buf, 256);
        var written = rusty_sdf_rich_get_metrics_buffer(native_handle, buffer_get_address(_io_buf), buffer_get_size(_io_buf));
        if (written <= 0) return;
        buffer_seek(_io_buf, buffer_seek_start, 0);
        var metrics = __ext_core_buffer_unmarshal_value(_io_buf, []);
        if (!is_array(metrics) || array_length(metrics) < 6) return;

        total_text_width = metrics[0];
        total_text_height = metrics[1];
        var page_count = metrics[2];
        last_line_width = (array_length(metrics) > 7) ? metrics[7] : 0;
        visual_line_count = (array_length(metrics) > 8) ? metrics[8] : 0;
        var stride = (array_length(metrics) > 9) ? metrics[9] : 20;
        if (stride <= 0) stride = 20;

        global.rusty_sdf_atlas.version = rusty_sdf_atlas_get_version();

        for (var page = 0; page < page_count; page++) {
            var nbytes = rusty_sdf_rich_get_page_byte_size(native_handle, page);
            if (nbytes <= 0) {
                array_push(shadow_buffers, undefined);
                array_push(shadow_sizes, 0);
                array_push(vbuffers, undefined);
                continue;
            }
            var sb = buffer_create(nbytes, buffer_fixed, 1);
            var vcount = rusty_sdf_rich_write_page_vertices(native_handle, page, buffer_get_address(sb), nbytes);
            array_push(shadow_buffers, sb);
            array_push(shadow_sizes, nbytes);
            if (vcount > 0) {
                array_push(vbuffers, vertex_create_buffer_from_buffer_ext(sb, global.rusty_sdf_vformat, 0, vcount));
            } else {
                array_push(vbuffers, undefined);
            }
        }

        atlas_version = global.rusty_sdf_atlas.version;
        is_dirty = false;
    }

    static get_fit_scale = function() {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        var fit_scale = 1.0;
        if (fit_width > 0 && total_text_width > fit_width) fit_scale = fit_width / total_text_width;
        if (fit_height > 0 && (total_text_height * fit_scale) > fit_height) fit_scale = fit_height / total_text_height;
        return fit_scale;
    }

    static get_drawn_width = function(xscale = 1) {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        return total_text_width * get_fit_scale() * xscale;
    }

    static get_drawn_height = function(yscale = 1) {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        return total_text_height * get_fit_scale() * yscale;
    }

    static draw = function(x, y, xscale = 1, yscale = 1, rot = 0) {
        RustySDF_AtlasCheckSurfaces();

        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        if (array_length(vbuffers) == 0) return;

        var sh = global.rusty_sdf_active_shader;
        shader_set(sh);
        var s = (style != undefined) ? style : global.rusty_sdf_text_style;
        var u = (sh == shd_msdf) ? global.rusty_sdf_uniforms.msdf : global.rusty_sdf_uniforms.sdf;

        shader_set_uniform_f(u.boldness, s.boldness);
        shader_set_uniform_f(u.outline_width, s.use_outline ? s.outline_width : 0.0);
        shader_set_uniform_f(u.glow_radius, s.use_glow ? s.glow_radius : 0.0);

        var fit_scale = get_fit_scale();
        var final_xscale = fit_scale * xscale;
        var final_yscale = fit_scale * yscale;

        shader_set_uniform_f(u.transform, final_xscale, final_yscale, x, y);
        shader_set_uniform_f(u.rotation, dcos(rot), dsin(rot));

        var c_a = s.core_alpha * alpha;
        var o_a = s.outline_alpha * alpha;
        var g_a = (s.use_glow ? s.glow_alpha : 0.0) * alpha;

        if (s.use_outline || (s.use_glow && s.glow_alpha > 0.0)) {
            shader_set_uniform_i(u.enable_outline, s.use_outline ? 1 : 0);
            shader_set_uniform_i(u.enable_glow, s.use_glow ? 1 : 0);
            shader_set_uniform_f(u.core_color, 0, 0, 0, 0);
            shader_set_uniform_f(u.outline_color, color_get_red(s.outline_color)/255, color_get_green(s.outline_color)/255, color_get_blue(s.outline_color)/255, o_a);
            shader_set_uniform_f(u.glow_color, color_get_red(s.glow_color)/255, color_get_green(s.glow_color)/255, color_get_blue(s.glow_color)/255, g_a);

            for (var i = 0; i < array_length(vbuffers); i++) {
                if (vbuffers[i] != undefined) {
                    var surf = global.rusty_sdf_atlas.pages[i].surface;
                    if (surface_exists(surf)) vertex_submit(vbuffers[i], pr_trianglelist, surface_get_texture(surf));
                }
            }
        }

        shader_set_uniform_i(u.enable_outline, 0);
        shader_set_uniform_i(u.enable_glow, 0);
        shader_set_uniform_f(u.core_color, color_get_red(s.core_color)/255, color_get_green(s.core_color)/255, color_get_blue(s.core_color)/255, c_a);

        for (var i = 0; i < array_length(vbuffers); i++) {
            if (vbuffers[i] != undefined) {
                var surf = global.rusty_sdf_atlas.pages[i].surface;
                if (surface_exists(surf)) vertex_submit(vbuffers[i], pr_trianglelist, surface_get_texture(surf));
            }
        }

        shader_reset();
    }
}
