// ##### extgen :: Auto-generated file do not edit!! #####

#import "RustySDFInternal_ios.h"
#import <objc/runtime.h>

extern "C" {

double __EXT_NATIVE__rusty_sdf_load_font(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__rusty_sdf_free_font(double font_handle);
double __EXT_NATIVE__rusty_sdf_add_fallback(double font_handle, double fallback_handle);
double __EXT_NATIVE__rusty_sdf_get_font_glyph_count(double font_handle);
double __EXT_NATIVE__rusty_sdf_shape_text(double font_handle, char* text, double font_size);
double __EXT_NATIVE__rusty_sdf_free_shape(double shape_handle);
double __EXT_NATIVE__rusty_sdf_get_shape_width(double shape_handle);
double __EXT_NATIVE__rusty_sdf_get_shape_height(double shape_handle);
double __EXT_NATIVE__rusty_sdf_get_shape_glyph_count(double shape_handle);
double __EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer(double shape_handle, char* buffer_ptr, double buffer_len);
double __EXT_NATIVE__rusty_sdf_set_bidi_mode(double mode);
double __EXT_NATIVE__rusty_sdf_set_buffer(char* buffer_ptr, double buf_w, double buf_h);
double __EXT_NATIVE__rusty_sdf_set_params(double padding, double spread);
double __EXT_NATIVE__rusty_sdf_set_mode(double mode);
double __EXT_NATIVE__rusty_sdf_get_mode(void);
double __EXT_NATIVE__rusty_sdf_get_buffer_bpp(void);
double __EXT_NATIVE__rusty_sdf_get_glyph_bounds(char* __arg_buffer, double __arg_buffer_length, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__rusty_sdf_render_glyph(double font_handle, double glyph_id, double font_size);
double __EXT_NATIVE__rusty_sdf_render_char(double font_handle, double char_code, double font_size);
double __EXT_NATIVE__rusty_sdf_request_glyph(double font_handle, double glyph_id, double font_size, double padding, double spread, double mode);
double __EXT_NATIVE__rusty_sdf_poll_glyph(char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__rusty_sdf_poll_glyph_pixels(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__rusty_sdf_measure_text(char* __arg_buffer, double __arg_buffer_length, char* __ret_buffer, double __ret_buffer_length);
char* __EXT_NATIVE__rusty_sdf_ping(void);
char* __EXT_NATIVE__rusty_sdf_get_last_error(void);
double __EXT_NATIVE__rusty_sdf_atlas_init(double width, double height, double padding);
double __EXT_NATIVE__rusty_sdf_atlas_reset(void);
double __EXT_NATIVE__rusty_sdf_atlas_clear(void);
double __EXT_NATIVE__rusty_sdf_atlas_get_version(void);
double __EXT_NATIVE__rusty_sdf_atlas_page_count(void);
double __EXT_NATIVE__rusty_sdf_atlas_ensure_glyph(double font_handle, double glyph_id, double base_size, double spread, double mode, double async_flag);
double __EXT_NATIVE__rusty_sdf_atlas_lookup(char* __arg_buffer, double __arg_buffer_length, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__rusty_sdf_atlas_commit_glyph(double font_handle, double glyph_id, double base_size, double spread, double width, double height, double raw_w, double raw_h, double x_min, double y_max);
double __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__rusty_sdf_rich_create(void);
double __EXT_NATIVE__rusty_sdf_rich_free(double handle);
double __EXT_NATIVE__rusty_sdf_rich_set_text(double handle, char* text);
double __EXT_NATIVE__rusty_sdf_rich_set_font(double handle, double font_handle, double font_size, double base_size, double spread);
double __EXT_NATIVE__rusty_sdf_rich_set_layout(double handle, double max_width, double line_height, double letter_spacing, double halign, double valign);
double __EXT_NATIVE__rusty_sdf_rich_set_default_style(double handle, char* buffer_ptr, double buffer_len);
double __EXT_NATIVE__rusty_sdf_rich_set_async(double handle, double enabled);
double __EXT_NATIVE__rusty_sdf_rich_set_plain(double handle, double enabled);
double __EXT_NATIVE__rusty_sdf_rich_set_config(double handle, char* buffer_ptr, double buffer_len);
double __EXT_NATIVE__rusty_sdf_rich_register_image(double handle, char* name);
double __EXT_NATIVE__rusty_sdf_rich_set_image_metrics(double handle, double spr_w, double spr_h, double xoff, double yoff);
double __EXT_NATIVE__rusty_sdf_rich_clear_images(double handle);
double __EXT_NATIVE__rusty_sdf_rich_build(double handle);
double __EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer(double handle, char* buffer_ptr, double buffer_len);
double __EXT_NATIVE__rusty_sdf_rich_get_page_byte_size(double handle, double page);
double __EXT_NATIVE__rusty_sdf_rich_write_page_vertices(double handle, double page, char* buffer_ptr, double buffer_len);
double __EXT_NATIVE__rusty_sdf_rich_get_images_buffer(double handle, char* buffer_ptr, double buffer_len);
char* __EXT_NATIVE__rusty_sdf_rich_get_image_name(double handle, double index);
double __EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer(double handle, char* buffer_ptr, double buffer_len);
char* __EXT_NATIVE__rusty_sdf_rich_get_plain_text(double handle);
double __EXT_NATIVE__RustySDF_queue_buffer(char* __arg_buffer, double __arg_buffer_length);
const char* __EXT_NATIVE__RustySDF_get_last_error(void);
}


static BOOL GMIsSubclassOf(Class cls, Class base)
{
    for (Class c = cls; c != Nil; c = class_getSuperclass(c)) {
        if (c == base) return YES;
    }
    return NO;
}

static void GMInjectSelectorsIntoSubclass(Class subclass, Class base)
{
    // Build set of methods already defined on subclass
    unsigned subCount = 0;
    Method *subList = class_copyMethodList(subclass, &subCount);

    CFMutableSetRef owned = CFSetCreateMutable(kCFAllocatorDefault, 0, NULL);
    for (unsigned i = 0; i < subCount; ++i) {
        CFSetAddValue(owned, method_getName(subList[i]));
    }

    // Walk base class methods
    unsigned baseCount = 0;
    Method *baseList = class_copyMethodList(base, &baseCount);

    for (unsigned i = 0; i < baseCount; ++i) {
        SEL sel = method_getName(baseList[i]);
        const char *name = sel_getName(sel);

        // Only inject extension selectors (methods prefixed with __EXT_NATIVE__)
        if (!name || strncmp(name, "__EXT_NATIVE__", 13) != 0) continue;

        // Add only if subclass doesn't already have it
        if (!CFSetContainsValue(owned, sel)) {
            IMP imp = method_getImplementation(baseList[i]);
            const char *types = method_getTypeEncoding(baseList[i]);
            if (class_addMethod(subclass, sel, imp, types)) {
                CFSetAddValue(owned, sel);
            }
        }
    }

    if (subList) free(subList);
    if (baseList) free(baseList);
    if (owned) CFRelease(owned);
}

@implementation RustySDFInternal

+ (void)load
{
    // Find all loaded classes
    int num = objc_getClassList(NULL, 0);
    if (num <= 0) return;

    Class *classes = (Class *)malloc(sizeof(Class) * (unsigned)num);
    num = objc_getClassList(classes, num);

    Class base = [RustySDFInternal class];

    for (int i = 0; i < num; ++i) {
        Class cls = classes[i];
        if (cls == base) continue;

        // We only care about direct or indirect subclasses
        if (GMIsSubclassOf(cls, base)) {
            GMInjectSelectorsIntoSubclass(cls, base);
        }
    }

    free(classes);
}

- (double)__EXT_NATIVE__rusty_sdf_load_font:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_load_font(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_free_font:(double)font_handle
{
    return __EXT_NATIVE__rusty_sdf_free_font(font_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_add_fallback:(double)font_handle arg1:(double)fallback_handle
{
    return __EXT_NATIVE__rusty_sdf_add_fallback(font_handle, fallback_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_get_font_glyph_count:(double)font_handle
{
    return __EXT_NATIVE__rusty_sdf_get_font_glyph_count(font_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_shape_text:(double)font_handle arg1:(char*)text arg2:(double)font_size
{
    return __EXT_NATIVE__rusty_sdf_shape_text(font_handle, text, font_size);
}
- (double)__EXT_NATIVE__rusty_sdf_free_shape:(double)shape_handle
{
    return __EXT_NATIVE__rusty_sdf_free_shape(shape_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_get_shape_width:(double)shape_handle
{
    return __EXT_NATIVE__rusty_sdf_get_shape_width(shape_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_get_shape_height:(double)shape_handle
{
    return __EXT_NATIVE__rusty_sdf_get_shape_height(shape_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_get_shape_glyph_count:(double)shape_handle
{
    return __EXT_NATIVE__rusty_sdf_get_shape_glyph_count(shape_handle);
}
- (double)__EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer:(double)shape_handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_get_shape_glyphs_buffer(shape_handle, buffer_ptr, buffer_len);
}
- (double)__EXT_NATIVE__rusty_sdf_set_bidi_mode:(double)mode
{
    return __EXT_NATIVE__rusty_sdf_set_bidi_mode(mode);
}
- (double)__EXT_NATIVE__rusty_sdf_set_buffer:(char*)buffer_ptr arg1:(double)buf_w arg2:(double)buf_h
{
    return __EXT_NATIVE__rusty_sdf_set_buffer(buffer_ptr, buf_w, buf_h);
}
- (double)__EXT_NATIVE__rusty_sdf_set_params:(double)padding arg1:(double)spread
{
    return __EXT_NATIVE__rusty_sdf_set_params(padding, spread);
}
- (double)__EXT_NATIVE__rusty_sdf_set_mode:(double)mode
{
    return __EXT_NATIVE__rusty_sdf_set_mode(mode);
}
- (double)__EXT_NATIVE__rusty_sdf_get_mode
{
    return __EXT_NATIVE__rusty_sdf_get_mode();
}
- (double)__EXT_NATIVE__rusty_sdf_get_buffer_bpp
{
    return __EXT_NATIVE__rusty_sdf_get_buffer_bpp();
}
- (double)__EXT_NATIVE__rusty_sdf_get_glyph_bounds:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_get_glyph_bounds(__arg_buffer, __arg_buffer_length, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_render_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)font_size
{
    return __EXT_NATIVE__rusty_sdf_render_glyph(font_handle, glyph_id, font_size);
}
- (double)__EXT_NATIVE__rusty_sdf_render_char:(double)font_handle arg1:(double)char_code arg2:(double)font_size
{
    return __EXT_NATIVE__rusty_sdf_render_char(font_handle, char_code, font_size);
}
- (double)__EXT_NATIVE__rusty_sdf_request_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)font_size arg3:(double)padding arg4:(double)spread arg5:(double)mode
{
    return __EXT_NATIVE__rusty_sdf_request_glyph(font_handle, glyph_id, font_size, padding, spread, mode);
}
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph:(char*)__ret_buffer arg1:(double)__ret_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_poll_glyph(__ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph_pixels:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_poll_glyph_pixels(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_poll_glyph_pixels_strided(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_measure_text:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_measure_text(__arg_buffer, __arg_buffer_length, __ret_buffer, __ret_buffer_length);
}
- (char*)__EXT_NATIVE__rusty_sdf_ping
{
    return __EXT_NATIVE__rusty_sdf_ping();
}
- (char*)__EXT_NATIVE__rusty_sdf_get_last_error
{
    return __EXT_NATIVE__rusty_sdf_get_last_error();
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_init:(double)width arg1:(double)height arg2:(double)padding
{
    return __EXT_NATIVE__rusty_sdf_atlas_init(width, height, padding);
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_reset
{
    return __EXT_NATIVE__rusty_sdf_atlas_reset();
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_clear
{
    return __EXT_NATIVE__rusty_sdf_atlas_clear();
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_get_version
{
    return __EXT_NATIVE__rusty_sdf_atlas_get_version();
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_page_count
{
    return __EXT_NATIVE__rusty_sdf_atlas_page_count();
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_ensure_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)base_size arg3:(double)spread arg4:(double)mode arg5:(double)async_flag
{
    return __EXT_NATIVE__rusty_sdf_atlas_ensure_glyph(font_handle, glyph_id, base_size, spread, mode, async_flag);
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_lookup:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_atlas_lookup(__arg_buffer, __arg_buffer_length, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_commit_glyph:(double)font_handle arg1:(double)glyph_id arg2:(double)base_size arg3:(double)spread arg4:(double)width arg5:(double)height arg6:(double)raw_w arg7:(double)raw_h arg8:(double)x_min arg9:(double)y_max
{
    return __EXT_NATIVE__rusty_sdf_atlas_commit_glyph(font_handle, glyph_id, base_size, spread, width, height, raw_w, raw_h, x_min, y_max);
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_meta(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__rusty_sdf_atlas_poll_dirty_pixels(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_create
{
    return __EXT_NATIVE__rusty_sdf_rich_create();
}
- (double)__EXT_NATIVE__rusty_sdf_rich_free:(double)handle
{
    return __EXT_NATIVE__rusty_sdf_rich_free(handle);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_text:(double)handle arg1:(char*)text
{
    return __EXT_NATIVE__rusty_sdf_rich_set_text(handle, text);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_font:(double)handle arg1:(double)font_handle arg2:(double)font_size arg3:(double)base_size arg4:(double)spread
{
    return __EXT_NATIVE__rusty_sdf_rich_set_font(handle, font_handle, font_size, base_size, spread);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_layout:(double)handle arg1:(double)max_width arg2:(double)line_height arg3:(double)letter_spacing arg4:(double)halign arg5:(double)valign
{
    return __EXT_NATIVE__rusty_sdf_rich_set_layout(handle, max_width, line_height, letter_spacing, halign, valign);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_default_style:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_set_default_style(handle, buffer_ptr, buffer_len);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_async:(double)handle arg1:(double)enabled
{
    return __EXT_NATIVE__rusty_sdf_rich_set_async(handle, enabled);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_plain:(double)handle arg1:(double)enabled
{
    return __EXT_NATIVE__rusty_sdf_rich_set_plain(handle, enabled);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_config:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_set_config(handle, buffer_ptr, buffer_len);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_register_image:(double)handle arg1:(char*)name
{
    return __EXT_NATIVE__rusty_sdf_rich_register_image(handle, name);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_set_image_metrics:(double)handle arg1:(double)spr_w arg2:(double)spr_h arg3:(double)xoff arg4:(double)yoff
{
    return __EXT_NATIVE__rusty_sdf_rich_set_image_metrics(handle, spr_w, spr_h, xoff, yoff);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_clear_images:(double)handle
{
    return __EXT_NATIVE__rusty_sdf_rich_clear_images(handle);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_build:(double)handle
{
    return __EXT_NATIVE__rusty_sdf_rich_build(handle);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_get_metrics_buffer(handle, buffer_ptr, buffer_len);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_get_page_byte_size:(double)handle arg1:(double)page
{
    return __EXT_NATIVE__rusty_sdf_rich_get_page_byte_size(handle, page);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_write_page_vertices:(double)handle arg1:(double)page arg2:(char*)buffer_ptr arg3:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_write_page_vertices(handle, page, buffer_ptr, buffer_len);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_get_images_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_get_images_buffer(handle, buffer_ptr, buffer_len);
}
- (char*)__EXT_NATIVE__rusty_sdf_rich_get_image_name:(double)handle arg1:(double)index
{
    return __EXT_NATIVE__rusty_sdf_rich_get_image_name(handle, index);
}
- (double)__EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer:(double)handle arg1:(char*)buffer_ptr arg2:(double)buffer_len
{
    return __EXT_NATIVE__rusty_sdf_rich_get_glyph_meta_buffer(handle, buffer_ptr, buffer_len);
}
- (char*)__EXT_NATIVE__rusty_sdf_rich_get_plain_text:(double)handle
{
    return __EXT_NATIVE__rusty_sdf_rich_get_plain_text(handle);
}
- (double)__EXT_NATIVE__RustySDF_queue_buffer:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__RustySDF_queue_buffer(__arg_buffer, __arg_buffer_length);
}
@end

