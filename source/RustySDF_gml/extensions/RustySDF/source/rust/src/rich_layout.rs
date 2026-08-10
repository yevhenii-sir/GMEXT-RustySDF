//! Rich text layout + vertex assembly.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use crate::atlas::{self, AtlasEntry};
use crate::async_renderer::request_glyph_async;
use crate::font_manager::Handle;
use crate::rich_parse::{parse_paragraph, RichRun, RichStyle};
use crate::rich_vertex::{
    push_plain_quad, push_quad, PLAIN_VERTEX_STRIDE, PLAIN_WHITE, RICH_VERTEX_STRIDE,
};
use crate::sdf_renderer::get_render_mode;
use crate::shaper::{shape_text_result, GlyphInfo};

#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub width: f32,
    pub height: f32,
    pub xoffset: f32,
    pub yoffset: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct VisualGlyph {
    is_img: bool,
    glyph: GlyphInfo,
    style: Arc<RichStyle>,
    entry: Option<AtlasEntry>,
    // image fields
    img_name: String,
    subimg: f32,
    scale: f32,
    tint: f32,
    spr_w: f32,
    spr_h: f32,
    xoff: f32,
    yoff: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct VisualLine {
    start_idx: usize,
    end_idx: isize, // inclusive; -1 = empty
    w: f32,
}

#[derive(Clone, Debug)]
pub struct ImageDraw {
    pub name: String,
    pub sub: f32,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub tint: f32,
    pub style: RichStyle,
}

#[derive(Clone, Debug)]
pub struct GlyphMeta {
    pub is_img: bool,
    pub page: i32,
    pub offset: i32,
    pub img_index: i32,
    pub is_sdf: f32,
    pub style: RichStyle,
}

/// Microseconds spent inside `build_rich` (for GML/native profiling).
#[derive(Clone, Debug, Default)]
pub struct BuildTimingUs {
    pub total: u64,
    pub parse: u64,
    pub shape: u64,
    pub ensure: u64,
    pub wrap: u64,
    pub layout: u64,
    pub verts: u64,
    pub refresh: u64,
}

#[derive(Clone, Debug)]
pub struct CachedLayout {
    pub(crate) glyphs: Vec<VisualGlyph>, // entries may be stale; refresh clears/re-ensures
    pub(crate) lines: Vec<VisualLine>,
    pub font_size: f64,
    pub base_size: f64,
    pub spread: f64,
    pub letter_spacing: f32,
    pub lh: f32,
    pub scale: f32,
    pub pad: u32,
    pub halign: i32,
    pub valign: i32,
    pub plain_mode: bool,
}

#[derive(Clone, Debug, Default)]
pub struct BuildResult {
    pub total_w: f32,
    pub total_h: f32,
    pub page_buffers: Vec<Vec<u8>>,
    pub images: Vec<ImageDraw>,
    pub glyph_meta: Vec<GlyphMeta>,
    pub plain_text: String,
    pub atlas_version: u64,
    pub pending: bool,
    pub vertex_stride: usize,
    pub last_line_w: f32,
    pub visual_line_count: u32,
    pub timing: BuildTimingUs,
    pub layout: Option<CachedLayout>,
}

fn store_cached_layout(
    visual_glyphs: &[VisualGlyph],
    visual_lines: &[VisualLine],
    font_size: f64,
    base_size: f64,
    spread: f64,
    letter_spacing: f32,
    lh: f32,
    scale: f32,
    pad: u32,
    halign: i32,
    valign: i32,
    plain_mode: bool,
) -> CachedLayout {
    CachedLayout {
        glyphs: visual_glyphs
            .iter()
            .map(|vg| {
                let mut g = vg.clone();
                g.entry = None;
                g
            })
            .collect(),
        lines: visual_lines.to_vec(),
        font_size,
        base_size,
        spread,
        letter_spacing,
        lh,
        scale,
        pad,
        halign,
        valign,
        plain_mode,
    }
}

pub fn build_rich(
    text: &str,
    font_handle: Handle,
    font_size: f64,
    base_size: f64,
    spread: f64,
    max_width: f32,
    line_height: f32,
    letter_spacing: f32,
    halign: i32, // 0 left, 1 center, 2 right
    valign: i32, // 0 top, 1 middle, 2 bottom
    default_style: &RichStyle,
    images: &HashMap<String, ImageInfo>,
    async_enabled: bool,
    plain_mode: bool,
) -> BuildResult {
    let mut result = BuildResult {
        vertex_stride: if plain_mode {
            PLAIN_VERTEX_STRIDE
        } else {
            RICH_VERTEX_STRIDE
        },
        ..BuildResult::default()
    };
    if font_handle == 0 || text.is_empty() {
        return result;
    }

    let t_total = Instant::now();
    let mut us_parse: u64 = 0;
    let mut us_shape: u64 = 0;
    let mut us_ensure: u64 = 0;
    let mut us_wrap: u64 = 0;

    let scale = (font_size / base_size.max(0.0001)) as f32;
    let lh = if line_height > 0.0 {
        line_height
    } else {
        font_size as f32 * 1.2
    };
    let mode = get_render_mode();
    let pad = atlas::get_padding().max((spread.round() as u32).saturating_add(1));

    let mut visual_glyphs: Vec<VisualGlyph> = Vec::new();
    let mut visual_lines: Vec<VisualLine> = Vec::new();
    let mut pending = false;

    let paragraphs: Vec<&str> = text.split('\n').collect();
    for (p_i, paragraph_str) in paragraphs.iter().enumerate() {
        if paragraph_str.is_empty() {
            visual_lines.push(VisualLine {
                start_idx: 0,
                end_idx: -1,
                w: 0.0,
            });
            result.plain_text.push('\n');
            continue;
        }

        let t_parse = Instant::now();
        let runs = if plain_mode {
            vec![RichRun::Text {
                text: paragraph_str.to_string(),
                style: Arc::new(default_style.clone()),
            }]
        } else {
            parse_paragraph(paragraph_str, default_style)
        };
        us_parse += t_parse.elapsed().as_micros() as u64;
        let paragraph_v_start = visual_glyphs.len();

        for run in runs {
            match run {
                RichRun::Image {
                    name,
                    subimg,
                    sc_mult,
                    y_off,
                    tint,
                    style,
                } => {
                    let info = images.get(&name);
                    let (spr_w, spr_h, xoff, yoff) = match info {
                        Some(i) => (i.width, i.height, i.xoffset, i.yoffset),
                        None => (0.0, 1.0, 0.0, 0.0),
                    };
                    let final_scale = if spr_h > 0.0 {
                        (font_size as f32 / spr_h) * sc_mult
                    } else {
                        sc_mult
                    };
                    visual_glyphs.push(VisualGlyph {
                        is_img: true,
                        glyph: GlyphInfo {
                            font_handle,
                            glyph_id: 0,
                            x_offset: 0.0,
                            y_offset: y_off,
                            x_advance: spr_w * final_scale,
                            y_advance: 0.0,
                            cluster: 0,
                            char_code: b' ' as u32,
                        },
                        style,
                        entry: None,
                        img_name: name,
                        subimg,
                        scale: final_scale,
                        tint,
                        spr_w,
                        spr_h,
                        xoff,
                        yoff,
                    });
                }
                RichRun::Text { text: run_text, style } => {
                    let t_shape = Instant::now();
                    let shape = shape_text_result(font_handle, &run_text, font_size);
                    us_shape += t_shape.elapsed().as_micros() as u64;

                    let t_ensure = Instant::now();
                    for g in &shape.glyphs {
                        let entry = ensure_glyph(
                            g.font_handle,
                            g.glyph_id,
                            base_size,
                            spread,
                            mode,
                            async_enabled,
                            &mut pending,
                        );
                        visual_glyphs.push(VisualGlyph {
                            is_img: false,
                            glyph: g.clone(),
                            style: Arc::clone(&style),
                            entry,
                            img_name: String::new(),
                            subimg: 0.0,
                            scale: 1.0,
                            tint: 0.0,
                            spr_w: 0.0,
                            spr_h: 0.0,
                            xoff: 0.0,
                            yoff: 0.0,
                        });
                    }
                    us_ensure += t_ensure.elapsed().as_micros() as u64;
                }
            }
        }

        // word wrap
        let t_wrap = Instant::now();
        let mut cur_start = paragraph_v_start;
        let mut pen_x = 0.0f32;
        let mut last_space_idx: isize = -1;
        let mut last_space_w = 0.0f32;

        for i in paragraph_v_start..visual_glyphs.len() {
            let vg = &visual_glyphs[i];
            let adv = vg.glyph.x_advance + letter_spacing;

            if !vg.is_img {
                let raw_w = vg.entry.as_ref().map(|e| e.raw_w).unwrap_or(0);
                if raw_w == 0 {
                    last_space_idx = i as isize;
                    last_space_w = pen_x;
                }
            }

            if max_width > 0.0 && (pen_x + adv) > max_width && cur_start < i {
                if last_space_idx >= cur_start as isize {
                    visual_lines.push(VisualLine {
                        start_idx: cur_start,
                        end_idx: last_space_idx - 1,
                        w: last_space_w,
                    });
                    cur_start = (last_space_idx + 1) as usize;
                    pen_x = 0.0;
                    for j in cur_start..=i {
                        pen_x += visual_glyphs[j].glyph.x_advance + letter_spacing;
                    }
                } else {
                    visual_lines.push(VisualLine {
                        start_idx: cur_start,
                        end_idx: (i as isize) - 1,
                        w: pen_x,
                    });
                    cur_start = i;
                    pen_x = adv;
                }
            } else {
                pen_x += adv;
            }
        }
        if cur_start < visual_glyphs.len() {
            visual_lines.push(VisualLine {
                start_idx: cur_start,
                end_idx: (visual_glyphs.len() as isize) - 1,
                w: pen_x,
            });
        }
        us_wrap += t_wrap.elapsed().as_micros() as u64;

        if p_i < paragraphs.len() - 1 {
            result.plain_text.push('\n');
        }
    }

    let (us_layout, us_verts) = assemble_from_layout(
        &mut result,
        &visual_glyphs,
        &visual_lines,
        font_size,
        scale,
        lh,
        letter_spacing,
        pad,
        halign,
        valign,
        plain_mode,
    );

    result.atlas_version = atlas::get_version();
    result.pending = pending;
    result.visual_line_count = visual_lines.len() as u32;
    result.last_line_w = visual_lines.last().map(|l| l.w).unwrap_or(0.0);
    result.layout = Some(store_cached_layout(
        &visual_glyphs,
        &visual_lines,
        font_size,
        base_size,
        spread,
        letter_spacing,
        lh,
        scale,
        pad,
        halign,
        valign,
        plain_mode,
    ));
    result.timing = BuildTimingUs {
        total: t_total.elapsed().as_micros() as u64,
        parse: us_parse,
        shape: us_shape,
        ensure: us_ensure,
        wrap: us_wrap,
        layout: us_layout,
        verts: us_verts,
        refresh: 0,
    };
    result
}

/// Re-ensure atlas entries and re-emit verts from a cached layout (no re-parse/shape/wrap).
pub fn refresh_layout(cache: &CachedLayout, async_enabled: bool) -> BuildResult {
    let t_total = Instant::now();
    let mode = get_render_mode();
    let mut pending = false;
    let mut visual_glyphs = cache.glyphs.clone();

    let t_ensure = Instant::now();
    for vg in &mut visual_glyphs {
        if !vg.is_img {
            vg.entry = ensure_glyph(
                vg.glyph.font_handle,
                vg.glyph.glyph_id,
                cache.base_size,
                cache.spread,
                mode,
                async_enabled,
                &mut pending,
            );
        }
    }
    let us_ensure = t_ensure.elapsed().as_micros() as u64;

    let mut result = BuildResult {
        vertex_stride: if cache.plain_mode {
            PLAIN_VERTEX_STRIDE
        } else {
            RICH_VERTEX_STRIDE
        },
        ..BuildResult::default()
    };

    let (us_layout, us_verts) = assemble_from_layout(
        &mut result,
        &visual_glyphs,
        &cache.lines,
        cache.font_size,
        cache.scale,
        cache.lh,
        cache.letter_spacing,
        cache.pad,
        cache.halign,
        cache.valign,
        cache.plain_mode,
    );

    result.atlas_version = atlas::get_version();
    result.pending = pending;
    result.visual_line_count = cache.lines.len() as u32;
    result.last_line_w = cache.lines.last().map(|l| l.w).unwrap_or(0.0);
    result.layout = Some(store_cached_layout(
        &visual_glyphs,
        &cache.lines,
        cache.font_size,
        cache.base_size,
        cache.spread,
        cache.letter_spacing,
        cache.lh,
        cache.scale,
        cache.pad,
        cache.halign,
        cache.valign,
        cache.plain_mode,
    ));

    let total = t_total.elapsed().as_micros() as u64;
    result.timing = BuildTimingUs {
        total,
        parse: 0,
        shape: 0,
        ensure: us_ensure,
        wrap: 0,
        layout: us_layout,
        verts: us_verts,
        refresh: total,
    };
    result
}

/// Metrics + origin + page buffer emission. Returns `(layout_us, verts_us)`.
fn assemble_from_layout(
    result: &mut BuildResult,
    visual_glyphs: &[VisualGlyph],
    visual_lines: &[VisualLine],
    font_size: f64,
    scale: f32,
    lh: f32,
    letter_spacing: f32,
    pad: u32,
    halign: i32,
    valign: i32,
    plain_mode: bool,
) -> (u64, u64) {
    let t_layout = Instant::now();
    let p_pad = pad as f32;
    let mut first_line_ascent = 0.0f32;
    let mut last_line_descent = 0.0f32;
    let mut total_text_width = 0.0f32;
    let line_count = visual_lines.len();

    for (l, line) in visual_lines.iter().enumerate() {
        total_text_width = total_text_width.max(line.w);
        if line.start_idx as isize > line.end_idx {
            continue;
        }
        for i in line.start_idx..=(line.end_idx as usize) {
            let vg = &visual_glyphs[i];
            if vg.is_img {
                let spr_h = vg.spr_h * vg.scale;
                let visual_center_up = font_size as f32 * 0.35;
                let ga = visual_center_up + (spr_h / 2.0) - vg.glyph.y_offset;
                let gd = (spr_h / 2.0) - visual_center_up + vg.glyph.y_offset;
                if l == 0 {
                    first_line_ascent = first_line_ascent.max(ga);
                }
                if l == line_count - 1 {
                    last_line_descent = last_line_descent.max(gd);
                }
                continue;
            }
            if let Some(entry) = &vg.entry {
                if entry.raw_w > 0 {
                    let g = &vg.glyph;
                    let ga = g.y_offset + (entry.y_max as f32) * scale;
                    let gd = -g.y_offset - (entry.y_max as f32) * scale
                        + (entry.raw_h as f32 - p_pad * 2.0) * scale;
                    if l == 0 {
                        first_line_ascent = first_line_ascent.max(ga);
                    }
                    if l == line_count - 1 {
                        last_line_descent = last_line_descent.max(gd);
                    }
                }
            }
        }
    }

    let total_text_height =
        first_line_ascent + ((line_count as f32 - 1.0).max(0.0) * lh) + last_line_descent;

    let origin_x = match halign {
        1 => total_text_width / 2.0,
        2 => total_text_width,
        _ => 0.0,
    };
    let origin_y = match valign {
        1 => total_text_height / 2.0,
        2 => total_text_height,
        _ => 0.0,
    };
    let us_layout = t_layout.elapsed().as_micros() as u64;

    let t_verts = Instant::now();
    let mut page_buffers: Vec<Vec<u8>> = Vec::new();
    let mut current_y = first_line_ascent;

    for line in visual_lines {
        if line.start_idx as isize > line.end_idx {
            current_y += lh;
            continue;
        }

        let mut cur_x = match halign {
            1 => (total_text_width - line.w) / 2.0,
            2 => total_text_width - line.w,
            _ => 0.0,
        };

        let mut ul_segments: Vec<(f32, f32, Arc<RichStyle>)> = Vec::new();
        let mut st_segments: Vec<(f32, f32, Arc<RichStyle>)> = Vec::new();
        let mut seg_start_x = cur_x;
        let mut seg_style: Option<Arc<RichStyle>> = None;

        for i in line.start_idx..=(line.end_idx as usize) {
            let vg = &visual_glyphs[i];
            let g = &vg.glyph;

            if vg.is_img {
                let spr_h = vg.spr_h * vg.scale;
                let base_x = cur_x - origin_x;
                let visual_center_y = current_y - (font_size as f32 * 0.35);
                let base_y = visual_center_y - (spr_h / 2.0) + g.y_offset - origin_y;
                let img_index = result.images.len() as i32;
                result.images.push(ImageDraw {
                    name: vg.img_name.clone(),
                    sub: vg.subimg,
                    x: base_x + (vg.xoff * vg.scale),
                    y: base_y + (vg.yoff * vg.scale),
                    scale: vg.scale,
                    tint: vg.tint,
                    style: (*vg.style).clone(),
                });
                result.glyph_meta.push(GlyphMeta {
                    is_img: true,
                    page: -1,
                    offset: -1,
                    img_index,
                    is_sdf: 0.0,
                    style: (*vg.style).clone(),
                });
                result.plain_text.push(' ');
                cur_x += g.x_advance + letter_spacing;
                continue;
            }

            match &seg_style {
                None => {
                    seg_style = Some(Arc::clone(&vg.style));
                    seg_start_x = cur_x;
                }
                Some(ss)
                    if ss.c != vg.style.c || ss.ul != vg.style.ul || ss.st != vg.style.st =>
                {
                    if ss.ul {
                        ul_segments.push((seg_start_x, cur_x, Arc::clone(ss)));
                    }
                    if ss.st {
                        st_segments.push((seg_start_x, cur_x, Arc::clone(ss)));
                    }
                    seg_style = Some(Arc::clone(&vg.style));
                    seg_start_x = cur_x;
                }
                _ => {}
            }

            if let Some(entry) = &vg.entry {
                if entry.raw_w > 0 && !entry.async_pending {
                    let page = entry.page_index as usize;
                    while page_buffers.len() <= page {
                        page_buffers.push(Vec::new());
                    }
                    let offset = page_buffers[page].len() as i32;
                    result.glyph_meta.push(GlyphMeta {
                        is_img: false,
                        page: page as i32,
                        offset,
                        img_index: -1,
                        is_sdf: 1.0,
                        style: (*vg.style).clone(),
                    });
                    if let Some(ch) = char::from_u32(g.char_code) {
                        result.plain_text.push(ch);
                    } else {
                        result.plain_text.push(' ');
                    }

                    let x1 = (cur_x + g.x_offset + (entry.x_min as f32) * scale - p_pad * scale)
                        - origin_x;
                    let y1 = (current_y - g.y_offset - (entry.y_max as f32) * scale - p_pad * scale)
                        - origin_y;
                    let x2 = x1 + (entry.w as f32) * scale;
                    let y2 = y1 + (entry.h as f32) * scale;

                    if plain_mode {
                        push_plain_quad(
                            &mut page_buffers[page],
                            x1,
                            y1,
                            x2,
                            y2,
                            entry.u1,
                            entry.v1,
                            entry.u2,
                            entry.v2,
                            PLAIN_WHITE,
                        );
                    } else {
                        let (cr, cg, cb) = RichStyle::color_rgb(vg.style.c);
                        let (or, og, ob) = RichStyle::color_rgb(vg.style.oc);
                        let (gr, gg, gb) = RichStyle::color_rgb(vg.style.gc);

                        push_quad(
                            &mut page_buffers[page],
                            x1,
                            y1,
                            x2,
                            y2,
                            entry.u1,
                            entry.v1,
                            entry.u2,
                            entry.v2,
                            cr,
                            cg,
                            cb,
                            vg.style.a,
                            or,
                            og,
                            ob,
                            vg.style.oa,
                            gr,
                            gg,
                            gb,
                            vg.style.ga,
                            vg.style.b,
                            vg.style.ow,
                            vg.style.gr,
                            1.0,
                        );
                    }
                } else {
                    result.glyph_meta.push(GlyphMeta {
                        is_img: false,
                        page: -1,
                        offset: -1,
                        img_index: -1,
                        is_sdf: 1.0,
                        style: (*vg.style).clone(),
                    });
                    result.plain_text.push(' ');
                }
            } else {
                result.glyph_meta.push(GlyphMeta {
                    is_img: false,
                    page: -1,
                    offset: -1,
                    img_index: -1,
                    is_sdf: 1.0,
                    style: (*vg.style).clone(),
                });
                result.plain_text.push(' ');
            }
            cur_x += g.x_advance + letter_spacing;
        }

        if !plain_mode {
            if let Some(ss) = &seg_style {
                if ss.ul {
                    ul_segments.push((seg_start_x, cur_x, Arc::clone(ss)));
                }
                if ss.st {
                    st_segments.push((seg_start_x, cur_x, Arc::clone(ss)));
                }
            }

            if page_buffers.is_empty() {
                page_buffers.push(Vec::new());
            }
            let sb0 = &mut page_buffers[0];
            let ul_thick = (font_size as f32 * 0.1).max(1.0);
            let ul_y = current_y + font_size as f32 * 0.12 - origin_y;
            let st_thick = (font_size as f32 * 0.1).max(1.0);
            let st_y = current_y - font_size as f32 * 0.3 - origin_y;

            for (x1, x2, s) in &ul_segments {
                add_decor(sb0, *x1 - origin_x, *x2 - origin_x, ul_y, ul_thick, s);
            }
            for (x1, x2, s) in &st_segments {
                add_decor(sb0, *x1 - origin_x, *x2 - origin_x, st_y, st_thick, s);
            }
        }

        current_y += lh;
    }

    result.total_w = total_text_width;
    result.total_h = total_text_height;
    result.page_buffers = page_buffers;
    (us_layout, t_verts.elapsed().as_micros() as u64)
}

fn add_decor(buf: &mut Vec<u8>, x1: f32, x2: f32, y: f32, th: f32, s: &RichStyle) {
    let (cr, cg, cb) = RichStyle::color_rgb(s.c);
    let y2 = y + th;
    push_quad(
        buf, x1, y, x2, y2, 0.0, 0.0, 0.0, 0.0, cr, cg, cb, s.a, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
}

fn ensure_glyph(
    font_handle: Handle,
    glyph_id: u32,
    base_size: f64,
    spread: f64,
    mode: u32,
    async_enabled: bool,
    pending: &mut bool,
) -> Option<AtlasEntry> {
    if let Some(e) = atlas::lookup(font_handle, glyph_id, base_size, spread) {
        if e.async_pending {
            *pending = true;
            return None;
        }
        return Some(e);
    }

    if async_enabled {
        let pad = atlas::get_padding().max((spread.round() as u32).saturating_add(1));
        request_glyph_async(font_handle, glyph_id, base_size, pad, spread.round() as u32, mode);
        atlas::mark_pending(font_handle, glyph_id, base_size, spread);
        *pending = true;
        return None;
    }

    let code = atlas::ensure_glyph_sync(font_handle, glyph_id, base_size, spread, mode);
    if code < 0 {
        return None;
    }
    atlas::lookup(font_handle, glyph_id, base_size, spread)
}
