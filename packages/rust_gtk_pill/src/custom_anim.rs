// Custom pill animations loaded from PNG frame sequences:
//   ~/.config/voquill-pill/frames/          -> recording (speed follows voice level)
//   ~/.config/voquill-pill/frames-loading/  -> transcribing (constant speed; falls
//                                              back to a "breathing" first recording
//                                              frame when absent)
// When custom frames are present the capsule background is hidden by the caller,
// so the sprite floats on its own. No frames at all -> official waveform/spinner.
use std::cell::{Cell, RefCell};

use gtk::cairo;
use gtk::gdk::prelude::GdkContextExt;
use gtk::gdk_pixbuf::Pixbuf;

use crate::state::PillState;

const RECORDING_DIR: &str = ".config/voquill-pill/frames";
const LOADING_DIR: &str = ".config/voquill-pill/frames-loading";
// Frames advanced per 16ms draw tick: idle crawl .. full-voice dance.
const SPEED_MIN: f64 = 0.12;
const SPEED_MAX: f64 = 1.0;
const LOADING_SPEED: f64 = 0.25;
// Volume bounce: sprite grows up to this much at full voice level.
const BOUNCE_MAX: f64 = 0.15;
// Breathing fallback for loading: gentle 1.6s pulse of +/-4%.
const BREATH_SPEED: f64 = 0.06;
const BREATH_DEPTH: f64 = 0.04;

thread_local! {
    static REC_FRAMES: RefCell<Option<Vec<Pixbuf>>> = const { RefCell::new(None) };
    static LOAD_FRAMES: RefCell<Option<Vec<Pixbuf>>> = const { RefCell::new(None) };
    static FRAME_POS: Cell<f64> = const { Cell::new(0.0) };
    static SMOOTH_LEVEL: Cell<f64> = const { Cell::new(0.0) };
    static BREATH_PHASE: Cell<f64> = const { Cell::new(0.0) };
}

fn load_frames(dir: &str) -> Vec<Pixbuf> {
    let Some(home) = std::env::var_os("HOME") else {
        return Vec::new();
    };
    let dir = std::path::Path::new(&home).join(dir);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut paths: Vec<_> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|e| e.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
        })
        .collect();
    paths.sort();
    paths
        .iter()
        .filter_map(|p| Pixbuf::from_file(p).ok())
        .collect()
}

fn with_rec<R>(f: impl FnOnce(&Vec<Pixbuf>) -> R) -> R {
    REC_FRAMES.with(|c| f(c.borrow_mut().get_or_insert_with(|| load_frames(RECORDING_DIR))))
}

fn with_load<R>(f: impl FnOnce(&Vec<Pixbuf>) -> R) -> R {
    LOAD_FRAMES.with(|c| f(c.borrow_mut().get_or_insert_with(|| load_frames(LOADING_DIR))))
}

/// True when the recording animation will draw (used to hide the capsule).
pub(crate) fn has_recording_frames() -> bool {
    with_rec(|f| !f.is_empty())
}

/// Sprite placement: oversized and bottom-anchored so it "stands" where the
/// pill is, spilling above (the surrounding window has headroom for this).
/// `bounce` in [0,1] additionally scales the sprite from its bottom center.
fn paint_sprite(
    cr: &cairo::Context,
    frame: &Pixbuf,
    rx: f64,
    ry: f64,
    pill_w: f64,
    pill_h: f64,
    alpha: f64,
    bounce: f64,
) {
    let fw = frame.width() as f64;
    let fh = frame.height() as f64;
    if fw <= 0.0 || fh <= 0.0 {
        return;
    }
    let scale = ((pill_h * 1.9) / fh).min((pill_w * 0.9) / fw) * (1.0 + bounce);
    let dh = fh * scale;
    let dx = rx + (pill_w - fw * scale) / 2.0;
    let dy = ry + pill_h - dh - pill_h * 0.08;

    cr.save().ok();
    cr.translate(dx, dy);
    cr.scale(scale, scale);
    cr.set_source_pixbuf(frame, 0.0, 0.0);
    cr.paint_with_alpha(alpha).ok();
    cr.restore().ok();
}

/// Recording animation. Returns false when no custom frames are available so
/// the caller can fall back to the waveform.
pub(crate) fn draw_custom_anim(
    cr: &cairo::Context,
    rx: f64,
    ry: f64,
    pill_w: f64,
    pill_h: f64,
    expand_t: f64,
    state: &PillState,
) -> bool {
    with_rec(|frames| {
        if frames.is_empty() {
            return false;
        }

        let level = state.current_level.get().clamp(0.0, 1.0);
        // Louder voice -> faster playback.
        let speed = SPEED_MIN + (SPEED_MAX - SPEED_MIN) * level;
        let pos = FRAME_POS.with(|p| {
            let next = (p.get() + speed) % frames.len() as f64;
            p.set(next);
            next
        });
        // Louder voice -> bigger sprite; smoothed so it breathes, not jitters.
        let bounce = SMOOTH_LEVEL.with(|s| {
            let next = s.get() * 0.8 + level * 0.2;
            s.set(next);
            next * BOUNCE_MAX
        });

        paint_sprite(cr, &frames[pos as usize], rx, ry, pill_w, pill_h, expand_t, bounce);
        true
    })
}

/// Loading (transcribing) animation. Plays frames-loading when present,
/// otherwise breathes on the first recording frame. Returns false when
/// neither is available so the caller keeps the official spinner.
pub(crate) fn draw_loading_anim(
    cr: &cairo::Context,
    rx: f64,
    ry: f64,
    pill_w: f64,
    pill_h: f64,
    expand_t: f64,
) -> bool {
    let drew = with_load(|frames| {
        if frames.is_empty() {
            return false;
        }
        let pos = FRAME_POS.with(|p| {
            let next = (p.get() + LOADING_SPEED) % frames.len() as f64;
            p.set(next);
            next
        });
        paint_sprite(cr, &frames[pos as usize], rx, ry, pill_w, pill_h, expand_t, 0.0);
        true
    });
    if drew {
        return true;
    }
    with_rec(|frames| {
        let Some(first) = frames.first() else {
            return false;
        };
        let breath = BREATH_PHASE.with(|p| {
            let next = p.get() + BREATH_SPEED;
            p.set(next);
            next.sin() * BREATH_DEPTH
        });
        paint_sprite(cr, first, rx, ry, pill_w, pill_h, expand_t, breath);
        true
    })
}
