if (!variable_global_exists("rusty_sdf_text_cache")) {
    global.rusty_sdf_text_cache = {};
}
if (!variable_global_exists("rusty_sdf_rich_cache")) {
    global.rusty_sdf_rich_cache = {};
}

global.default_text_bold = 0.47;
global.default_text_outline = 0.165;

/// @func RustySDF_CacheDraw(id, font, text, font_size, [wrap], [halign], [valign], [fit_w], [fit_h])
function RustySDF_CacheDraw(_id, _font, _text, _font_size, _wrap = 0, _halign = fa_center, _valign = fa_middle, _fit_width = -1, _fit_height = -1) {
    var _txt_obj = global.rusty_sdf_text_cache[$ _id];

	if (_font == -1) {
		_font = get_locale_font(global.gameProgress[? "locale"]);
	}

	var is_initialization = false;
    if (is_undefined(_txt_obj)) {
        _txt_obj = new RustySDF_Text(_font, _font_size);
        _txt_obj.draw_ext = method(_txt_obj, function(_x, _y, _xscale = 1, _yscale = 1, _rot = 0, _alpha = 1.0) {
            set_alpha(_alpha);
            draw(_x, _y, _xscale, _yscale, _rot);
            return self;
        });

        global.rusty_sdf_text_cache[$ _id] = _txt_obj;
    }

    _txt_obj.set_font(_font)
            .set_font_size(_font_size)
            .set_text(_text)
            .set_wrap(_wrap)
            .set_align(_halign, _valign)
            .set_fit_size(_fit_width, _fit_height);

    return _txt_obj;
}

/// @func RustySDF_CacheDrawRich(id, font, text, font_size, [wrap], [halign], [valign], [fit_w], [fit_h])
/// @desc Like RustySDF_CacheDraw but uses RustySDF_RichText which supports [img=], [c=], [b=], etc. tags.
function RustySDF_CacheDrawRich(_id, _font, _text, _font_size, _wrap = 0, _halign = fa_center, _valign = fa_middle, _fit_width = -1, _fit_height = -1) {
    var _txt_obj = global.rusty_sdf_rich_cache[$ _id];

    if (_font == -1) {
        _font = get_locale_font(global.gameProgress[? "locale"]);
    }

    if (is_undefined(_txt_obj)) {
        _txt_obj = new RustySDF_RichText(_font, _font_size);
        _txt_obj.draw_ext = method(_txt_obj, function(_x, _y, _xscale = 1, _yscale = 1, _rot = 0, _alpha = 1.0) {
            set_alpha(_alpha);
            draw(_x, _y, _xscale, _yscale, _rot);
            return self;
        });

        global.rusty_sdf_rich_cache[$ _id] = _txt_obj;
    }

    _txt_obj.set_font(_font)
            .set_font_size(_font_size)
            .set_text(_text)
            .set_wrap(_wrap)
            .set_align(_halign, _valign)
            .set_fit_size(_fit_width, _fit_height);

    return _txt_obj;
}

/// @func RustySDF_CacheClear()
function RustySDF_CacheClear() {
    var _keys = struct_get_names(global.rusty_sdf_text_cache);
    for (var i = 0; i < array_length(_keys); i++) {
        var _txt_obj = global.rusty_sdf_text_cache[$ _keys[i]];
        _txt_obj.free();
    }
    global.rusty_sdf_text_cache = {};
}

/// @func RustySDF_CacheClearRich()
/// @desc Free all cached RustySDF_RichText objects.
function RustySDF_CacheClearRich() {
    if (!variable_global_exists("rusty_sdf_rich_cache")) return;
    var _keys = struct_get_names(global.rusty_sdf_rich_cache);
    for (var i = 0; i < array_length(_keys); i++) {
        var _txt_obj = global.rusty_sdf_rich_cache[$ _keys[i]];
        _txt_obj.free();
    }
    global.rusty_sdf_rich_cache = {};
}
