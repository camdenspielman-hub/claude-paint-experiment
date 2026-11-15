//! How far the pass being painted has got: its strokes (or touches) planned
//! and painted so far, and when it began, so the easel can say while a long
//! chunk runs how much of its current pass is left and about how long that
//! will take.
//!
//! Counting only: nothing here changes what is painted. One pass at a time is
//! counted per process (the easel paints one canvas at a time); a pass on
//! another thread just restarts the count.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

static TOTAL: AtomicU64 = AtomicU64::new(0);
static DONE: AtomicU64 = AtomicU64::new(0);
/// When the pass began, µs after `EPOCH` (0: no pass yet).
static BEGAN: AtomicU64 = AtomicU64::new(0);
/// Passes begun since the process started.
static PASSES: AtomicU64 = AtomicU64::new(0);
static EPOCH: OnceLock<Instant> = OnceLock::new();

fn now_us() -> u64 {
    EPOCH.get_or_init(Instant::now).elapsed().as_micros() as u64 + 1
}

/// The current (or last) pass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pass {
    /// Its number since the process started (1 for the first).
    pub number: u64,
    /// Strokes or touches planned, and painted so far.
    pub total: u64,
    pub done: u64,
    /// How long it has run.
    pub elapsed: Duration,
}

impl Pass {
    /// The time left at the rate so far, once enough of it is painted to say
    /// (None: just begun, or done).
    pub fn eta(&self) -> Option<Duration> {
        if self.done == 0 || self.done >= self.total || self.elapsed < Duration::from_millis(500) {
            return None;
        }
        Some(self.elapsed.mul_f64((self.total - self.done) as f64 / self.done as f64))
    }
}

/// The pass being painted, or the last one (None: none yet).
pub fn current() -> Option<Pass> {
    let began = BEGAN.load(Ordering::Acquire);
    (began > 0).then(|| Pass {
        number: PASSES.load(Ordering::Relaxed),
        total: TOTAL.load(Ordering::Relaxed),
        done: DONE.load(Ordering::Relaxed).min(TOTAL.load(Ordering::Relaxed)),
        elapsed: Duration::from_micros(now_us().saturating_sub(began)),
    })
}

/// A pass of `total` strokes or touches begins.
pub(crate) fn begin(total: u64) {
    DONE.store(0, Ordering::Relaxed);
    TOTAL.store(total, Ordering::Relaxed);
    PASSES.fetch_add(1, Ordering::Relaxed);
    BEGAN.store(now_us(), Ordering::Release);
}

/// `n` more of them are painted.
pub(crate) fn advance(n: u64) {
    DONE.fetch_add(n, Ordering::Relaxed);
}
