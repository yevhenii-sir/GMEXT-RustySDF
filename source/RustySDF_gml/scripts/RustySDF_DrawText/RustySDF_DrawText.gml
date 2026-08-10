/// RustySDF_DrawText
/// High-level text drawing with Atlas and Vertex Caching

// ─── Default Global Settings ──────────────────────────────────────────────────

if (!variable_global_exists("rusty_sdf_text_style")) {
    global.rusty_sdf_text_style = {
        core_color: c_white, core_alpha: 1.0,
        outline_color: c_black, outline_alpha: 1.0, outline_width: 0.08,
        glow_color: c_aqua, glow_alpha: 0.0, glow_radius: 0.20,
        boldness: 0.36, use_outline: true, use_glow: false,
        alpha: 1.0
    };
}

if (!variable_global_exists("rusty_sdf_default_base_size")) {
    global.rusty_sdf_default_base_size = 38; // Default base resolution for SDF glyphs
}

if (!variable_global_exists("rusty_sdf_default_spread")) {
    global.rusty_sdf_default_spread = 8;     // Default SDF spread distance
}

function RustySDF_TextStyleReset() {
    var s = global.rusty_sdf_text_style;
    s.core_color = c_white; s.core_alpha = 1.0;
    s.outline_color = c_black; s.outline_alpha = 1.0; s.outline_width = 0.08;
    s.glow_color = c_aqua; s.glow_alpha = 0.0; s.glow_radius = 0.20;
    s.boldness = 0.36; s.use_outline = true; s.use_glow = false;
    s.alpha = 1.0;
}

function RustySDF_TextStyleSet(core_color, core_alpha, outline_color, outline_alpha, outline_width, glow_color, glow_alpha, glow_radius, boldness) {
    var s = global.rusty_sdf_text_style;
    s.core_color = core_color; s.core_alpha = core_alpha;
    s.outline_color = outline_color; s.outline_alpha = outline_alpha; s.outline_width = outline_width;
    s.glow_color = glow_color; s.glow_alpha = glow_alpha; s.glow_radius = glow_radius;
    s.boldness = boldness; s.use_outline = (outline_width > 0); s.use_glow = (glow_alpha > 0);
}

/// @func RustySDF_StyleCreate(core_color, core_alpha, outline_color, outline_alpha, outline_width, glow_color, glow_alpha, glow_radius, boldness)
/// @desc Creates a reusable style struct that can be passed to RustySDF_Text.set_style_struct()
function RustySDF_StyleCreate(core_color, core_alpha, outline_color, outline_alpha, outline_width, glow_color, glow_alpha, glow_radius, boldness) {
    return {
        core_color: core_color, core_alpha: core_alpha,
        outline_color: outline_color, outline_alpha: outline_alpha, outline_width: outline_width,
        glow_color: glow_color, glow_alpha: glow_alpha, glow_radius: glow_radius,
        boldness: boldness, use_outline: (outline_width > 0), use_glow: (glow_alpha > 0),
        alpha: 1.0
    };
}

function __RustySDF_ApplyShader() {
    var sh = global.rusty_sdf_active_shader;
    shader_set(sh);
    var s = global.rusty_sdf_text_style;
    var a = variable_struct_exists(s, "alpha") ? s.alpha : 1.0;

    shader_set_uniform_f(shader_get_uniform(sh, "u_boldness"), s.boldness);
    shader_set_uniform_f(shader_get_uniform(sh, "u_outline_width"), s.use_outline ? s.outline_width : 0.0);
    shader_set_uniform_f(shader_get_uniform(sh, "u_glow_radius"), s.use_glow ? s.glow_radius : 0.0);

    shader_set_uniform_i(shader_get_uniform(sh, "u_enable_outline"), s.use_outline ? 1 : 0);
    shader_set_uniform_i(shader_get_uniform(sh, "u_enable_glow"), s.use_glow ? 1 : 0);

    shader_set_uniform_f(shader_get_uniform(sh, "u_core_color"), color_get_red(s.core_color)/255, color_get_green(s.core_color)/255, color_get_blue(s.core_color)/255, s.core_alpha * a);
    shader_set_uniform_f(shader_get_uniform(sh, "u_outline_color"), color_get_red(s.outline_color)/255, color_get_green(s.outline_color)/255, color_get_blue(s.outline_color)/255, s.outline_alpha * a);
    shader_set_uniform_f(shader_get_uniform(sh, "u_glow_color"), color_get_red(s.glow_color)/255, color_get_green(s.glow_color)/255, color_get_blue(s.glow_color)/255, (s.use_glow ? s.glow_alpha : 0.0) * a);
}

// ============================================================================
// Legacy compatibility functions
// ============================================================================

function RustySDF_DrawText(_x, _y, font_handle, text, font_size, base_font_size = global.rusty_sdf_default_base_size, spread = global.rusty_sdf_default_spread) {
    if (font_handle < 0) return -1;
    var shape = RustySDF_ShapeText(font_handle, text, font_size);
    if (is_undefined(shape)) return -1;
    var glyphs = shape.glyphs;
    var glyph_count = shape.glyph_count;
    if (glyph_count <= 0) { RustySDF_FreeShape(shape); return 0; }

    for (var i = 0; i < glyph_count; i++) RustySDF_AtlasPackGlyph(glyphs[i].font_handle, glyphs[i].glyph_id, base_font_size, spread);

    var sh = global.rusty_sdf_active_shader;
    shader_set(sh);
    
    var s = global.rusty_sdf_text_style;
    var u = (sh == shd_msdf) ? global.rusty_sdf_uniforms.msdf : global.rusty_sdf_uniforms.sdf;
    var a = variable_struct_exists(s, "alpha") ? s.alpha : 1.0;

    shader_set_uniform_f(u.boldness, s.boldness);
    shader_set_uniform_f(u.outline_width, s.use_outline ? s.outline_width : 0.0);
    shader_set_uniform_f(u.glow_radius, s.use_glow ? s.glow_radius : 0.0);

    var scale = font_size / base_font_size;
    
    // ПРОХОД 1: Эффекты
    if (s.use_outline || s.use_glow) {
        shader_set_uniform_i(u.enable_outline, s.use_outline ? 1 : 0);
        shader_set_uniform_i(u.enable_glow, s.use_glow ? 1 : 0);
        
        shader_set_uniform_f(u.core_color, 0, 0, 0, 0);
        shader_set_uniform_f(u.outline_color, color_get_red(s.outline_color)/255, color_get_green(s.outline_color)/255, color_get_blue(s.outline_color)/255, s.outline_alpha * a);
        shader_set_uniform_f(u.glow_color, color_get_red(s.glow_color)/255, color_get_green(s.glow_color)/255, color_get_blue(s.glow_color)/255, (s.use_glow ? s.glow_alpha : 0.0) * a);

        var pen_x = _x, pen_y = _y;
        for (var i = 0; i < glyph_count; i++) {
            var g = glyphs[i];
            var entry = RustySDF_AtlasPackGlyph(g.font_handle, g.glyph_id, base_font_size, spread);
            if (is_undefined(entry) || variable_struct_get(entry, "raw_w") <= 0) { pen_x += g.x_advance; continue; }

            var atlas_pad = max(global.rusty_sdf_atlas.padding, spread + 1);
            var draw_x = pen_x + g.x_offset + (variable_struct_get(entry, "x_min") * scale) - (atlas_pad * scale);
            var draw_y = pen_y - g.y_offset - (variable_struct_get(entry, "y_max") * scale) - (atlas_pad * scale);
        
            draw_surface_part_ext(
                global.rusty_sdf_atlas.pages[variable_struct_get(entry, "page_index")].surface,
                variable_struct_get(entry, "atlas_x"), variable_struct_get(entry, "atlas_y"),
                variable_struct_get(entry, "w"), variable_struct_get(entry, "h"),
                draw_x, draw_y, scale, scale, c_white, 1.0
            );
            pen_x += g.x_advance;
            if (!is_undefined(g.y_advance)) pen_y -= g.y_advance;
        }
    }

    // ПРОХОД 2: Сердцевина текста
    shader_set_uniform_i(u.enable_outline, 0);
    shader_set_uniform_i(u.enable_glow, 0);
    shader_set_uniform_f(u.core_color, color_get_red(s.core_color)/255, color_get_green(s.core_color)/255, color_get_blue(s.core_color)/255, s.core_alpha * a);

    var pen_x = _x, pen_y = _y;
    for (var i = 0; i < glyph_count; i++) {
        var g = glyphs[i];
        var entry = RustySDF_AtlasPackGlyph(g.font_handle, g.glyph_id, base_font_size, spread);
        if (is_undefined(entry) || variable_struct_get(entry, "raw_w") <= 0) { pen_x += g.x_advance; continue; }

        var atlas_pad = max(global.rusty_sdf_atlas.padding, spread + 1);
        var draw_x = pen_x + g.x_offset + (variable_struct_get(entry, "x_min") * scale) - (atlas_pad * scale);
        var draw_y = pen_y - g.y_offset - (variable_struct_get(entry, "y_max") * scale) - (atlas_pad * scale);
    
        draw_surface_part_ext(
            global.rusty_sdf_atlas.pages[variable_struct_get(entry, "page_index")].surface,
            variable_struct_get(entry, "atlas_x"), variable_struct_get(entry, "atlas_y"),
            variable_struct_get(entry, "w"), variable_struct_get(entry, "h"),
            draw_x, draw_y, scale, scale, c_white, 1.0
        );
        pen_x += g.x_advance;
        if (!is_undefined(g.y_advance)) pen_y -= g.y_advance;
    }

    shader_reset();
    RustySDF_FreeShape(shape);
    return 0;
}