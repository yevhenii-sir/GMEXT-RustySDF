{
  "$GMExtension": "",
  "%Name": "RustySDF",
  "androidactivityinject": "",
  "androidclassname": "RustySDF",
  "androidcodeinjection": "",
  "androidinject": "",
  "androidmanifestinject": "",
  "androidPermissions": [],
  "androidProps": true,
  "androidsourcedir": "",
  "author": "",
  "classname": "RustySDF",
  "copyToTargets": 9007199254741198,
  "description": "High-performance SDF text rendering extension powered by Rust",
  "exportToGame": true,
  "extensionVersion": "1.1.0",
  "files": [
    {
      "$GMExtensionFile": "v1",
      "%Name": "",
      "constants": [],
      "copyToTargets": 9007199254741198,
      "filename": "RustySDF.ext",
      "final": "",
      "functions": [
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_load_font",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_load_font",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_load_font",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_free_font",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_free_font",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_free_font",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_add_fallback",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} fallback_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_add_fallback",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_add_fallback",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_font_glyph_count",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_font_glyph_count",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_font_glyph_count",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_shape_text",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {String} text\r\n@param {Real} font_size\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_shape_text",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_shape_text",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_free_shape",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} shape_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_free_shape",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_free_shape",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_shape_width",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} shape_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_shape_width",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_shape_width",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_shape_height",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} shape_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_shape_height",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_shape_height",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_shape_glyph_count",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} shape_handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_shape_glyph_count",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_shape_glyph_count",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_shape_glyphs_buffer",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} shape_handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_shape_glyphs_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_set_bidi_mode",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} mode\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_set_bidi_mode",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_set_bidi_mode",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_set_buffer",
          "argCount": 3,
          "args": [
            1,
            2,
            2
          ],
          "documentation": "@param {Pointer} buffer_ptr\r\n@param {Real} buf_w\r\n@param {Real} buf_h\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_set_buffer",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_set_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_set_params",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} padding\r\n@param {Real} spread\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_set_params",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_set_params",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_set_mode",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} mode\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_set_mode",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_set_mode",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_mode",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_mode",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_mode",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_buffer_bpp",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_buffer_bpp",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_buffer_bpp",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_get_glyph_bounds",
          "argCount": 4,
          "args": [
            1,
            2,
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_glyph_bounds",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_get_glyph_bounds",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_render_glyph",
          "argCount": 3,
          "args": [
            2,
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} glyph_id\r\n@param {Real} font_size\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_render_glyph",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_render_glyph",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_render_char",
          "argCount": 3,
          "args": [
            2,
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} char_code\r\n@param {Real} font_size\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_render_char",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_render_char",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_request_glyph",
          "argCount": 6,
          "args": [
            2,
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} glyph_id\r\n@param {Real} font_size\r\n@param {Real} padding\r\n@param {Real} spread\r\n@param {Real} mode\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_request_glyph",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_request_glyph",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_poll_glyph",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_poll_glyph",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_poll_glyph",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_poll_glyph_pixels",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_poll_glyph_pixels",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_poll_glyph_pixels",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_poll_glyph_pixels_strided",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_poll_glyph_pixels_strided",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_measure_text",
          "argCount": 4,
          "args": [
            1,
            2,
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_measure_text",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_measure_text",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_ping",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {String}",
          "externalName": "__EXT_NATIVE__rusty_sdf_ping",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_ping",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_get_last_error",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {String}",
          "externalName": "__EXT_NATIVE__rusty_sdf_get_last_error",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_get_last_error",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_init",
          "argCount": 3,
          "args": [
            2,
            2,
            2
          ],
          "documentation": "@param {Real} width\r\n@param {Real} height\r\n@param {Real} padding\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_init",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_init",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_reset",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_reset",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_reset",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_clear",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_clear",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_clear",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_get_version",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_get_version",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_get_version",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_page_count",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_page_count",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_page_count",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_ensure_glyph",
          "argCount": 6,
          "args": [
            2,
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} glyph_id\r\n@param {Real} base_size\r\n@param {Real} spread\r\n@param {Real} mode\r\n@param {Real} async_flag\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_ensure_glyph",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_ensure_glyph",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_atlas_lookup",
          "argCount": 4,
          "args": [
            1,
            2,
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_lookup",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_atlas_lookup",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_atlas_commit_glyph",
          "argCount": 10,
          "args": [
            2,
            2,
            2,
            2,
            2,
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} font_handle\r\n@param {Real} glyph_id\r\n@param {Real} base_size\r\n@param {Real} spread\r\n@param {Real} width\r\n@param {Real} height\r\n@param {Real} raw_w\r\n@param {Real} raw_h\r\n@param {Real} x_min\r\n@param {Real} y_max\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_commit_glyph",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_atlas_commit_glyph",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_atlas_poll_dirty_meta",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_atlas_poll_dirty_meta",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__rusty_sdf_atlas_poll_dirty_pixels",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__rusty_sdf_atlas_poll_dirty_pixels",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_create",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_create",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_create",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_free",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_free",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_free",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_text",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} text\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_text",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_text",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_font",
          "argCount": 5,
          "args": [
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} font_handle\r\n@param {Real} font_size\r\n@param {Real} base_size\r\n@param {Real} spread\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_font",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_font",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_layout",
          "argCount": 6,
          "args": [
            2,
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} max_width\r\n@param {Real} line_height\r\n@param {Real} letter_spacing\r\n@param {Real} halign\r\n@param {Real} valign\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_layout",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_layout",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_default_style",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_default_style",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_default_style",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_async",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} enabled\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_async",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_async",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_plain",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} enabled\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_plain",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_plain",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_config",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_config",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_config",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_register_image",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} name\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_register_image",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_register_image",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_set_image_metrics",
          "argCount": 5,
          "args": [
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} spr_w\r\n@param {Real} spr_h\r\n@param {Real} xoff\r\n@param {Real} yoff\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_set_image_metrics",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_set_image_metrics",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_clear_images",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_clear_images",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_clear_images",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_build",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_build",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_build",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_metrics_buffer",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_metrics_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_page_byte_size",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} page\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_page_byte_size",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_page_byte_size",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_write_page_vertices",
          "argCount": 4,
          "args": [
            2,
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} page\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_write_page_vertices",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_write_page_vertices",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_images_buffer",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_images_buffer",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_images_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_image_name",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} index\r\n@returns {String}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_image_name",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_image_name",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_glyph_meta_buffer",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} buffer_ptr\r\n@param {Real} buffer_len\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_glyph_meta_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "rusty_sdf_rich_get_plain_text",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {String}",
          "externalName": "__EXT_NATIVE__rusty_sdf_rich_get_plain_text",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "rusty_sdf_rich_get_plain_text",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__RustySDF_queue_buffer",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _buffer_ptr\r\n@param {Real} _buffer_size",
          "externalName": "__EXT_NATIVE__RustySDF_queue_buffer",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__RustySDF_queue_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        }
      ],
      "init": "",
      "kind": 4,
      "name": "",
      "origname": "",
      "ProxyFiles": [
        {
          "$GMProxyFile": "",
          "%Name": "RustySDF.dll",
          "name": "RustySDF.dll",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 6
        },
        {
          "$GMProxyFile": "",
          "%Name": "libRustySDF.dylib",
          "name": "libRustySDF.dylib",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 1
        },
        {
          "$GMProxyFile": "",
          "%Name": "libRustySDF.so",
          "name": "libRustySDF.so",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 7
        }
      ],
      "resourceType": "GMExtensionFile",
      "resourceVersion": "2.0",
      "uncompress": false,
      "usesRunnerInterface": false
    }
  ],
  "gradleinject": "",
  "hasConvertedCodeInjection": true,
  "helpfile": "",
  "HTML5CodeInjection": "",
  "html5Props": false,
  "IncludedResources": [],
  "installdir": "",
  "iosCocoaPodDependencies": "",
  "iosCocoaPods": "",
  "ioscodeinjection": "",
  "iosdelegatename": "",
  "iosplistinject": "",
  "iosProps": true,
  "iosSystemFrameworkEntries": [],
  "iosThirdPartyFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "RustySDF_Rust.xcframework",
      "embed": 1,
      "name": "RustySDF_Rust.xcframework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ],
  "license": "",
  "maccompilerflags": "",
  "maclinkerflags": "-ObjC",
  "macsourcedir": "",
  "name": "RustySDF",
  "options": [],
  "optionsFile": "options.json",
  "packageId": "",
  "parent": {
    "name": "RustySDF",
    "path": "folders/Extensions/RustySDF.yy"
  },
  "productId": "",
  "resourceType": "GMExtension",
  "resourceVersion": "2.0",
  "sourcedir": "",
  "supportedTargets": -1,
  "tvosclassname": "RustySDF",
  "tvosCocoaPodDependencies": "",
  "tvosCocoaPods": "",
  "tvoscodeinjection": "",
  "tvosdelegatename": "",
  "tvosmaccompilerflags": "",
  "tvosmaclinkerflags": "-ObjC",
  "tvosplistinject": "",
  "tvosProps": true,
  "tvosSystemFrameworkEntries": [],
  "tvosThirdPartyFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "RustySDF_Rust.xcframework",
      "embed": 1,
      "name": "RustySDF_Rust.xcframework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ]
}