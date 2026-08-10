use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};

use crate::font_manager::Handle;
use crate::sdf_renderer::get_glyph_bounds;

// ─── Async glyph generation ─────────────────────────────────────────────────

/// A job sent to the worker thread pool.
#[derive(Debug, Clone)]
struct GlyphJob {
    font_handle: Handle,
    glyph_id: u32,
    font_size: f64,
    padding: u32,
    spread: u32,
    mode: u32, // 0=SDF, 1=PSDF, 2=MSDF, 3=MTSDF
}

/// Result returned from the worker thread.
pub struct GlyphResult {
    pub font_handle: Handle,
    pub glyph_id: u32,
    pub font_size: f64,
    pub padding: u32,
    pub spread: u32,
    pub width: u32,
    pub height: u32,
    pub raw_w: u32,
    pub raw_h: u32,
    pub x_min: f64,
    pub y_max: f64,
    pub pixels: Vec<u8>, // RGBA8
}

static JOB_SENDER: LazyLock<Mutex<Option<Sender<GlyphJob>>>> = LazyLock::new(|| Mutex::new(None));
static RESULT_RECEIVER: LazyLock<Mutex<Option<Receiver<GlyphResult>>>> =
    LazyLock::new(|| Mutex::new(None));

/// Initialize the async thread pool. Called automatically on first request.
pub fn init_async_pool(threads: usize) {
    let mut sender_guard = JOB_SENDER.lock().unwrap_or_else(|e| e.into_inner());
    if sender_guard.is_some() {
        return; // Already initialized
    }

    let (job_tx, job_rx) = channel::<GlyphJob>();
    let (result_tx, result_rx) = channel::<GlyphResult>();

    // Wrap receiver in Arc<Mutex> for thread-safe sharing
    let job_rx = Arc::new(Mutex::new(job_rx));

    // Spawn worker threads
    for _ in 0..threads.max(1) {
        let result_tx = result_tx.clone();
        let job_rx = Arc::clone(&job_rx);
        std::thread::spawn(move || {
            loop {
                let job = match job_rx.lock().unwrap_or_else(|e| e.into_inner()).recv() {
                    Ok(j) => j,
                    Err(_) => break, // Channel closed
                };

                let result = std::panic::catch_unwind(|| {
                    render_glyph_async(
                        job.font_handle,
                        job.glyph_id,
                        job.font_size,
                        job.padding,
                        job.spread,
                        job.mode,
                    )
                });

                match result {
                    Ok(Some(r)) => {
                        let _ = result_tx.send(r);
                    }
                    Ok(None) => {
                        let _ = result_tx.send(GlyphResult {
                            font_handle: job.font_handle,
                            glyph_id: job.glyph_id,
                            font_size: job.font_size,
                            padding: job.padding,
                            spread: job.spread,
                            width: 0,
                            height: 0,
                            raw_w: 0,
                            raw_h: 0,
                            x_min: 0.0,
                            y_max: 0.0,
                            pixels: vec![],
                        });
                    }
                    Err(_) => {
                        #[cfg(target_os = "android")]
                        log::error!("RustySDF: worker thread panicked during glyph render");

                        let _ = result_tx.send(GlyphResult {
                            font_handle: job.font_handle,
                            glyph_id: job.glyph_id,
                            font_size: job.font_size,
                            padding: job.padding,
                            spread: job.spread,
                            width: 0,
                            height: 0,
                            raw_w: 0,
                            raw_h: 0,
                            x_min: 0.0,
                            y_max: 0.0,
                            pixels: vec![],
                        });
                    }
                }
            }
        });
    }

    *sender_guard = Some(job_tx);
    *RESULT_RECEIVER.lock().unwrap_or_else(|e| e.into_inner()) = Some(result_rx);
}

/// Request async glyph generation. Returns true if queued.
pub fn request_glyph_async(
    font_handle: Handle,
    glyph_id: u32,
    font_size: f64,
    padding: u32,
    spread: u32,
    mode: u32,
) -> bool {
    init_async_pool(4);

    let sender = JOB_SENDER.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref tx) = *sender {
        let _ = tx.send(GlyphJob {
            font_handle,
            glyph_id,
            font_size,
            padding,
            spread,
            mode,
        });
        true
    } else {
        false
    }
}

/// Poll for a completed glyph result. Returns None if no results ready.
pub fn poll_glyph_result() -> Option<GlyphResult> {
    let receiver = RESULT_RECEIVER.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref rx) = *receiver {
        match rx.try_recv() {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    } else {
        None
    }
}

// ─── Internal async render ────────────────────────────────────────────────────

fn render_glyph_async(
    font_handle: Handle,
    glyph_id: u32,
    font_size: f64,
    padding: u32,
    spread: u32,
    mode: u32,
) -> Option<GlyphResult> {
    let bounds = match get_glyph_bounds(font_handle, glyph_id, font_size) {
        Some(b) => b,
        None => return None,
    };
    let (gw, gh, x_min, y_max) = bounds;

    if gw == 0 || gh == 0 {
        return Some(GlyphResult {
            font_handle,
            glyph_id,
            font_size,
            padding,
            spread,
            width: 0,
            height: 0,
            raw_w: 0,
            raw_h: 0,
            x_min: 0.0,
            y_max: 0.0,
            pixels: vec![],
        });
    }

    let raw_w = gw + padding * 2;
    let raw_h = gh + padding * 2;
    let align_w = raw_w + ((4 - (raw_w % 4)) % 4);
    let align_h = raw_h + ((4 - (raw_h % 4)) % 4);

    let buf_size = (align_w * align_h * 4) as usize;
    let mut buffer = vec![0u8; buf_size];

    {
        use crate::sdf_renderer::{
            clear_render_buffer, render_glyph_sdf, set_render_buffer, set_render_mode,
            set_render_params,
        };
        set_render_buffer(buffer.as_mut_ptr(), align_w, align_h);
        set_render_params(padding, spread);
        let _ = set_render_mode(mode);

        let ok = render_glyph_sdf(font_handle, glyph_id, font_size);
        clear_render_buffer();

        if !ok {
            return None;
        }
    }

    Some(GlyphResult {
        font_handle,
        glyph_id,
        font_size,
        padding,
        spread,
        width: align_w,
        height: align_h,
        raw_w,
        raw_h,
        x_min,
        y_max,
        pixels: buffer,
    })
}
