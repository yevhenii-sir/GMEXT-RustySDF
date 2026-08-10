RustySDF_TextStyleSet(
    sdf_core_color, sdf_core_alpha,
    sdf_outline_color, sdf_outline_alpha,
    sdf_use_outline ? sdf_outline_width : 0.0,
    sdf_glow_color, sdf_use_glow ? sdf_glow_alpha : 0.0,
    sdf_glow_radius, sdf_boldness
);

var timer_start = get_timer();

text_obj.draw(room_width / 2, 200);
rich_text_obj.draw(room_width / 2, 500);

var timer_end = get_timer();
var ms_taken = (timer_end - timer_start) / 1000.0;
//show_debug_message($"Render 2 texts time -> {timer_end - timer_start}")

draw_set_color(c_white);
draw_text(10, 10, "RustySDF Interactive Showcase");
draw_text(10, 30, "Current Mode: " + (RustySDF_GetMode() >= 2 ? "MSDF / MTSDF" : "SDF / PSDF"));
draw_text(10, 50, "Vertex Rendering Time: " + string_format(ms_taken, 1, 3) + " ms");
draw_text(10, 70, "F1: Toggle Configurator");