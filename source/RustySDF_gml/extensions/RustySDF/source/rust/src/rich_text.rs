//! Rich text handle store for native build API.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::font_manager::Handle;
use crate::rich_layout::{self, BuildResult, ImageInfo};
use crate::rich_parse::RichStyle;

static NEXT: LazyLock<Mutex<Handle>> = LazyLock::new(|| Mutex::new(1));
static HANDLES: LazyLock<Mutex<HashMap<Handle, RichTextState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
pub struct RichTextState {
    pub text: String,
    pub font_handle: Handle,
    pub font_size: f64,
    pub base_size: f64,
    pub spread: f64,
    pub max_width: f32,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub halign: i32,
    pub valign: i32,
    pub default_style: RichStyle,
    pub images: HashMap<String, ImageInfo>,
    pub async_enabled: bool,
    pub plain_mode: bool,
    pub built: Option<BuildResult>,
}

impl Default for RichTextState {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_handle: 0,
            font_size: 32.0,
            base_size: 48.0,
            spread: 8.0,
            max_width: 0.0,
            line_height: 0.0,
            letter_spacing: 0.0,
            halign: 0,
            valign: 0,
            default_style: RichStyle::default(),
            images: HashMap::new(),
            async_enabled: true,
            plain_mode: false,
            built: None,
        }
    }
}

fn alloc() -> Handle {
    let mut n = NEXT.lock().unwrap_or_else(|e| e.into_inner());
    let h = *n;
    *n += 1;
    h
}

pub fn create() -> Handle {
    let h = alloc();
    HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(h, RichTextState::default());
    h
}

pub fn free(h: Handle) -> bool {
    HANDLES
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&h)
        .is_some()
}

pub fn with_mut<R>(h: Handle, f: impl FnOnce(&mut RichTextState) -> R) -> Option<R> {
    let mut map = HANDLES.lock().unwrap_or_else(|e| e.into_inner());
    map.get_mut(&h).map(f)
}

pub fn with_ref<R>(h: Handle, f: impl FnOnce(&RichTextState) -> R) -> Option<R> {
    let map = HANDLES.lock().unwrap_or_else(|e| e.into_inner());
    map.get(&h).map(f)
}

pub fn build(h: Handle) -> Option<(i32, u64)> {
    with_mut(h, |st| {
        // Atlas-only refresh: keep shaped/wrapped layout, re-ensure glyphs + rewrite verts.
        if let Some(prev) = st.built.take() {
            if let Some(cache) = prev.layout.as_ref() {
                let cur_ver = crate::atlas::get_version();
                if prev.atlas_version != cur_ver {
                    let built = rich_layout::refresh_layout(cache, st.async_enabled);
                    let pending = built.pending;
                    let ver = built.atlas_version;
                    st.built = Some(built);
                    return if pending { (0, ver) } else { (1, ver) };
                }
                // Atlas unchanged — keep previous build.
                let pending = prev.pending;
                let ver = prev.atlas_version;
                st.built = Some(prev);
                return if pending { (0, ver) } else { (1, ver) };
            }
        }

        let built = rich_layout::build_rich(
            &st.text,
            st.font_handle,
            st.font_size,
            st.base_size,
            st.spread,
            st.max_width,
            st.line_height,
            st.letter_spacing,
            st.halign,
            st.valign,
            &st.default_style,
            &st.images,
            st.async_enabled,
            st.plain_mode,
        );
        let pending = built.pending;
        let ver = built.atlas_version;
        st.built = Some(built);
        if pending {
            (0, ver)
        } else {
            (1, ver)
        }
    })
}
