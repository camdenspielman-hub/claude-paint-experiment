//! Kubelka–Munk pigments, following Curtis et al. 1997, "Computer-Generated
//! Watercolor", section 5.1:
//! https://grail.cs.washington.edu/wp-content/uploads/2015/08/curtis-1997-cgw.pdf
//!
//! A pigment layer is absorption K and scattering S per channel (per coat of
//! thickness). Two ways to name one:
//!
//! - by **masstone** and scattering (`Pigment::masstone`): the color of the
//!   paint laid thick enough to hide anything, R∞, plus how strongly it
//!   scatters. This is how wet paint is described (`Paint`, `Tube`): a coat
//!   of paint over paint of its own masstone looks the same at any thickness,
//!   so a mark matched to the field it sits in disappears into it.
//! - by **appearance** (`from_appearance`, `with_hiding`, `transparent`, …):
//!   how a unit layer looks over white and over black. Natural for glazes
//!   and varnish films named by the tint they give a white ground.
//!
//! Either way we composite layers of any thickness over whatever is already
//! painted.

use crate::color::{Rgb, luminance};

/// K/S of a paint whose masstone (infinitely thick reflectance) is `r`.
#[inline]
pub fn ks_of(r: f32) -> f32 {
    let r = r.clamp(0.002, 0.995);
    (1.0 - r) * (1.0 - r) / (2.0 * r)
}

/// Hiding of a unit coat of a paint with masstone reflectance `r` and
/// scattering `s`: its reflectance over black divided by over white (the
/// paint industry's contrast ratio). 0 = clear glaze, 1 = hides completely.
pub fn hiding_of(r: f32, s: f32) -> f32 {
    hiding_ks(ks_of(r), s)
}

/// `hiding_of` given `q = ks_of(r)`. The same operations as
/// `Pigment::over` on channel 0 of a gray pigment (every channel is that
/// one), with the unit layer evaluated once for both substrates. Keep the
/// over formula literal: `t * t * 0.0` is not 0 for an infinite or NaN `t`.
#[inline]
fn hiding_ks(q: f32, s: f32) -> f32 {
    let (r, t) = layer1(s * q, s, 1.0);
    let (z, o) = (0.0f32, 1.0f32);
    let b = r + t * t * z / (1.0 - r * z).max(1e-6);
    let w = r + t * t * o / (1.0 - r * o).max(1e-6);
    (b / w.max(1e-6)).clamp(0.0, 1.0)
}

/// The scattering (per coat) that gives a unit coat of masstone `r`
/// (luminance) the contrast ratio `hiding`. Inverse of `hiding_of`.
pub fn scatter_for(r: f32, hiding: f32) -> f32 {
    let h = hiding.clamp(1e-4, 0.9995);
    let q = ks_of(r);
    let (mut lo, mut hi) = (-9.0f32, 9.0f32); // ln s
    for _ in 0..40 {
        let mid = 0.5 * (lo + hi);
        if hiding_ks(q, mid.exp()) < h {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

/// K and S of one channel (or one wavelength) of a unit layer that
/// reflects `rw` over white and `rb` over black.
#[inline]
pub(crate) fn ks_from_appearance(rw: f32, rb: f32) -> (f32, f32) {
    let rw = rw.clamp(0.004, 0.996);
    let rb = rb.clamp(0.001, rw - 0.002);
    let a = 0.5 * (rw + (rb - rw + 1.0) / rb);
    let b = (a * a - 1.0).max(1e-8).sqrt();
    let z = (b * b - (a - rw) * (a - 1.0)) / (b * (1.0 - rw));
    let arcoth = 0.5 * ((z + 1.0) / (z - 1.0).max(1e-8)).ln();
    let s = (arcoth / b).max(0.0);
    (s * (a - 1.0), s)
}

/// Reflectance and transmittance of one channel (or one wavelength) of a
/// layer with absorption `k` and scattering `s` per coat, `x` coats thick
/// (`x` > 0).
#[inline]
pub(crate) fn layer1(k: f32, s: f32, x: f32) -> (f32, f32) {
    if s < 1e-6 {
        // pure absorber
        return (0.0, (-k * x).exp());
    }
    let a = 1.0 + k / s;
    let b = (a * a - 1.0).max(0.0).sqrt();
    if b < 1e-3 {
        // (nearly) non-absorbing: the b → 0 limit of the formulas below
        let sx = s * x;
        return (sx / (1.0 + a * sx), 1.0 / (1.0 + a * sx));
    }
    let bsx = (b * s * x).min(40.0);
    let (sh, ch) = (bsx.sinh(), bsx.cosh());
    let c = a * sh + b * ch;
    (sh / c, b / c)
}

#[derive(Clone, Copy, Debug)]
pub struct Pigment {
    pub k: Rgb,
    pub s: Rgb,
}

impl Pigment {
    /// Derive K and S from a layer's appearance over white and over black.
    pub fn from_appearance(on_white: Rgb, on_black: Rgb) -> Self {
        let mut k = [0.0; 3];
        let mut s = [0.0; 3];
        for i in 0..3 {
            (k[i], s[i]) = ks_from_appearance(on_white[i], on_black[i]);
        }
        Pigment { k, s }
    }

    /// A paint of masstone `r` (the color it has laid thick, or over itself)
    /// that scatters `s` per coat, the same in every channel: scattering
    /// (by white and by particle edges) is nearly flat across the spectrum,
    /// absorption carries the hue.
    pub fn masstone(r: Rgb, s: f32) -> Self {
        let s = s.max(1e-6);
        Pigment { k: [s * ks_of(r[0]), s * ks_of(r[1]), s * ks_of(r[2])], s: [s; 3] }
    }

    /// A paint of masstone `r` with `hiding` (contrast ratio of a unit coat,
    /// measured on the luminance of the masstone).
    pub fn masstone_hiding(r: Rgb, hiding: f32) -> Self {
        Self::masstone(r, scatter_for(luminance(r), hiding))
    }

    /// The masstone R∞ of this pigment: what an infinitely thick layer looks like.
    pub fn masstone_color(&self) -> Rgb {
        std::array::from_fn(|i| {
            if self.s[i] < 1e-9 {
                return 0.0;
            }
            let a = 1.0 + self.k[i] / self.s[i];
            a - (a * a - 1.0).max(0.0).sqrt()
        })
    }

    /// A transparent glazing pigment (hides very little).
    pub fn transparent(color: Rgb) -> Self {
        Self::with_hiding(color, 0.06)
    }

    /// Aged varnish: a clear film that only absorbs (yellows), with almost
    /// no scattering, so it warms the darks without veiling them.
    pub fn varnish(color: Rgb) -> Self {
        Self::with_hiding(color, 0.004)
    }

    /// Semi-opaque (scumbles, thin body color).
    pub fn semi(color: Rgb) -> Self {
        Self::with_hiding(color, 0.45)
    }

    /// Nearly opaque body color.
    pub fn opaque(color: Rgb) -> Self {
        Self::with_hiding(color, 0.92)
    }

    /// `hiding` = fraction of the over-white appearance that survives over black.
    pub fn with_hiding(color: Rgb, hiding: f32) -> Self {
        let on_black = [color[0] * hiding, color[1] * hiding, color[2] * hiding];
        Self::from_appearance(color, on_black)
    }

    /// Reflectance and transmittance of a layer of thickness `x`.
    #[inline]
    pub fn layer(&self, x: f32) -> (Rgb, Rgb) {
        let mut r = [0.0; 3];
        let mut t = [1.0; 3];
        if x <= 0.0 {
            return ([0.0; 3], [1.0; 3]);
        }
        for i in 0..3 {
            (r[i], t[i]) = layer1(self.k[i], self.s[i], x);
        }
        (r, t)
    }

    /// Composite a layer of thickness `x` over an opaque substrate.
    #[inline]
    pub fn over(&self, sub: Rgb, x: f32) -> Rgb {
        if x <= 0.0 {
            return sub;
        }
        let (r, t) = self.layer(x);
        let mut out = [0.0; 3];
        for i in 0..3 {
            out[i] = r[i] + t[i] * t[i] * sub[i] / (1.0 - r[i] * sub[i]).max(1e-6);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `hiding_of` as it was, through `Pigment::over` on all three channels.
    fn hiding_of_over(r: f32, s: f32) -> f32 {
        let p = Pigment { k: [s * ks_of(r); 3], s: [s; 3] };
        let b = p.over([0.0; 3], 1.0)[0];
        let w = p.over([1.0; 3], 1.0)[0];
        (b / w.max(1e-6)).clamp(0.0, 1.0)
    }

    /// The same, given K/S directly, to reach K/S below what `ks_of` returns.
    fn hiding_q_over(q: f32, s: f32) -> f32 {
        let p = Pigment { k: [s * q; 3], s: [s; 3] };
        let b = p.over([0.0; 3], 1.0)[0];
        let w = p.over([1.0; 3], 1.0)[0];
        (b / w.max(1e-6)).clamp(0.0, 1.0)
    }

    /// `scatter_for` as it was, bisecting on `hiding_of_over`.
    fn scatter_for_over(r: f32, hiding: f32) -> f32 {
        let h = hiding.clamp(1e-4, 0.9995);
        let (mut lo, mut hi) = (-9.0f32, 9.0f32);
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if hiding_of_over(r, mid.exp()) < h {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (0.5 * (lo + hi)).exp()
    }

    /// Same bits, or both NaN (NaN sign and payload are not guaranteed).
    fn same(a: f32, b: f32) -> bool {
        if a.is_nan() { b.is_nan() } else { a.to_bits() == b.to_bits() }
    }

    fn grid(lo: f32, hi: f32, n: usize) -> impl Iterator<Item = f32> {
        (0..=n).map(move |i| lo + (hi - lo) * i as f32 / n as f32)
    }

    /// The one-layer `hiding_of` and the hoisted `scatter_for` give the bits
    /// the three-channel `Pigment::over` form gave, on every branch of
    /// `layer1`: pure absorber, nearly non-absorbing and clamped `bsx`.
    #[test]
    fn hiding_of_matches_pigment_over_bit_for_bit() {
        let mut rs: Vec<f32> = grid(0.0, 1.0, 64).collect();
        rs.extend([0.002, 0.995, 0.9999, -0.5, 1.5, f32::NAN]);
        let mut ss: Vec<f32> = (0..=96).map(|i| 1e-6 * 8e9f32.powf(i as f32 / 96.0)).collect();
        ss.extend([0.0, -0.0, 1e-7, 9.9e-7, 1e-6, 0.16, 8e3, 1e30, f32::INFINITY, f32::NAN]);
        for &r in &rs {
            for &s in &ss {
                assert!(same(hiding_of(r, s), hiding_of_over(r, s)), "hiding_of({r}, {s})");
            }
        }
        for q in [0.0, 1e-9, 1e-8, 1e-7, 4e-7, 1e-6, 1e-4, 1.0, 249.0, f32::INFINITY, f32::NAN] {
            for &s in &ss {
                assert!(same(hiding_ks(q, s), hiding_q_over(q, s)), "hiding_ks({q}, {s})");
            }
        }
        let mut hs: Vec<f32> = grid(0.0, 1.0, 40).collect();
        hs.extend([1e-5, 0.9999, f32::NAN]);
        for r in rs.iter().copied().step_by(4).chain([0.002, 0.995, f32::NAN]) {
            for &h in &hs {
                assert!(same(scatter_for(r, h), scatter_for_over(r, h)), "scatter_for({r}, {h})");
            }
        }
    }
}
