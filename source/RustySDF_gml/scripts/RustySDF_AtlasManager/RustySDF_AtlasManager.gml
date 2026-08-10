/// RustySDF_AtlasManager
/// GPU surfaces + upload scratch in GML; shelf pack / UV cache live in Rust.

if (!variable_global_exists("rusty_sdf_atlas")) {
    global.rusty_sdf_atlas = {
        pages: [],            // array of { surface: -1 }
        width: 1024,
        height: 1024,
        padding: 4,
        version: 0
    };
}

if (!variable_global_exists("rusty_sdf_async_enabled")) {
    global.rusty_sdf_async_enabled = true;
}

if (!variable_global_exists("rusty_sdf_upload")) {
    global.rusty_sdf_upload = { surf: -1, buf: -1, w: 0, h: 0 };
}

if (!variable_global_exists("rusty_sdf_atlas_lookup_buf")) {
    global.rusty_sdf_atlas_lookup_buf = buffer_create(256, buffer_fixed, 1);
}

if (!variable_global_exists("rusty_sdf_atlas_dirty_meta_buf")) {
    global.rusty_sdf_atlas_dirty_meta_buf = buffer_create(128, buffer_fixed, 1);
}

/// @func __RustySDF_EnsureGlyphUploadTarget(w, h)
function __RustySDF_EnsureGlyphUploadTarget(w, h) {
    var u = global.rusty_sdf_upload;
    var reused = surface_exists(u.surf) && u.w == w && u.h == h;

    if (!reused) {
        if (surface_exists(u.surf)) surface_free(u.surf);
        u.surf = surface_create(w, h, surface_rgba8unorm);
        u.w = w;
        u.h = h;
    }

    var bytes = w * h * 4;
    if (u.buf < 0 || !buffer_exists(u.buf)) {
        u.buf = buffer_create(max(bytes, 1024 * 64), buffer_fixed, 1);
    } else if (buffer_get_size(u.buf) < bytes) {
        buffer_delete(u.buf);
        u.buf = buffer_create(bytes, buffer_fixed, 1);
    }

    return u;
}

/// @func __RustySDF_BlitUploadToAtlas(dest_surf, placed_x, placed_y)
function __RustySDF_BlitUploadToAtlas(dest_surf, placed_x, placed_y) {
    var u = global.rusty_sdf_upload;
    if (!surface_exists(dest_surf) || !surface_exists(u.surf)) return false;

    surface_set_target(dest_surf);
        gpu_set_blendmode_ext(bm_one, bm_zero);
        draw_surface(u.surf, placed_x, placed_y);
        gpu_set_blendmode(bm_normal);
    surface_reset_target();
    return true;
}

/// @func __RustySDF_UploadPackedGlyphToScratch(src_buf, glyph_w, glyph_h)
function __RustySDF_UploadPackedGlyphToScratch(src_buf, glyph_w, glyph_h) {
    var u = __RustySDF_EnsureGlyphUploadTarget(glyph_w, glyph_h);
    if (!surface_exists(u.surf)) return undefined;
    buffer_set_surface(src_buf, u.surf, 0);
    return u;
}

function __RustySDF_AtlasKey(font_handle, glyph_id, base_font_size, spread) {
    var b = round(base_font_size);
    var s = round(spread);
    return font_handle * 8589934592 + glyph_id * 131072 + b * 256 + s;
}

function __RustySDF_AtlasAddPageSurface() {
    var atlas = global.rusty_sdf_atlas;
    var surf = surface_create(atlas.width, atlas.height, surface_rgba8unorm);
    if (surface_exists(surf)) {
        surface_set_target(surf);
        draw_clear_alpha(c_black, 0);
        surface_reset_target();
    }
    array_push(atlas.pages, { surface: surf });
    return array_length(atlas.pages) - 1;
}

/// Ensure GML page surfaces match native page_count.
function __RustySDF_AtlasSyncPageSurfaces() {
    var atlas = global.rusty_sdf_atlas;
    var need = rusty_sdf_atlas_page_count();
    while (array_length(atlas.pages) < need) {
        __RustySDF_AtlasAddPageSurface();
    }
}

/// Flush dirty glyph pixels from Rust into atlas surfaces.
function RustySDF_AtlasFlushDirty() {
    var atlas = global.rusty_sdf_atlas;
    var meta_buf = global.rusty_sdf_atlas_dirty_meta_buf;
    var pixel_buf = global.rusty_sdf_pixel_buf;
    if (!variable_global_exists("rusty_sdf_pixel_buf") || !buffer_exists(global.rusty_sdf_pixel_buf)) {
        global.rusty_sdf_pixel_buf = buffer_create(1024 * 1024, buffer_grow, 1);
        pixel_buf = global.rusty_sdf_pixel_buf;
    }

    while (true) {
        var written = rusty_sdf_atlas_poll_dirty_meta(buffer_get_address(meta_buf), buffer_get_size(meta_buf));
        if (written <= 0) break;

        buffer_seek(meta_buf, buffer_seek_start, 0);
        var flat = __ext_core_buffer_unmarshal_value(meta_buf, []);
        if (!is_array(flat) || array_length(flat) < 5) break;

        var page = flat[0];
        var px = flat[1];
        var py = flat[2];
        var w = flat[3];
        var h = flat[4];

        __RustySDF_AtlasSyncPageSurfaces();
        if (page < 0 || page >= array_length(atlas.pages)) continue;

        var req = w * h * 4;
        if (buffer_get_size(pixel_buf) < req) buffer_resize(pixel_buf, req);

        var copied = rusty_sdf_atlas_poll_dirty_pixels(buffer_get_address(pixel_buf), buffer_get_size(pixel_buf));
        if (copied <= 0) continue;

        var upload = __RustySDF_UploadPackedGlyphToScratch(pixel_buf, w, h);
        if (is_undefined(upload)) continue;
        __RustySDF_BlitUploadToAtlas(atlas.pages[page].surface, px, py);
    }

    atlas.version = rusty_sdf_atlas_get_version();
}

function __RustySDF_AtlasLookupEntry(font_handle, glyph_id, base_font_size, spread) {
    rusty_sdf_atlas_prepare_lookup(font_handle, glyph_id, base_font_size, spread);
    var buf = global.rusty_sdf_atlas_lookup_buf;
    var written = rusty_sdf_atlas_lookup_buffer(buffer_get_address(buf), buffer_get_size(buf));
    if (written <= 0) return undefined;
    buffer_seek(buf, buffer_seek_start, 0);
    var flat = __ext_core_buffer_unmarshal_value(buf, []);
    if (!is_array(flat) || array_length(flat) < 1 || flat[0] < 1) return undefined;
    if (array_length(flat) < 15) return undefined;
    return {
        page_index: flat[1],
        atlas_x: flat[2], atlas_y: flat[3],
        w: flat[4], h: flat[5], raw_w: flat[6], raw_h: flat[7],
        u1: flat[8], v1: flat[9], u2: flat[10], v2: flat[11],
        x_min: flat[12], y_max: flat[13],
        glyph_id: glyph_id, font_handle: font_handle, base_font_size: base_font_size,
        _async_pending: (flat[14] > 0)
    };
}

function RustySDF_AtlasCheckSurfaces() {
    var atlas = global.rusty_sdf_atlas;
    var lost = false;
    for (var i = 0; i < array_length(atlas.pages); i++) {
        if (!surface_exists(atlas.pages[i].surface)) {
            lost = true;
            break;
        }
    }
    if (lost) {
        show_debug_message("RustySDF_AtlasManager: Surfaces lost! Rebuilding atlases...");
        RustySDF_AtlasReset();
        return true;
    }
    return false;
}

function RustySDF_AtlasInit(width, height, padding) {
    var atlas = global.rusty_sdf_atlas;
    RustySDF_AtlasFree();

    atlas.width = width;
    atlas.height = height;
    atlas.padding = padding;
    atlas.pages = [];

    rusty_sdf_atlas_init(width, height, padding);
    __RustySDF_AtlasAddPageSurface();
    atlas.version = rusty_sdf_atlas_get_version();
}

function RustySDF_AtlasClear() {
    var atlas = global.rusty_sdf_atlas;

    for (var i = 0; i < array_length(atlas.pages); i++) {
        var surf = atlas.pages[i].surface;
        if (surface_exists(surf)) surface_free(surf);
    }
    atlas.pages = [];

    rusty_sdf_atlas_clear();
    __RustySDF_AtlasAddPageSurface();
    atlas.version = rusty_sdf_atlas_get_version();
}

function RustySDF_AtlasReset() {
    var atlas = global.rusty_sdf_atlas;
    for (var i = 0; i < array_length(atlas.pages); i++) {
        var surf = atlas.pages[i].surface;
        if (!surface_exists(surf)) {
            surf = surface_create(atlas.width, atlas.height, surface_rgba8unorm);
            atlas.pages[i].surface = surf;
        }
        if (surface_exists(surf)) {
            surface_set_target(surf);
            draw_clear_alpha(c_black, 0);
            surface_reset_target();
        }
    }

    rusty_sdf_atlas_reset();
    __RustySDF_AtlasSyncPageSurfaces();
    atlas.version = rusty_sdf_atlas_get_version();
}

function RustySDF_AtlasFree() {
    var atlas = global.rusty_sdf_atlas;
    for (var i = 0; i < array_length(atlas.pages); i++) {
        var surf = atlas.pages[i].surface;
        if (surface_exists(surf)) surface_free(surf);
    }
    atlas.pages = [];

    var u = global.rusty_sdf_upload;
    if (surface_exists(u.surf)) surface_free(u.surf);
    if (u.buf >= 0 && buffer_exists(u.buf)) buffer_delete(u.buf);
    u.surf = -1;
    u.buf = -1;
    u.w = 0;
    u.h = 0;

    rusty_sdf_atlas_clear();
    atlas.version = rusty_sdf_atlas_get_version();
}

/// @func RustySDF_AtlasPackGlyph(font_handle, glyph_id, base_font_size, spread)
function RustySDF_AtlasPackGlyph(font_handle, glyph_id, base_font_size, spread) {
    RustySDF_AtlasCheckSurfaces();

    var existing = __RustySDF_AtlasLookupEntry(font_handle, glyph_id, base_font_size, spread);
    if (!is_undefined(existing)) {
        if (variable_struct_get(existing, "_async_pending")) return undefined;
        return existing;
    }

    var async_flag = global.rusty_sdf_async_enabled ? 1 : 0;
    var code = rusty_sdf_atlas_ensure_glyph(font_handle, glyph_id, base_font_size, spread, global.rusty_sdf_mode, async_flag);

    if (code == 0) {
        // pending async
        return undefined;
    }

    if (code == 2 || code == 1) {
        RustySDF_AtlasFlushDirty();
        __RustySDF_AtlasSyncPageSurfaces();
    }

    return __RustySDF_AtlasLookupEntry(font_handle, glyph_id, base_font_size, spread);
}
