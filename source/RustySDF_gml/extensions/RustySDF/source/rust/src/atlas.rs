//! Multi-page shelf-packed SDF atlas (CPU side). GPU surfaces stay in GML.

use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex};

use crate::font_manager::Handle;
use crate::sdf_renderer::{
    clear_render_buffer, get_glyph_bounds, get_render_mode, render_glyph_sdf, set_render_buffer,
    set_render_mode, set_render_params, RenderMode,
};

static ATLAS: LazyLock<Mutex<AtlasState>> = LazyLock::new(|| Mutex::new(AtlasState::default()));

#[derive(Clone, Debug)]
pub struct AtlasEntry {
    pub page_index: u32,
    pub atlas_x: u32,
    pub atlas_y: u32,
    pub w: u32,
    pub h: u32,
    pub raw_w: u32,
    pub raw_h: u32,
    pub u1: f32,
    pub v1: f32,
    pub u2: f32,
    pub v2: f32,
    pub x_min: f64,
    pub y_max: f64,
    pub async_pending: bool,
}

#[derive(Clone, Debug)]
pub struct DirtyGlyph {
    pub page: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub pixels: Vec<u8>,
}

#[derive(Clone, Debug)]
struct Shelf {
    y: u32,
    h: u32,
    x: u32,
}

#[derive(Clone, Debug)]
struct Page {
    shelves: Vec<Shelf>,
}

struct AtlasState {
    width: u32,
    height: u32,
    padding: u32,
    pages: Vec<Page>,
    cache: HashMap<u64, AtlasEntry>,
    version: u64,
    dirty: VecDeque<DirtyGlyph>,
    /// Last polled dirty item kept for pixel fetch (like async TLS).
    last_dirty: Option<DirtyGlyph>,
}

impl Default for AtlasState {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
            padding: 4,
            pages: vec![Page { shelves: Vec::new() }],
            cache: HashMap::new(),
            version: 0,
            dirty: VecDeque::new(),
            last_dirty: None,
        }
    }
}

/// Bit-packed key matching GML `__RustySDF_AtlasKey`.
pub fn atlas_key(font_handle: Handle, glyph_id: u32, base_font_size: f64, spread: f64) -> u64 {
    let b = base_font_size.round() as u64;
    let s = spread.round() as u64;
    (font_handle as u64)
        .wrapping_mul(8_589_934_592)
        .wrapping_add((glyph_id as u64).wrapping_mul(131_072))
        .wrapping_add(b.wrapping_mul(256))
        .wrapping_add(s)
}

fn align4(v: u32) -> u32 {
    let rem = v % 4;
    if rem == 0 {
        v
    } else {
        v + (4 - rem)
    }
}

fn with_atlas<R>(f: impl FnOnce(&mut AtlasState) -> R) -> R {
    let mut guard = ATLAS.lock().unwrap_or_else(|e| e.into_inner());
    f(&mut guard)
}

pub fn init(width: u32, height: u32, padding: u32) {
    with_atlas(|a| {
        a.width = width.max(1);
        a.height = height.max(1);
        a.padding = padding;
        a.pages.clear();
        a.pages.push(Page {
            shelves: Vec::new(),
        });
        a.cache.clear();
        a.dirty.clear();
        a.last_dirty = None;
        a.version = a.version.wrapping_add(1);
    });
}

pub fn reset() {
    with_atlas(|a| {
        for page in &mut a.pages {
            page.shelves.clear();
        }
        if a.pages.is_empty() {
            a.pages.push(Page {
                shelves: Vec::new(),
            });
        }
        a.cache.clear();
        a.dirty.clear();
        a.last_dirty = None;
        a.version = a.version.wrapping_add(1);
    });
}

pub fn clear_pages() {
    with_atlas(|a| {
        a.pages.clear();
        a.pages.push(Page {
            shelves: Vec::new(),
        });
        a.cache.clear();
        a.dirty.clear();
        a.last_dirty = None;
        a.version = a.version.wrapping_add(1);
    });
}

pub fn get_version() -> u64 {
    with_atlas(|a| a.version)
}

pub fn page_count() -> u32 {
    with_atlas(|a| a.pages.len() as u32)
}

pub fn get_padding() -> u32 {
    with_atlas(|a| a.padding)
}

pub fn lookup(
    font_handle: Handle,
    glyph_id: u32,
    base_font_size: f64,
    spread: f64,
) -> Option<AtlasEntry> {
    let key = atlas_key(font_handle, glyph_id, base_font_size, spread);
    with_atlas(|a| a.cache.get(&key).cloned())
}

fn add_page(a: &mut AtlasState) -> u32 {
    a.pages.push(Page {
        shelves: Vec::new(),
    });
    (a.pages.len() - 1) as u32
}

fn pack_rect(a: &mut AtlasState, pack_w: u32, pack_h: u32) -> Option<(u32, u32, u32)> {
    if pack_w > a.width || pack_h > a.height {
        return None;
    }

    for (p, page) in a.pages.iter().enumerate() {
        for (i, sh) in page.shelves.iter().enumerate() {
            if sh.h >= pack_h && (a.width.saturating_sub(sh.x)) >= pack_w {
                let placed_x = sh.x;
                let placed_y = sh.y;
                a.pages[p].shelves[i].x = sh.x.saturating_add(pack_w);
                return Some((p as u32, placed_x, placed_y));
            }
        }
    }

    if a.pages.is_empty() {
        add_page(a);
    }

    let mut p = (a.pages.len() - 1) as u32;
    let bottom_y = a.pages[p as usize]
        .shelves
        .iter()
        .map(|sh| sh.y.saturating_add(sh.h))
        .max()
        .unwrap_or(0);

    let (page_idx, placed_y) = if bottom_y.saturating_add(pack_h) > a.height {
        let np = add_page(a);
        (np, 0u32)
    } else {
        (p, bottom_y)
    };

    p = page_idx;
    let pack_w = pack_w;
    a.pages[p as usize].shelves.push(Shelf {
        y: placed_y,
        h: pack_h,
        x: pack_w,
    });
    Some((p, 0, placed_y))
}

fn make_empty_entry(pending: bool) -> AtlasEntry {
    AtlasEntry {
        page_index: 0,
        atlas_x: 0,
        atlas_y: 0,
        w: 0,
        h: 0,
        raw_w: 0,
        raw_h: 0,
        u1: 0.0,
        v1: 0.0,
        u2: 0.0,
        v2: 0.0,
        x_min: 0.0,
        y_max: 0.0,
        async_pending: pending,
    }
}

fn insert_entry(a: &mut AtlasState, key: u64, entry: AtlasEntry) {
    a.cache.insert(key, entry);
}

/// Mark glyph as async-pending placeholder. Returns true if newly marked.
pub fn mark_pending(
    font_handle: Handle,
    glyph_id: u32,
    base_font_size: f64,
    spread: f64,
) -> bool {
    let key = atlas_key(font_handle, glyph_id, base_font_size, spread);
    with_atlas(|a| {
        if let Some(e) = a.cache.get(&key) {
            return e.async_pending;
        }
        insert_entry(
            a,
            key,
            make_empty_entry(true),
        );
        true
    })
}

/// Commit pixels from GML (async path). Packs into atlas; GML blits with its own pixel buffer.
/// Returns entry on success.
pub fn commit_glyph(
    font_handle: Handle,
    glyph_id: u32,
    base_font_size: f64,
    spread: f64,
    width: u32,
    height: u32,
    raw_w: u32,
    raw_h: u32,
    x_min: f64,
    y_max: f64,
) -> Option<AtlasEntry> {
    let key = atlas_key(font_handle, glyph_id, base_font_size, spread);
    with_atlas(|a| {
        if width == 0 || height == 0 {
            let entry = make_empty_entry(false);
            insert_entry(a, key, entry.clone());
            a.version = a.version.wrapping_add(1);
            return Some(entry);
        }
        if width > a.width || height > a.height {
            let entry = make_empty_entry(false);
            insert_entry(a, key, entry.clone());
            a.version = a.version.wrapping_add(1);
            return Some(entry);
        }

        let pack_w = width.saturating_add(1);
        let pack_h = height.saturating_add(1);
        let (page, px, py) = match pack_rect(a, pack_w, pack_h) {
            Some(v) => v,
            None => {
                let entry = make_empty_entry(false);
                insert_entry(a, key, entry.clone());
                a.version = a.version.wrapping_add(1);
                return Some(entry);
            }
        };

        let aw = a.width as f32;
        let ah = a.height as f32;
        let entry = AtlasEntry {
            page_index: page,
            atlas_x: px,
            atlas_y: py,
            w: width,
            h: height,
            raw_w,
            raw_h,
            u1: px as f32 / aw,
            v1: py as f32 / ah,
            u2: (px + width) as f32 / aw,
            v2: (py + height) as f32 / ah,
            x_min,
            y_max,
            async_pending: false,
        };
        insert_entry(a, key, entry.clone());
        a.version = a.version.wrapping_add(1);
        Some(entry)
    })
}

/// Sync: bounds → render into internal buffer → pack → enqueue dirty upload.
/// Returns: 1 = already ready, 2 = newly packed (dirty queued), 0 = empty/space, -1 = error.
pub fn ensure_glyph_sync(
    font_handle: Handle,
    glyph_id: u32,
    base_font_size: f64,
    spread: f64,
    mode: u32,
) -> i32 {
    let key = atlas_key(font_handle, glyph_id, base_font_size, spread);
    {
        let existing = with_atlas(|a| a.cache.get(&key).cloned());
        if let Some(e) = existing {
            if !e.async_pending {
                return 1;
            }
        }
    }

    let pad = with_atlas(|a| a.padding.max((spread.round() as u32).saturating_add(1)));

    let bounds = match get_glyph_bounds(font_handle, glyph_id, base_font_size) {
        Some(b) => b,
        None => return -1,
    };
    let (gw, gh, x_min, y_max) = bounds;

    if gw == 0 || gh == 0 {
        with_atlas(|a| {
            insert_entry(
                a,
                key,
                make_empty_entry(false),
            );
        });
        return 0;
    }

    let raw_w = gw.saturating_add(pad.saturating_mul(2));
    let raw_h = gh.saturating_add(pad.saturating_mul(2));
    let total_w = align4(raw_w);
    let total_h = align4(raw_h);

    let mut pixels = vec![0u8; (total_w as usize).saturating_mul(total_h as usize).saturating_mul(4)];
    let prev_mode = get_render_mode();
    set_render_mode(mode);
    set_render_params(pad, spread.round().max(0.0) as u32);
    set_render_buffer(pixels.as_mut_ptr(), total_w, total_h);
    let ok = render_glyph_sdf(font_handle, glyph_id, base_font_size);
    clear_render_buffer();
    set_render_mode(prev_mode);

    if !ok {
        return -1;
    }

    with_atlas(|a| {
        let pack_w = total_w.saturating_add(1);
        let pack_h = total_h.saturating_add(1);
        let (page, px, py) = match pack_rect(a, pack_w, pack_h) {
            Some(v) => v,
            None => {
                insert_entry(
                    a,
                    key,
                    make_empty_entry(false),
                );
                return 0;
            }
        };

        let aw = a.width as f32;
        let ah = a.height as f32;
        let entry = AtlasEntry {
            page_index: page,
            atlas_x: px,
            atlas_y: py,
            w: total_w,
            h: total_h,
            raw_w,
            raw_h,
            u1: px as f32 / aw,
            v1: py as f32 / ah,
            u2: (px + total_w) as f32 / aw,
            v2: (py + total_h) as f32 / ah,
            x_min,
            y_max,
            async_pending: false,
        };
        insert_entry(a, key, entry);
        a.dirty.push_back(DirtyGlyph {
            page,
            x: px,
            y: py,
            w: total_w,
            h: total_h,
            pixels,
        });
        // Don't bump version on every sync pack during a single rich_build —
        // GML flushes dirty then rebuilds only when async completes.
        // But existing GML bumps version on async only. Sync packs don't bump
        // atlas.version in original AtlasPackGlyph. Keep that behavior.
        2
    })
}

pub fn poll_dirty_meta() -> Option<(u32, u32, u32, u32, u32)> {
    with_atlas(|a| {
        let item = a.dirty.pop_front()?;
        let meta = (item.page, item.x, item.y, item.w, item.h);
        a.last_dirty = Some(item);
        Some(meta)
    })
}

pub fn poll_dirty_pixels(dst: &mut [u8]) -> Option<usize> {
    with_atlas(|a| {
        let item = a.last_dirty.take()?;
        let n = item.pixels.len().min(dst.len());
        dst[..n].copy_from_slice(&item.pixels[..n]);
        Some(n)
    })
}

pub fn entry_to_f64s(e: &AtlasEntry) -> [f64; 15] {
    [
        1.0, // found
        e.page_index as f64,
        e.atlas_x as f64,
        e.atlas_y as f64,
        e.w as f64,
        e.h as f64,
        e.raw_w as f64,
        e.raw_h as f64,
        e.u1 as f64,
        e.v1 as f64,
        e.u2 as f64,
        e.v2 as f64,
        e.x_min,
        e.y_max,
        if e.async_pending { 1.0 } else { 0.0 },
    ]
}

#[allow(dead_code)]
pub fn render_mode_from_u32(mode: u32) -> Option<RenderMode> {
    match mode {
        0 => Some(RenderMode::Sdf),
        1 => Some(RenderMode::Psdf),
        2 => Some(RenderMode::Msdf),
        3 => Some(RenderMode::Mtsdf),
        _ => None,
    }
}
