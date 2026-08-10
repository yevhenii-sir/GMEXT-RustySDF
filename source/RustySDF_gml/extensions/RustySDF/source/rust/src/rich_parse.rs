//! BBCode-ish rich text parser matching GML `RustySDF_RichText.parse_text`.

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RichStyle {
    pub c: u32, // GM make_color_rgb packed
    pub a: f32,
    pub b: f32,
    pub oc: u32,
    pub oa: f32,
    pub ow: f32,
    pub gc: u32,
    pub ga: f32,
    pub gr: f32,
    pub ul: bool,
    pub st: bool,
}

impl Default for RichStyle {
    fn default() -> Self {
        Self {
            c: 0xFFFFFF, // c_white
            a: 1.0,
            b: 0.360,
            oc: 0x000000, // c_black
            oa: 1.0,
            ow: 0.0,
            gc: 0xFFFF00, // c_aqua in GM is BGR: R=0 G=255 B=255 → 0xFFFF00
            ga: 0.0,
            gr: 0.20,
            ul: false,
            st: false,
        }
    }
}

impl RichStyle {
    pub fn color_rgb(c: u32) -> (f32, f32, f32) {
        let r = (c & 0xFF) as f32 / 255.0;
        let g = ((c >> 8) & 0xFF) as f32 / 255.0;
        let b = ((c >> 16) & 0xFF) as f32 / 255.0;
        (r, g, b)
    }
}

#[derive(Clone, Debug)]
pub enum RichRun {
    Text {
        text: String,
        style: Arc<RichStyle>,
    },
    Image {
        name: String,
        subimg: f32,
        sc_mult: f32,
        y_off: f32,
        tint: f32,
        style: Arc<RichStyle>,
    },
}

fn hex_to_color(hex: &str) -> u32 {
    let h = hex.trim().trim_start_matches('#');
    if h.len() < 6 {
        return 0xFFFFFF;
    }
    let r = u32::from_str_radix(&h[0..2], 16).unwrap_or(255);
    let g = u32::from_str_radix(&h[2..4], 16).unwrap_or(255);
    let b = u32::from_str_radix(&h[4..6], 16).unwrap_or(255);
    r | (g << 8) | (b << 16)
}

fn commit_style(style: &RichStyle) -> Arc<RichStyle> {
    Arc::new(style.clone())
}

/// Parse one paragraph (no `\n` handling — caller splits).
pub fn parse_paragraph(text: &str, default_style: &RichStyle) -> Vec<RichRun> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut runs = Vec::new();
    let mut cur_text = String::new();
    let mut cur_style = default_style.clone();
    let mut style_stack: Vec<RichStyle> = Vec::new();

    let mut i = 0usize;
    while i < len {
        if chars[i] == '[' {
            if let Some(close_rel) = chars[i + 1..].iter().position(|&c| c == ']') {
                let close_idx = i + 1 + close_rel;
                let tag_full: String = chars[i + 1..close_idx].iter().collect();
                let is_close = tag_full.starts_with('/');

                if is_close {
                    if !cur_text.is_empty() {
                        runs.push(RichRun::Text {
                            text: std::mem::take(&mut cur_text),
                            style: commit_style(&cur_style),
                        });
                    }
                    if let Some(prev) = style_stack.pop() {
                        cur_style = prev;
                    }
                } else if let Some(eq) = tag_full.find('=') {
                    let tag_name = &tag_full[..eq];
                    let tag_val = &tag_full[eq + 1..];

                    if tag_name == "img" {
                        if !cur_text.is_empty() {
                            runs.push(RichRun::Text {
                                text: std::mem::take(&mut cur_text),
                                style: commit_style(&cur_style),
                            });
                        }
                        let args: Vec<&str> = tag_val.split(',').collect();
                        let name = args.first().copied().unwrap_or("").trim().to_string();
                        if !name.is_empty() {
                            runs.push(RichRun::Image {
                                name,
                                subimg: args.get(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0.0),
                                sc_mult: args.get(2).and_then(|s| s.trim().parse().ok()).unwrap_or(1.0),
                                y_off: args.get(3).and_then(|s| s.trim().parse().ok()).unwrap_or(0.0),
                                tint: args.get(4).and_then(|s| s.trim().parse().ok()).unwrap_or(0.0),
                                style: commit_style(&cur_style),
                            });
                        }
                        i = close_idx + 1;
                        continue;
                    }

                    if !cur_text.is_empty() {
                        runs.push(RichRun::Text {
                            text: std::mem::take(&mut cur_text),
                            style: commit_style(&cur_style),
                        });
                    }
                    style_stack.push(cur_style.clone());
                    match tag_name {
                        "c" => cur_style.c = hex_to_color(tag_val),
                        "a" => cur_style.a = tag_val.trim().parse().unwrap_or(cur_style.a),
                        "b" => cur_style.b = tag_val.trim().parse().unwrap_or(cur_style.b),
                        "oc" => cur_style.oc = hex_to_color(tag_val),
                        "oa" => cur_style.oa = tag_val.trim().parse().unwrap_or(cur_style.oa),
                        "ow" => cur_style.ow = tag_val.trim().parse().unwrap_or(cur_style.ow),
                        "gc" => cur_style.gc = hex_to_color(tag_val),
                        "ga" => cur_style.ga = tag_val.trim().parse().unwrap_or(cur_style.ga),
                        "gr" => cur_style.gr = tag_val.trim().parse().unwrap_or(cur_style.gr),
                        _ => {}
                    }
                } else {
                    if !cur_text.is_empty() {
                        runs.push(RichRun::Text {
                            text: std::mem::take(&mut cur_text),
                            style: commit_style(&cur_style),
                        });
                    }
                    style_stack.push(cur_style.clone());
                    match tag_full.as_str() {
                        "ul" => cur_style.ul = true,
                        "st" => cur_style.st = true,
                        _ => {}
                    }
                }
                i = close_idx + 1;
                continue;
            }
        }
        cur_text.push(chars[i]);
        i += 1;
    }
    if !cur_text.is_empty() {
        runs.push(RichRun::Text {
            text: cur_text,
            style: commit_style(&cur_style),
        });
    }
    runs
}
