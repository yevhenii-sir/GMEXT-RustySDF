use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

use fdsm_ttf_parser::ttf_parser::Face;

use crate::error::set_last_error;

pub type Handle = u64;

static NEXT_FONT_HANDLE: LazyLock<Mutex<Handle>> = LazyLock::new(|| Mutex::new(1));

pub static FONT_HANDLES: LazyLock<Mutex<HashMap<Handle, Arc<FontState>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct FontState {
    pub font_data: Arc<Vec<u8>>,
    pub fallbacks: RwLock<Vec<Handle>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedGlyph {
    pub font_handle: Handle,
    pub glyph_id: u32,
}

fn alloc_font_handle() -> Handle {
    let mut next = NEXT_FONT_HANDLE.lock().unwrap_or_else(|e| e.into_inner());
    let h = *next;
    *next += 1;
    h
}

/// Load a font from raw bytes. Returns a handle on success, 0 on error.
pub fn load_font(data: Vec<u8>) -> Handle {
    let font_data = Arc::new(data);

    // Validate that the font can be parsed (zero-allocation check via ttf-parser).
    if Face::parse(&font_data[..], 0).is_err() {
        set_last_error("failed to parse font data");
        return 0;
    }

    let handle = alloc_font_handle();
    let state = FontState {
        font_data,
        fallbacks: RwLock::new(Vec::new()),
    };

    FONT_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(handle, Arc::new(state));
    handle
}

/// Free a font and all associated data.
pub fn free_font(handle: Handle) -> bool {
    crate::shaper::invalidate_face_cache(handle);
    crate::sdf_renderer::invalidate_ttf_face_cache(handle);
    FONT_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&handle)
        .is_some()
}

/// Add a fallback font to a font's fallback chain.
pub fn add_fallback(font_handle: Handle, fallback_handle: Handle) -> bool {
    let fonts = FONT_HANDLES.lock().unwrap_or_else(|e| e.into_inner());

    if !fonts.contains_key(&fallback_handle) {
        set_last_error("invalid fallback font handle");
        return false;
    }

    let font = match fonts.get(&font_handle) {
        Some(f) => f,
        None => {
            set_last_error("invalid font handle");
            return false;
        }
    };

    font.fallbacks
        .write()
        .unwrap_or_else(|e| e.into_inner())
        .push(fallback_handle);
    true
}

/// Get the number of glyphs in a font.
pub fn get_font_glyph_count(handle: Handle) -> Option<u32> {
    // Prefer cached rustybuzz face (no re-parse).
    if let Some(n) = crate::shaper::cached_glyph_count(handle) {
        return Some(n);
    }
    let fonts = FONT_HANDLES.lock().unwrap_or_else(|e| e.into_inner());
    let font = fonts.get(&handle)?;
    let face = Face::parse(&font.font_data, 0).ok()?;
    Some(face.number_of_glyphs() as u32)
}

/// Look up a font by handle.
pub fn get_font(handle: Handle) -> Option<Arc<FontState>> {
    FONT_HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .get(&handle)
        .cloned()
}

/// Resolve a character to the first available glyph in primary font + fallback chain.
/// Uses the shared rustybuzz face cache (no `Face::parse` per character).
pub fn resolve_char_with_fallback(font_handle: Handle, ch: char) -> Option<ResolvedGlyph> {
    let chain = {
        let fonts = FONT_HANDLES.lock().unwrap_or_else(|e| e.into_inner());
        let primary = fonts.get(&font_handle)?;
        let fallbacks = primary
            .fallbacks
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        let mut chain = Vec::with_capacity(1 + fallbacks.len());
        chain.push(font_handle);
        chain.extend(fallbacks);
        chain
    };

    for h in chain {
        if let Some(glyph_id) = crate::shaper::cached_glyph_index(h, ch) {
            // Skip .notdef (0) — keep searching fallbacks.
            if glyph_id != 0 {
                return Some(ResolvedGlyph {
                    font_handle: h,
                    glyph_id,
                });
            }
        }
    }

    None
}
