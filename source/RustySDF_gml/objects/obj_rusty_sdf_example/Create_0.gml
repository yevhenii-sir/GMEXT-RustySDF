font_noto = RustySDF_GetFont("NotoSans-Regular.ttf");
font_desas = RustySDF_GetFont("PlaywriteDESAS-Regular.ttf");
font_arabic = RustySDF_GetFont("NotoSansArabic-Medium.ttf");
font_tc = RustySDF_GetFont("NotoSansTC-Regular.ttf");
font_hebrew = RustySDF_GetFont("NotoSerifHebrew-Regular.ttf");

RustySDF_AddFallback(font_desas, font_noto);
RustySDF_AddFallback(font_arabic, font_noto);
RustySDF_AddFallback(font_tc, font_noto);
RustySDF_AddFallback(font_hebrew, font_noto);

RustySDF_AddFallback(font_noto, font_arabic);
RustySDF_AddFallback(font_noto, font_tc);
RustySDF_AddFallback(font_noto, font_hebrew);

fonts_array =[font_noto, font_desas, font_arabic, font_tc, font_hebrew];
plain_font_index = 0;
rich_font_index = 0;

render_mode = ERustySDFMode.SDF; 

atlas_base_size = global.rusty_sdf_default_base_size;
atlas_spread = global.rusty_sdf_default_spread;

sdf_boldness = 0.5;
sdf_outline_width = 0.08;
sdf_glow_radius = 0.20;

sdf_core_color = c_white; sdf_outline_color = c_black; sdf_glow_color = make_color_rgb(0, 153, 255);
sdf_core_alpha = 1.0; sdf_outline_alpha = 1.0; sdf_glow_alpha = 0.0;
sdf_use_outline = true; sdf_use_glow = true;

rich_def_core_color = c_white; rich_def_core_alpha = 1.0; rich_def_boldness = 0.36;
rich_def_outline_color = c_black; rich_def_outline_alpha = 1.0; rich_def_outline_width = 0.0;
rich_def_glow_color = c_aqua; rich_def_glow_alpha = 0.0; rich_def_glow_radius = 0.20;

reset_shader_debug = function() {
    sdf_boldness = 0.5; sdf_outline_width = 0.08; sdf_glow_radius = 0.20;
    sdf_core_color = c_white; sdf_outline_color = c_black; sdf_glow_color = make_color_rgb(0, 153, 255);
    sdf_core_alpha = 1.0; sdf_outline_alpha = 1.0; sdf_glow_alpha = 0.0;
    sdf_use_outline = true; sdf_use_glow = true;
};
reset_rich_debug = function() {
    rich_def_core_color = c_white; rich_def_core_alpha = 1.0; rich_def_boldness = 0.36;
    rich_def_outline_color = c_black; rich_def_outline_alpha = 1.0; rich_def_outline_width = 0.0;
    rich_def_glow_color = c_aqua; rich_def_glow_alpha = 0.0; rich_def_glow_radius = 0.20;
};

plain_text_str = "Hello RustySDF!\\nПривет мир!\\nمرحبا بالعالم!\\n你好世界！\\nשלום עולם!";
plain_font_size = 54; plain_wrap = 1900; plain_halign = fa_center; plain_valign = fa_middle;
plain_line_height = 0; plain_letter_spacing = 0;
text_obj = new RustySDF_Text(font_arabic);

rich_text_str = "Hello[c=#FF0000][b=0.36]World[/b][/c]!\\n[c=#0000FF][ul]Welcome to[/ul][/c] [oc=#000000][ow=0.17]RustySDF[/ow][/oc] [st]Rich Text[/st].\\nReward: 1500 [img=spr_coin] — [c=#00FF00][b=0.30]Русский[/b][/c], مرحبا, 你好 и שלום.";
rich_font_size = 40; rich_wrap = 600; rich_halign = fa_center; rich_valign = fa_top;
rich_text_obj = new RustySDF_RichText(font_noto);

custom_dbgview = dbg_view("RustySDF Configurator", true);

dbg_section("Rendering Engine");
dbg_drop_down(ref_create(self, "render_mode"), "SDF:0,PSDF:1,MSDF:2,MTSDF:3", "Render Mode");
dbg_button("Clear Atlas Buffer", function() { RustySDF_AtlasClear(); });

dbg_section("Atlas Generation Params");
dbg_slider(ref_create(self, "atlas_base_size"), 16, 128, "Base Font Size", 1);
dbg_slider(ref_create(self, "atlas_spread"), 1, 32, "Spread Distance", 1);

dbg_section("Plain Text Formatting");
dbg_drop_down(ref_create(self, "plain_font_index"), "NotoSans:0,PlaywriteDESAS:1,Arabic:2,Chinese TC:3,Hebrew:4", "Font");
dbg_text_input(ref_create(self, "plain_text_str"), "Text String");
dbg_slider(ref_create(self, "plain_font_size"), 8, 128, "Font Size", 1);
dbg_slider(ref_create(self, "plain_wrap"), 0, 1200, "Wrap Width", 10);
dbg_drop_down(ref_create(self, "plain_halign"), "Left:0,Center:1,Right:2", "Align X");
dbg_drop_down(ref_create(self, "plain_valign"), "Top:0,Middle:1,Bottom:2", "Align Y");
dbg_slider(ref_create(self, "plain_line_height"), 0, 150, "Line Height (0=Auto)", 1);
dbg_slider(ref_create(self, "plain_letter_spacing"), -10, 50, "Letter Spacing", 1);

dbg_section("Plain Text - Global Shader Style");
dbg_slider(ref_create(self, "sdf_boldness"), 0.0, 1.0, "Boldness", 0.005);
dbg_slider(ref_create(self, "sdf_outline_width"), 0.0, 0.4, "Outline Width", 0.005);
dbg_checkbox(ref_create(self, "sdf_use_outline"), "Enable Outline");
dbg_slider(ref_create(self, "sdf_glow_radius"), 0.0, 0.8, "Glow Radius", 0.005);
dbg_checkbox(ref_create(self, "sdf_use_glow"), "Enable Glow");
dbg_slider(ref_create(self, "sdf_glow_alpha"), 0.0, 1.0, "Glow Alpha", 0.01);
dbg_colour(ref_create(self, "sdf_core_color"), "Core Color");
dbg_colour(ref_create(self, "sdf_outline_color"), "Outline Color");
dbg_colour(ref_create(self, "sdf_glow_color"), "Glow Color");
dbg_button("Reset Plain Style", ref_create(self, "reset_shader_debug"));

dbg_section("Rich Text Formatting");
dbg_drop_down(ref_create(self, "rich_font_index"), "NotoSans:0,PlaywriteDESAS:1,Arabic:2,Chinese TC:3,Hebrew:4", "Font");
dbg_text_input(ref_create(self, "rich_text_str"), "BBCode String");
dbg_slider(ref_create(self, "rich_font_size"), 8, 128, "Font Size", 1);
dbg_slider(ref_create(self, "rich_wrap"), 0, 1200, "Wrap Width", 10);
dbg_drop_down(ref_create(self, "rich_halign"), "Left:0,Center:1,Right:2", "Align X");

dbg_section("Rich Text - Default Style");
dbg_colour(ref_create(self, "rich_def_core_color"), "Core Color");
dbg_slider(ref_create(self, "rich_def_core_alpha"), 0.0, 1.0, "Core Alpha", 0.01);
dbg_slider(ref_create(self, "rich_def_boldness"), 0.0, 1.0, "Boldness", 0.005);
dbg_colour(ref_create(self, "rich_def_outline_color"), "Outline Color");
dbg_slider(ref_create(self, "rich_def_outline_width"), 0.0, 0.4, "Outline Width", 0.005);
dbg_colour(ref_create(self, "rich_def_glow_color"), "Glow Color");
dbg_slider(ref_create(self, "rich_def_glow_alpha"), 0.0, 1.0, "Glow Alpha", 0.01);
dbg_slider(ref_create(self, "rich_def_glow_radius"), 0.0, 0.8, "Glow Radius", 0.005);
dbg_button("Reset Rich Defaults", ref_create(self, "reset_rich_debug"));

show_debug_overlay(true);