/// RustySDF_Wrapper
/// High-level GML wrapper for the RustySDF Rust extension.

if (!variable_global_exists("rusty_sdf_font_buffers")) {
    global.rusty_sdf_font_buffers = ds_map_create();
}
if (!variable_global_exists("rusty_sdf_fonts_by_path")) {
    global.rusty_sdf_fonts_by_path = ds_map_create();
}
if (!variable_global_exists("rusty_sdf_path_by_handle")) {
    global.rusty_sdf_path_by_handle = ds_map_create();
}
if (!variable_global_exists("rusty_sdf_mode")) {
    global.rusty_sdf_mode = 0;
}
if (!variable_global_exists("rusty_sdf_active_shader")) {
    global.rusty_sdf_active_shader = shd_sdf;
}
if (!variable_global_exists("rusty_sdf_vformat")) {
    global.rusty_sdf_vformat = -1;
}

enum ERustySDFMode {
    SDF = 0,
    PSDF = 1,
    MSDF = 2,
    MTSDF = 3
}

enum ERustySDFBidi {
    Auto = 0,
    ForceLTR = 1,
    ForceRTL = 2
}

function RustySDF_Init() {
    var version = rusty_sdf_ping();
    show_debug_message("RustySDF: Extension loaded, version=" + version);

    // Cache shader uniforms for both SDF and MSDF shaders
    var _cache_shd = function(_shd) {
        return {
            boldness: shader_get_uniform(_shd, "u_boldness"),
            outline_width: shader_get_uniform(_shd, "u_outline_width"),
            glow_radius: shader_get_uniform(_shd, "u_glow_radius"),
            core_color: shader_get_uniform(_shd, "u_core_color"),
            outline_color: shader_get_uniform(_shd, "u_outline_color"),
            glow_color: shader_get_uniform(_shd, "u_glow_color"),
            enable_outline: shader_get_uniform(_shd, "u_enable_outline"),
            enable_glow: shader_get_uniform(_shd, "u_enable_glow"),
            transform: shader_get_uniform(_shd, "u_transform"),
            rotation: shader_get_uniform(_shd, "u_rotation")
        };
    };

    global.rusty_sdf_uniforms = {
        sdf: _cache_shd(shd_sdf),
        msdf: _cache_shd(shd_msdf)
    };

    // Initialize global vertex format for ultra-fast text rendering
    vertex_format_begin();
    vertex_format_add_position();
    vertex_format_add_color();
    vertex_format_add_texcoord();
    global.rusty_sdf_vformat = vertex_format_end();

	vertex_format_begin();
    vertex_format_add_position(); // x, y
    vertex_format_add_texcoord(); // u, v
    vertex_format_add_custom(vertex_type_float4, vertex_usage_texcoord); // Core
    vertex_format_add_custom(vertex_type_float4, vertex_usage_texcoord); // Outline
    vertex_format_add_custom(vertex_type_float4, vertex_usage_texcoord); // Glow
    vertex_format_add_custom(vertex_type_float4, vertex_usage_texcoord); // Params
    global.rusty_sdf_vformat_rich = vertex_format_end();

	global.rusty_sdf_rich_uniforms = {
        sdf: {
            pass: shader_get_uniform(shd_sdf_rich, "u_render_pass"),
            alpha: shader_get_uniform(shd_sdf_rich, "u_global_alpha"),
            transform: shader_get_uniform(shd_sdf_rich, "u_transform"),
            rotation: shader_get_uniform(shd_sdf_rich, "u_rotation")
        },
        msdf: {
            pass: shader_get_uniform(shd_msdf_rich, "u_render_pass"),
            alpha: shader_get_uniform(shd_msdf_rich, "u_global_alpha"),
            transform: shader_get_uniform(shd_msdf_rich, "u_transform"),
            rotation: shader_get_uniform(shd_msdf_rich, "u_rotation")
        }
    };

	if (!variable_global_exists("rusty_sdf_poll_buf")) {
        global.rusty_sdf_poll_buf = buffer_create(128, buffer_fixed, 1);
    }
    if (!variable_global_exists("rusty_sdf_pixel_buf")) {
        global.rusty_sdf_pixel_buf = buffer_create(1024 * 1024, buffer_grow, 1);
    }
}

/// @func RustySDF_GetFont(font_path)
function RustySDF_GetFont(font_path, fallback_handle_id = -1) {
    if (ds_map_exists(global.rusty_sdf_fonts_by_path, font_path)) {
        return ds_map_find_value(global.rusty_sdf_fonts_by_path, font_path);
    }

    var buf = buffer_load(font_path);
    if (buf < 0) {
        show_debug_message("RustySDF: Failed to load font file: " + font_path);
        return -1;
    }

    var bufferPtr = buffer_get_address(buf);
    var handle = rusty_sdf_load_font(bufferPtr, buffer_get_size(buf));

    if (handle < 0) {
        show_debug_message("RustySDF: Failed to load font: " + rusty_sdf_get_last_error());
        buffer_delete(buf);
        return -1;
    }

	if (fallback_handle_id != -1) {
		RustySDF_AddFallback(handle, fallback_handle_id);
	}

    ds_map_add(global.rusty_sdf_font_buffers, handle, buf);
    ds_map_add(global.rusty_sdf_fonts_by_path, font_path, handle);
    ds_map_add(global.rusty_sdf_path_by_handle, handle, font_path);

    show_debug_message("RustySDF: Font Loaded -> " + font_path + " (Handle: " + string(handle) + ")");
    return handle;
}

function RustySDF_FreeFont(font_handle) {
    var result = rusty_sdf_free_font(font_handle);

    if (ds_map_exists(global.rusty_sdf_font_buffers, font_handle)) {
        var buf = ds_map_find_value(global.rusty_sdf_font_buffers, font_handle);
        buffer_delete(buf);
        ds_map_delete(global.rusty_sdf_font_buffers, font_handle);
    }

    if (ds_map_exists(global.rusty_sdf_path_by_handle, font_handle)) {
        var path = ds_map_find_value(global.rusty_sdf_path_by_handle, font_handle);
        ds_map_delete(global.rusty_sdf_fonts_by_path, path);
        ds_map_delete(global.rusty_sdf_path_by_handle, font_handle);
    }

    return result;
}

function RustySDF_AddFallback(font_handle, fallback_handle) {
    return rusty_sdf_add_fallback(font_handle, fallback_handle);
}

function RustySDF_SetBidiMode(mode) {
    return rusty_sdf_set_bidi_mode(mode);
}

function RustySDF_GetFontGlyphCount(font_handle) {
    return rusty_sdf_get_font_glyph_count(font_handle);
}

function RustySDF_ShapeText(font_handle, text, font_size) {
    var shape_handle = rusty_sdf_shape_text(font_handle, text, font_size);
    if (shape_handle < 0) {
        show_debug_message("[RustySDF] ShapeText FAILED: " + rusty_sdf_get_last_error());
        return undefined;
    }

    var width = rusty_sdf_get_shape_width(shape_handle);
    var height = rusty_sdf_get_shape_height(shape_handle);
    var glyph_count = rusty_sdf_get_shape_glyph_count(shape_handle);

    // Use binary buffer path for zero-allocation glyph transfer
    var glyphs = [];
    var buf_size = max(glyph_count * 64 + 16, 256);
	static _buf = buffer_create(1024, buffer_grow, 1);
    if (buffer_get_size(_buf) < buf_size) {
        buffer_resize(_buf, buf_size);
    }

    var written = rusty_sdf_get_shape_glyphs_buffer(shape_handle, buffer_get_address(_buf), buf_size);

    if (written > 0) {
        buffer_seek(_buf, buffer_seek_start, 0);
        var flat = __ext_core_buffer_unmarshal_value(_buf, []);
        if (is_array(flat)) {
            var n = array_length(flat) / 8;
            glyphs = array_create(n);
            for (var i = 0; i < n; i++) {
                var base = i * 8;
                glyphs[i] = {
                    font_handle: flat[base + 0],
                    glyph_id: flat[base + 1],
                    x_offset: flat[base + 2],
                    y_offset: flat[base + 3],
                    x_advance: flat[base + 4],
                    y_advance: flat[base + 5],
                    cluster: flat[base + 6],
                    char_code: flat[base + 7]
                };
            }
        } else {
            show_debug_message("[RustySDF] ShapeText: unmarshal returned non-array");
        }
    } else {
        show_debug_message("[RustySDF] ShapeText: get_shape_glyphs_buffer failed, err=" + rusty_sdf_get_last_error());
    }

    if (!is_array(glyphs)) glyphs = [];

    return {
        handle: shape_handle, width: width, height: height,
        glyph_count: glyph_count, glyphs: glyphs
    };
}

function RustySDF_FreeShape(shape_struct) {
    if (!is_struct(shape_struct) || !variable_struct_exists(shape_struct, "handle")) return -1;
    return rusty_sdf_free_shape(shape_struct.handle);
}

function RustySDF_SetMode(mode) {
    var result = rusty_sdf_set_mode(mode);
    if (result < 0) return -1;
    global.rusty_sdf_mode = mode;
    global.rusty_sdf_active_shader = (mode >= 2) ? shd_msdf : shd_sdf;
    return 0;
}

function RustySDF_GetMode() { return rusty_sdf_get_mode(); }
function RustySDF_GetBufferBPP() { return 4; }

function RustySDF_GetGlyphBounds(font_handle, glyph_id, font_size) {
    static _buf = buffer_create(64, buffer_fixed, 1);
    var written = rusty_sdf_get_glyph_bounds_buffer(font_handle, glyph_id, font_size, buffer_get_address(_buf));
    if (written <= 0) return undefined;

    buffer_seek(_buf, buffer_seek_start, 0);
    var flat = __ext_core_buffer_unmarshal_value(_buf, []);

    if (!is_array(flat) || array_length(flat) < 4) {
        return undefined;
    }
    return { width: flat[0], height: flat[1], x_min: flat[2], y_max: flat[3] };
}

function RustySDF_SetBuffer(surface) {
    if (!surface_exists(surface)) return -1;
    var w = surface_get_width(surface), h = surface_get_height(surface);
    var buf = buffer_create(w * h * 4, buffer_fixed, 1);

    if (!variable_global_exists("rusty_sdf_temp_buffer")) global.rusty_sdf_temp_buffer = -1;
    if (global.rusty_sdf_temp_buffer >= 0) buffer_delete(global.rusty_sdf_temp_buffer);
    global.rusty_sdf_temp_buffer = buf;

    return rusty_sdf_set_buffer(buffer_get_address(buf), w, h);
}

function RustySDF_SetParams(padding, spread) { return rusty_sdf_set_params(padding, spread); }
function RustySDF_RenderGlyph(font_handle, glyph_id, font_size) { return rusty_sdf_render_glyph(font_handle, glyph_id, font_size); }
function RustySDF_RenderChar(font_handle, char_code, font_size) { return rusty_sdf_render_char(font_handle, char_code, font_size); }

function RustySDF_MeasureText(font_handle, text, font_size) {
    var json_str = rusty_sdf_measure_text(font_handle, text, font_size);
    if (!is_string(json_str) || json_str == "") return undefined;
    return json_parse(json_str);
}

function RustySDF_GetLastError() { return rusty_sdf_get_last_error(); }
function RustySDF_Ping() { return rusty_sdf_ping(); }

// ─── Async Glyph Generation ─────────────────────────────────────────────────

function RustySDF_RequestGlyph(font_handle, glyph_id, font_size, padding, spread, mode) {
    return rusty_sdf_request_glyph(font_handle, glyph_id, font_size, padding, spread, mode) > 0;
}

function RustySDF_PollGlyph() {
    var buf = global.rusty_sdf_poll_buf;

    var written = rusty_sdf_poll_glyph_buffer(buffer_get_address(buf), 128);
    if (written <= 0) return undefined;

    buffer_seek(buf, buffer_seek_start, 0);
    var flat = __ext_core_buffer_unmarshal_value(buf, []);

    if (!is_array(flat) || array_length(flat) < 11) return undefined;
    return {
        font_handle: flat[0], glyph_id: flat[1], font_size: flat[2],
        padding: flat[3], spread: flat[4],
        width: flat[5], height: flat[6],
        raw_w: flat[7], raw_h: flat[8],
        x_min: flat[9], y_max: flat[10]
    };
}
function RustySDF_PollGlyphPixels(buffer) {
    if (!buffer_exists(buffer)) return -1;
    return rusty_sdf_poll_glyph_pixels(buffer_get_address(buffer), buffer_get_size(buffer));
}

/// @func RustySDF_PollGlyphPixelsStrided(buffer, stride_w, stride_h)
/// @desc Writes last polled glyph into buffer with row stride (for fixed upload scratch).
function RustySDF_PollGlyphPixelsStrided(buffer, stride_w, stride_h) {
    if (!buffer_exists(buffer)) return -1;
    return rusty_sdf_poll_glyph_pixels_strided(
        buffer_get_address(buffer),
        buffer_get_size(buffer),
        stride_w,
        stride_h
    );
}

function RustySDF_RenderGlyphSDF(font_handle, glyph_id, font_size, target_surface, padding, spread) {
    if (!surface_exists(target_surface)) return -1;

    var target_w = surface_get_width(target_surface);
    var target_h = surface_get_height(target_surface);
    var align_w = target_w + ((4 - (target_w mod 4)) mod 4);
    var align_h = target_h + ((4 - (target_h mod 4)) mod 4);
	var req_size = align_w * align_h * 4;

    static _buf = buffer_create(1024 * 64, buffer_grow, 1);
    if (buffer_get_size(_buf) < req_size) {
        buffer_resize(_buf, req_size);
    }

    rusty_sdf_set_buffer(buffer_get_address(_buf), align_w, align_h);
    rusty_sdf_set_params(padding, spread);

    if (rusty_sdf_render_glyph(font_handle, glyph_id, font_size) < 0) {
        return -1;
    }

    if (align_w == target_w && align_h == target_h) {
        buffer_set_surface(_buf, target_surface, 0);
    } else {
        var upload = __RustySDF_UploadPackedGlyphToScratch(_buf, align_w, align_h);
        if (is_undefined(upload)) return -1;

        surface_set_target(target_surface);
        draw_clear_alpha(c_black, 0);
        draw_surface_part(upload.surf, 0, 0, target_w, target_h, 0, 0);
        surface_reset_target();
    }

    return 0;
}
