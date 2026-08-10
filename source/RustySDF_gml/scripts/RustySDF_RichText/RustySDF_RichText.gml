// ============================================================================
// RICH TEXT MODEL — native parse/layout/vertices (Rust), GML owns GPU buffers
// ============================================================================

/// @func RustySDF_RichText(font_handle,[font_size],[base_size], [spread])
function RustySDF_RichText(_font, _font_size = 32, _base_size = global.rusty_sdf_default_base_size, _spread = global.rusty_sdf_default_spread) constructor {
    font_handle = _font;
    base_size = _base_size;
    spread = _spread;

    raw_text = "";
    plain_text = "";
    font_size = _font_size;
    alpha = 1.0;

    max_width = 0;
    fit_width = -1;
    fit_height = -1;
    halign = fa_left;
    valign = fa_top;
    line_height = 0;
    letter_spacing = 0;

    default_style = {
        c: c_white, a: 1.0, b: 0.360,
        oc: c_black, oa: 1.0, ow: 0.0,
        gc: c_aqua, ga: 0.0, gr: 0.20,
        ul: false, st: false
    };

    vbuffers = [];
    shadow_buffers = [];
    shadow_sizes = [];
    image_draw_list = [];
    glyph_metadata = [];
    _glyph_meta_count = 0;
    _glyph_meta_ready = false;

    is_dirty = true;
    is_style_dirty = false;
    atlas_version = -1;
    runs_shapes = [];

    total_text_width = 0;
    total_text_height = 0;

    native_handle = rusty_sdf_rich_create();
    _io_buf = buffer_create(4096, buffer_grow, 1);

    static set_font = function(f) { if (font_handle != f) { font_handle = f; is_dirty = true; } return self; }
    static set_text = function(t) { if (raw_text != t) { raw_text = t; is_dirty = true; } return self; }
    static set_font_size = function(sz) { if (font_size != sz) { font_size = sz; is_dirty = true; } return self; }
    static set_wrap = function(w) { if (max_width != w) { max_width = w; is_dirty = true; } return self; }
    static set_fit_size = function(fw, fh) { fit_width = fw; fit_height = fh; return self; }
    static set_align = function(ha, va) { if (halign != ha || valign != va) { halign = ha; valign = va; is_dirty = true; } return self; }
    static set_spacing = function(lh, ls) { if (line_height != lh || letter_spacing != ls) { line_height = lh; letter_spacing = ls; is_dirty = true; } return self; }
    static set_base_size = function(sz) { if (base_size != sz) { base_size = sz; is_dirty = true; } return self; }
    static set_spread = function(sp) { if (spread != sp) { spread = sp; is_dirty = true; } return self; }
    static set_alpha = function(a) { alpha = a; return self; }

    static _styles_equal = function(s1, s2) {
        return (s1.c == s2.c && s1.a == s2.a && s1.b == s2.b &&
                s1.oc == s2.oc && s1.oa == s2.oa && s1.ow == s2.ow &&
                s1.gc == s2.gc && s1.ga == s2.ga && s1.gr == s2.gr &&
                s1.ul == s2.ul && s1.st == s2.st);
    }

    static set_default_style = function(c, a, b, oc, oa, ow, gc, ga, gr, ul = false, st = false) {
        var geo_changed = (default_style.ul != ul) || (default_style.st != st);
        var visual_changed = (default_style.c != c) || (default_style.a != a) || (default_style.b != b) ||
                             (default_style.oc != oc) || (default_style.oa != oa) || (default_style.ow != ow) ||
                             (default_style.gc != gc) || (default_style.ga != ga) || (default_style.gr != gr);

        if (!geo_changed && !visual_changed) return self;

        var old_style = _copy_style(default_style);

        default_style.c = c; default_style.a = a; default_style.b = b;
        default_style.oc = oc; default_style.oa = oa; default_style.ow = ow;
        default_style.gc = gc; default_style.ga = ga; default_style.gr = gr;
        default_style.ul = ul; default_style.st = st;

        if (geo_changed) {
            is_dirty = true;
        }
        else if (visual_changed && !is_dirty) {
            _ensure_glyph_meta();
            for (var i = 0; i < array_length(glyph_metadata); i++) {
                if (variable_struct_exists(glyph_metadata[i], "base_style")) {
                    if (_styles_equal(glyph_metadata[i].base_style, old_style)) {
                        set_glyph_style(i, default_style);
                    }
                }
            }
        }
        return self;
    }

    static set_base_color = function(_color) {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        _ensure_glyph_meta();
        for (var i = 0; i < array_length(glyph_metadata); i++) {
            var meta = glyph_metadata[i];
            if (meta.base_style.c != _color) {
                meta.base_style.c = _color;
                if (meta.is_img) {
                    image_draw_list[meta.img_index].style.c = _color;
                } else {
                    var page = meta.page;
                    var offset = meta.offset;
                    if (page < 0 || offset < 0) continue;
                    var sb = shadow_buffers[page];
                    var cr = color_get_red(_color) / 255;
                    var cg = color_get_green(_color) / 255;
                    var cb = color_get_blue(_color) / 255;
                    for (var v = 0; v < 6; v++) {
                        buffer_seek(sb, buffer_seek_start, offset + (v * 80) + 16);
                        buffer_write(sb, buffer_f32, cr);
                        buffer_write(sb, buffer_f32, cg);
                        buffer_write(sb, buffer_f32, cb);
                    }
                }
            }
        }
        default_style.c = _color;
        is_style_dirty = true;
        return self;
    }

    static free = function() {
        for (var i = 0; i < array_length(vbuffers); i++) {
            if (vbuffers[i] != undefined && vertex_buffer_exists(vbuffers[i])) vertex_delete_buffer(vbuffers[i]);
        }
        for (var i = 0; i < array_length(shadow_buffers); i++) {
            if (shadow_buffers[i] != undefined && buffer_exists(shadow_buffers[i])) buffer_delete(shadow_buffers[i]);
        }
        vbuffers = [];
        shadow_buffers = [];
        shadow_sizes = [];
        glyph_metadata = [];
        _glyph_meta_count = 0;
        _glyph_meta_ready = false;
        runs_shapes = [];
    }

    /// Lazy: glyph meta is only needed for per-glyph style APIs, not for draw.
    static _ensure_glyph_meta = function() {
        if (_glyph_meta_ready) return;
        if (native_handle <= 0) {
            glyph_metadata = [];
            _glyph_meta_ready = true;
            return;
        }

        glyph_metadata = [];
        var count = _glyph_meta_count;
        if (count <= 0) {
            _glyph_meta_ready = true;
            return;
        }

        var meta_need = max(256, count * 16 * 8 + 16);
        if (buffer_get_size(_io_buf) < meta_need) buffer_resize(_io_buf, meta_need);
        var written = rusty_sdf_rich_get_glyph_meta_buffer(native_handle, buffer_get_address(_io_buf), buffer_get_size(_io_buf));
        if (written > 0) {
            buffer_seek(_io_buf, buffer_seek_start, 0);
            var mflat = __ext_core_buffer_unmarshal_value(_io_buf, []);
            if (is_array(mflat)) {
                for (var gi = 0; gi < count; gi++) {
                    var mo = gi * 16;
                    if (mo + 15 >= array_length(mflat)) break;
                    array_push(glyph_metadata, {
                        is_img: mflat[mo] > 0,
                        page: mflat[mo + 1],
                        offset: mflat[mo + 2],
                        img_index: mflat[mo + 3],
                        is_sdf: mflat[mo + 4],
                        base_style: {
                            c: mflat[mo + 5], a: mflat[mo + 6], b: mflat[mo + 7],
                            oc: mflat[mo + 8], oa: mflat[mo + 9], ow: mflat[mo + 10],
                            gc: mflat[mo + 11], ga: mflat[mo + 12], gr: mflat[mo + 13],
                            ul: mflat[mo + 14] > 0, st: mflat[mo + 15] > 0
                        }
                    });
                }
            }
        }
        _glyph_meta_ready = true;
    }

    static destroy = function() {
        free();
        if (native_handle >= 0) {
            rusty_sdf_rich_free(native_handle);
            native_handle = -1;
        }
        if (buffer_exists(_io_buf)) buffer_delete(_io_buf);
    }

    static _copy_style = function(s) {
        return { c: s.c, a: s.a, b: s.b, oc: s.oc, oa: s.oa, ow: s.ow, gc: s.gc, ga: s.ga, gr: s.gr, ul: s.ul, st: s.st };
    }

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

    static _register_images_from_text = function() {
        rusty_sdf_rich_clear_images(native_handle);
        var text = raw_text;
        var len = string_length(text);
        var i = 1;
        while (i <= len) {
            if (string_char_at(text, i) == "[") {
                var close_idx = string_pos_ext("]", text, i);
                if (close_idx > 0) {
                    var tag_full = string_copy(text, i + 1, close_idx - i - 1);
                    if (string_copy(tag_full, 1, 4) == "img=") {
                        var tag_val = string_copy(tag_full, 5, string_length(tag_full) - 4);
                        var args = string_split(tag_val, ",");
                        var spr_name = args[0];
                        var spr_idx = asset_get_index(spr_name);
                        if (sprite_exists(spr_idx)) {
                            rusty_sdf_rich_register_image(native_handle, spr_name);
                            rusty_sdf_rich_set_image_metrics(
                                native_handle,
                                sprite_get_width(spr_idx),
                                sprite_get_height(spr_idx),
                                sprite_get_xoffset(spr_idx),
                                sprite_get_yoffset(spr_idx)
                            );
                        }
                    }
                    i = close_idx + 1;
                    continue;
                }
            }
            i++;
        }
    }

    static _push_config_buffer = function(_plain) {
        var s = default_style;
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
        buffer_write(_io_buf, buffer_f64, _plain ? 1 : 0);
        buffer_write(_io_buf, buffer_f64, global.rusty_sdf_async_enabled ? 1 : 0);
        buffer_write(_io_buf, buffer_f64, s.c);
        buffer_write(_io_buf, buffer_f64, s.a);
        buffer_write(_io_buf, buffer_f64, s.b);
        buffer_write(_io_buf, buffer_f64, s.oc);
        buffer_write(_io_buf, buffer_f64, s.oa);
        buffer_write(_io_buf, buffer_f64, s.ow);
        buffer_write(_io_buf, buffer_f64, s.gc);
        buffer_write(_io_buf, buffer_f64, s.ga);
        buffer_write(_io_buf, buffer_f64, s.gr);
        buffer_write(_io_buf, buffer_f64, s.ul ? 1 : 0);
        buffer_write(_io_buf, buffer_f64, s.st ? 1 : 0);
        rusty_sdf_rich_set_config(native_handle, buffer_get_address(_io_buf), need);
    }

    static build = function() {
        if (font_handle < 0 || raw_text == "" || native_handle <= 0) return;
        var atlas_only = !is_dirty && atlas_version >= 0 && atlas_version != global.rusty_sdf_atlas.version;

        free();

        image_draw_list = [];
        glyph_metadata = [];
        _glyph_meta_count = 0;
        _glyph_meta_ready = false;
        shadow_sizes = [];
        plain_text = "";

        if (!atlas_only) {
            rusty_sdf_rich_set_text(native_handle, raw_text);
            _push_config_buffer(0);
            _register_images_from_text();
        }

        rusty_sdf_rich_build(native_handle);

        RustySDF_AtlasFlushDirty();
        __RustySDF_AtlasSyncPageSurfaces();

        // metrics: total_w, total_h, page_count, image_count, glyph_meta_count, atlas_version, pending
        if (buffer_get_size(_io_buf) < 256) buffer_resize(_io_buf, 256);
        var written = rusty_sdf_rich_get_metrics_buffer(native_handle, buffer_get_address(_io_buf), buffer_get_size(_io_buf));
        if (written <= 0) return;
        buffer_seek(_io_buf, buffer_seek_start, 0);
        var metrics = __ext_core_buffer_unmarshal_value(_io_buf, []);
        if (!is_array(metrics) || array_length(metrics) < 6) return;

        total_text_width = metrics[0];
        total_text_height = metrics[1];
        var page_count = metrics[2];
        var image_count = metrics[3];
        _glyph_meta_count = metrics[4];
        atlas_version = metrics[5];
        global.rusty_sdf_atlas.version = rusty_sdf_atlas_get_version();

        // pages → shadow + vertex buffers
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
                array_push(vbuffers, vertex_create_buffer_from_buffer_ext(sb, global.rusty_sdf_vformat_rich, 0, vcount));
            } else {
                array_push(vbuffers, undefined);
            }
        }

        // images
        var img_need = max(256, image_count * 16 * 8 + 16);
        if (buffer_get_size(_io_buf) < img_need) buffer_resize(_io_buf, img_need);
        written = rusty_sdf_rich_get_images_buffer(native_handle, buffer_get_address(_io_buf), buffer_get_size(_io_buf));
        if (written > 0 && image_count > 0) {
            buffer_seek(_io_buf, buffer_seek_start, 0);
            var flat = __ext_core_buffer_unmarshal_value(_io_buf, []);
            if (is_array(flat)) {
                for (var ii = 0; ii < image_count; ii++) {
                    var o = ii * 16;
                    if (o + 15 >= array_length(flat)) break;
                    var name = rusty_sdf_rich_get_image_name(native_handle, ii);
                    var spr = asset_get_index(name);
                    array_push(image_draw_list, {
                        spr: spr,
                        sub: flat[o + 3],
                        x: flat[o],
                        y: flat[o + 1],
                        scale: flat[o + 2],
                        tint: flat[o + 4],
                        style: {
                            c: flat[o + 5], a: flat[o + 6], b: flat[o + 7],
                            oc: flat[o + 8], oa: flat[o + 9], ow: flat[o + 10],
                            gc: flat[o + 11], ga: flat[o + 12], gr: flat[o + 13],
                            ul: flat[o + 14] > 0, st: flat[o + 15] > 0
                        }
                    });
                }
            }
        }

        plain_text = rusty_sdf_rich_get_plain_text(native_handle);

        atlas_version = global.rusty_sdf_atlas.version;
        is_dirty = false;
        is_style_dirty = false;
    }

    static get_glyph_count = function() {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        _ensure_glyph_meta();
        return array_length(glyph_metadata);
    }

    static get_plain_text = function() {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        return plain_text;
    }

    static get_glyph_base_style = function(_index) {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        _ensure_glyph_meta();
        if (_index < 0 || _index >= array_length(glyph_metadata)) return undefined;
        return _copy_style(glyph_metadata[_index].base_style);
    }

    static set_glyph_style = function(_index, _style) {
        if (is_dirty || atlas_version != global.rusty_sdf_atlas.version) build();
        _ensure_glyph_meta();
        if (_index < 0 || _index >= array_length(glyph_metadata)) return;

        var meta = glyph_metadata[_index];
        meta.base_style = _copy_style(_style);

        if (meta.is_img) {
            image_draw_list[meta.img_index].style = _copy_style(_style);
            return;
        }

        var page = meta.page;
        var offset = meta.offset;
        if (page < 0 || offset < 0) return;

        var sb = shadow_buffers[page];

        var cr = color_get_red(_style.c)/255, cg = color_get_green(_style.c)/255, cb = color_get_blue(_style.c)/255, ca = _style.a;
        var out_r = color_get_red(_style.oc)/255, out_g = color_get_green(_style.oc)/255, out_b = color_get_blue(_style.oc)/255, out_a = _style.oa;
        var gr = color_get_red(_style.gc)/255, gg = color_get_green(_style.gc)/255, gb = color_get_blue(_style.gc)/255, ga = _style.ga;
        var b = _style.b, ow = _style.ow, rad = _style.gr, is_sdf = meta.is_sdf;

        for (var v = 0; v < 6; v++) {
            buffer_seek(sb, buffer_seek_start, offset + (v * 80) + 16);
            buffer_write(sb, buffer_f32, cr);      buffer_write(sb, buffer_f32, cg);
            buffer_write(sb, buffer_f32, cb);      buffer_write(sb, buffer_f32, ca);
            buffer_write(sb, buffer_f32, out_r);   buffer_write(sb, buffer_f32, out_g);
            buffer_write(sb, buffer_f32, out_b);   buffer_write(sb, buffer_f32, out_a);
            buffer_write(sb, buffer_f32, gr);      buffer_write(sb, buffer_f32, gg);
            buffer_write(sb, buffer_f32, gb);      buffer_write(sb, buffer_f32, ga);
            buffer_write(sb, buffer_f32, b);       buffer_write(sb, buffer_f32, ow);
            buffer_write(sb, buffer_f32, rad);     buffer_write(sb, buffer_f32, is_sdf);
        }
        is_style_dirty = true;
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

        if (is_style_dirty) {
            for (var i = 0; i < array_length(shadow_buffers); i++) {
                if (shadow_buffers[i] != undefined && vbuffers[i] != undefined) {
                    var size = shadow_sizes[i];
                    if (size > 0) vertex_update_buffer_from_buffer(vbuffers[i], 0, shadow_buffers[i], 0, size);
                }
            }
            for (var i = 0; i < array_length(shadow_buffers); i++) {
                if (shadow_buffers[i] != undefined) buffer_seek(shadow_buffers[i], buffer_seek_start, shadow_sizes[i]);
            }
            is_style_dirty = false;
        }

        var fit_scale = get_fit_scale();
        var final_xscale = fit_scale * xscale;
        var final_yscale = fit_scale * yscale;

        var _cos = dcos(rot);
        var _sin = dsin(rot);

        if (array_length(vbuffers) > 0) {
            var is_msdf = (global.rusty_sdf_mode >= 2);
            var sh = is_msdf ? shd_msdf_rich : shd_sdf_rich;
            var u = is_msdf ? global.rusty_sdf_rich_uniforms.msdf : global.rusty_sdf_rich_uniforms.sdf;

            shader_set(sh);
            shader_set_uniform_f(u.alpha, alpha);
            shader_set_uniform_f(u.transform, final_xscale, final_yscale, x, y);
            shader_set_uniform_f(u.rotation, _cos, _sin);

            for (var pass = 0; pass <= 1; pass++) {
                shader_set_uniform_i(u.pass, pass);
                for (var i = 0; i < array_length(vbuffers); i++) {
                    if (vbuffers[i] != undefined) {
                        var surf = global.rusty_sdf_atlas.pages[i].surface;
                        if (surface_exists(surf)) vertex_submit(vbuffers[i], pr_trianglelist, surface_get_texture(surf));
                    }
                }
            }
            shader_reset();
        }

        for (var i = 0; i < array_length(image_draw_list); i++) {
            var img = image_draw_list[i];
            if (!sprite_exists(img.spr)) continue;

            var lx = img.x * final_xscale;
            var ly = img.y * final_yscale;
            var final_img_x = x + (lx * _cos + ly * _sin);
            var final_img_y = y + (-lx * _sin + ly * _cos);
            var img_blend = (variable_struct_exists(img, "tint") && img.tint > 0) ? img.style.c : c_white;

            draw_sprite_ext(
                img.spr, img.sub,
                final_img_x, final_img_y,
                img.scale * final_xscale, img.scale * final_yscale,
                rot, img_blend, alpha * img.style.a
            );
        }
    }
}
