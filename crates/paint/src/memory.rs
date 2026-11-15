//! What a canvas holds in memory, buffer by buffer, and what a canvas of a
//! given size will need before it is made.
//!
//! Counting only: nothing here changes what is painted. `Canvas::memory`
//! walks the canvas's own buffers (their capacity, and the heap behind every
//! pixel's wet paint); `estimate` predicts the same for a canvas that doesn't
//! exist yet from its pixel count, so the easel can say what a painting will
//! cost before it starts one.

use crate::canvas::Canvas;
use crate::wet::{Latent, Layer};
use std::mem::size_of;

/// Bytes a vector's buffer holds.
fn vec_bytes<T>(v: &Vec<T>) -> usize {
    v.capacity() * size_of::<T>()
}

/// The heap behind one latent: its tube parts and its retained layers (each a
/// latent of its own).
fn latent_heap(l: &Latent) -> usize {
    vec_bytes(&l.parts) + vec_bytes(&l.layers) + l.layers.iter().map(layer_heap).sum::<usize>()
}

fn layer_heap(l: &Layer) -> usize {
    latent_heap(&l.lat)
}

/// A canvas's memory, by part.
#[derive(Clone, Debug, Default)]
pub struct Memory {
    /// Pixels of the buffer.
    pub pixels: usize,
    /// (part, bytes), largest first.
    pub parts: Vec<(&'static str, usize)>,
}

impl Memory {
    pub fn total(&self) -> usize {
        self.parts.iter().map(|p| p.1).sum()
    }

    /// One line: `412 MiB (2400×1600, 112 B/px): wet paint 300 MiB, …`.
    pub fn summary(&self) -> String {
        let total = self.total();
        let per = if self.pixels > 0 { total / self.pixels } else { 0 };
        let parts: Vec<String> = self.parts.iter().filter(|p| p.1 >= 1 << 20).map(|(n, b)| format!("{n} {}", mib(*b))).collect();
        format!("{} ({per} B/px){}{}", mib(total), if parts.is_empty() { "" } else { ": " }, parts.join(", "))
    }
}

/// `412 MiB`, `1.6 GiB`.
pub fn mib(bytes: usize) -> String {
    let m = bytes as f64 / (1u64 << 20) as f64;
    if m >= 1024.0 { format!("{:.1} GiB", m / 1024.0) } else { format!("{m:.0} MiB") }
}

impl Canvas {
    /// What this canvas holds in memory, buffer by buffer (capacity, not
    /// length, so it is what the allocator gave out).
    pub fn memory(&self) -> Memory {
        let w = &self.wet;
        let lat_heap: usize = w.lat.iter().map(latent_heap).sum();
        let mut parts = vec![
            ("picture", vec_bytes(&self.px) + vec_bytes(&self.gloss) + vec_bytes(&self.absorb)),
            ("spectral backing", vec_bytes(&self.backing)),
            ("relief", vec_bytes(&self.height) + vec_bytes(&self.film) + self.base.as_ref().map_or(0, |b| vec_bytes(&b.1)) + self.e7_contact.bytes()),
            ("wet paint", vec_bytes(&w.vol) + vec_bytes(&w.lat) + vec_bytes(&w.hide) + vec_bytes(&w.cover) + vec_bytes(&w.floor)),
            ("wet paint's tubes and layers", lat_heap),
            ("stroke ids", vec_bytes(&w.stroke) + vec_bytes(&w.touched) + vec_bytes(&w.laid_id)),
            ("solvent", vec_bytes(&w.solv) + vec_bytes(&w.laid)),
            ("drying", vec_bytes(&w.clock.px)),
            ("skin", self.skin.bytes()),
            ("drawing", self.drawing.as_ref().map_or(0, |d| d.bytes())),
            ("support", vec_bytes(&self.micro) + self.soak.as_ref().map_or(0, |s| vec_bytes(&s.weave)) + self.sheet.as_ref().map_or(0, |s| vec_bytes(&s.over) + vec_bytes(&s.a) + vec_bytes(&s.r))),
            ("blend", self.receptive.as_ref().map_or(0, vec_bytes)),
        ];
        parts.retain(|p| p.1 > 0);
        parts.sort_by(|a, b| b.1.cmp(&a.1));
        Memory { pixels: self.f.w * self.f.h, parts }
    }
}

/// Bytes per pixel a canvas holds whatever is painted on it: the picture,
/// relief, wet layer, stroke ids and drying state every canvas allocates.
pub const FIXED_PER_PIXEL: usize = {
    let rgb = size_of::<crate::color::Rgb>();
    // px, gloss, absorb; height, film, contact level and its seen copy;
    // vol, lat, hide, cover, floor; stroke, touched; drying
    rgb + 2 * 4 + 4 * 4 + 4 + size_of::<Latent>() + size_of::<crate::wet::Prop>() + 2 * 4 + 2 * 4 + size_of::<crate::drying::Px>()
};

/// Bytes per pixel engine 9 adds once paint is on it: the spectral backing,
/// solvent bookkeeping, and the cure skin's column head.
pub const ENGINE9_PER_PIXEL: usize = size_of::<crate::mixer::Backing>() + 3 * 4 + 4;

/// What a canvas of `w × h` pixels with engine `engine` will hold once
/// painted, before any retained wet layers (each layer a pixel keeps adds
/// about `LAYER_BYTES`). A lower bound: the buffers every such canvas has.
pub fn estimate(w: usize, h: usize, engine: u32) -> usize {
    let per = FIXED_PER_PIXEL + if engine >= crate::PIGMENT_MIXER { ENGINE9_PER_PIXEL } else { 0 };
    w * h * per
}

/// About what one retained wet layer adds to the pixel that keeps it.
pub const LAYER_BYTES: usize = size_of::<Layer>();
