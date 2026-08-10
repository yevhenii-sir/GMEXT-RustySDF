/// @func RustySDF_AsyncManager()
/// @desc Polls async glyph results and commits them into the Rust atlas + GPU surfaces.
function RustySDF_AsyncManager() constructor {
    pending_glyphs = [];

    static set_dummy = function(_info) {
        rusty_sdf_atlas_commit_glyph(
            _info.font_handle, _info.glyph_id, _info.font_size, _info.spread,
            0, 0, 0, 0, 0, 0
        );
        global.rusty_sdf_atlas.version = rusty_sdf_atlas_get_version();
    };

    static update = function() {
        while (true) {
            var info = RustySDF_PollGlyph();
            if (is_undefined(info)) break;

            if (info.width == 0 && info.height == 0) {
                set_dummy(info);
                continue;
            }

            var atlas = global.rusty_sdf_atlas;
            if (info.width > atlas.width || info.height > atlas.height) {
                show_debug_message("[RustySDF] WARNING: Glyph too huge. Replacing with empty space.");
                set_dummy(info);
                continue;
            }

            var req_size = info.width * info.height * 4;
            var pixel_buf = global.rusty_sdf_pixel_buf;
            if (buffer_get_size(pixel_buf) < req_size) {
                buffer_resize(pixel_buf, req_size);
            }

            var copied = RustySDF_PollGlyphPixels(pixel_buf);
            if (copied <= 0) {
                set_dummy(info);
                continue;
            }

            var ok = rusty_sdf_atlas_commit_glyph(
                info.font_handle, info.glyph_id, info.font_size, info.spread,
                info.width, info.height, info.raw_w, info.raw_h, info.x_min, info.y_max
            );
            if (ok < 0) {
                set_dummy(info);
                continue;
            }

            __RustySDF_AtlasSyncPageSurfaces();
            var entry = __RustySDF_AtlasLookupEntry(info.font_handle, info.glyph_id, info.font_size, info.spread);
            if (is_undefined(entry) || entry.w <= 0) {
                atlas.version = rusty_sdf_atlas_get_version();
                continue;
            }

            var upload = __RustySDF_UploadPackedGlyphToScratch(pixel_buf, info.width, info.height);
            if (is_undefined(upload)) {
                set_dummy(info);
                continue;
            }

            var dest_surf = atlas.pages[entry.page_index].surface;
            if (__RustySDF_BlitUploadToAtlas(dest_surf, entry.atlas_x, entry.atlas_y)) {
                atlas.version = rusty_sdf_atlas_get_version();
            } else {
                set_dummy(info);
            }
        }
    }
}
