if (keyboard_check_pressed(vk_f1)) {
    show_debug_overlay(!is_debug_overlay_open());
}
if (keyboard_check_pressed(vk_escape)) {
    RustySDF_AtlasFree();
}

if (render_mode != RustySDF_GetMode()) {
    RustySDF_SetMode(render_mode);
    RustySDF_AtlasClear(); 
}

var parsed_plain_text = string_replace_all(plain_text_str, "\\n", "\n");
var parsed_rich_text = string_replace_all(rich_text_str, "\\n", "\n");

text_obj.set_font(fonts_array[plain_font_index])
        .set_base_size(atlas_base_size)
        .set_spread(atlas_spread)
        .set_text(parsed_plain_text)
        .set_font_size(plain_font_size)
        .set_wrap(plain_wrap)
        .set_align(plain_halign, plain_valign)
        .set_spacing(plain_line_height, plain_letter_spacing);

rich_text_obj.set_font(fonts_array[rich_font_index])
             .set_base_size(atlas_base_size)
             .set_spread(atlas_spread)
             .set_default_style(
                 rich_def_core_color, rich_def_core_alpha, rich_def_boldness,
                 rich_def_outline_color, rich_def_outline_alpha, rich_def_outline_width,
                 rich_def_glow_color, rich_def_glow_alpha, rich_def_glow_radius
             )
             .set_text(parsed_rich_text)
             .set_font_size(rich_font_size)
             .set_wrap(rich_wrap)
             .set_align(rich_halign, rich_valign);