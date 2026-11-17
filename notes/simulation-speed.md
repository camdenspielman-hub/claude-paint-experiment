# Painting faster, every bit the same

The simulator now does the same work with 20 to 45% fewer instructions,
and every painting replays to the same bytes: checkpoints, PNGs, printed
output and every chunk's state digest. Nothing a painting can see changed;
only how the engine keeps its books.

## Where the time went

Callgrind, one thread, the unchanged build (139f104), on six workloads:
a brushed ground at 150 px (the old kernel, which lays every brushed ground,
also for engine 9), an engine-3 log with pencil, the legacy verbs, the
sampler's tools on engine 9 (mix), and engine-9 passes over wet paint with
and without long waits (dry, paint).

- **Engine 9 spent most of its time on bookkeeping, not paint.** Every
  bristle touch of a pixel that keeps skim layers cloned the pixel's whole
  layer stack (288 bytes a layer plus its tube list), split it, mixed the
  top and threw the clones away; and every mix looked up each tube in the
  pixel's list with a linear search. On the paint workload `from_layers`,
  `take_with_lat`, `lay_layers`, `add_parts` and `split_top` were some 70%
  of all instructions, allocation 11%. The kernel's own arithmetic was a
  minority.
- **The old kernel** (engines 1 to 6, and brushed grounds) cloned latents it
  only read, returned 290-byte tuples to throw most of them away, dropped an
  always-empty vector on every mix, converted plough targets with saturating
  casts and called libm for floor.
- Drying, the spectral optics and the scheduler were each under 1-2%.

## What changed

Fifteen changes, each its own commit, each checked on its own and then all
together (below). In engine 9's layer path: the top of a layered pixel is
split in place and mixed straight into its latent (`split_top_in_place`,
`take_scaled_layers`, `peek_top`); tube entries are found by fast paths for
the two shapes nearly every call has, and a first-index table otherwise;
empty lists skip the calls; the kernel sums tubes in plain scratch lists and
mixes taken paint into a reused scratch latent; a concentration by exactly 1
is skipped; the landing loop vectorizes; the plough keeps its cell slots. In
the old kernel: borrowed latents, inline layerless take, pickup and lay with
the layered cases out of line, `&Surf`, float bounds for plough targets, an
exact inline floor. Elsewhere: a paint's hiding from one Kubelka-Munk layer
instead of six, and the passage sort by passage.

What makes each one exact: the same IEEE operations on the same values in
the same order (no reassociation, no fused multiply-add, no changed
precision or libm), the same entries in the same order in every tube and
layer list, the same RNG draws, and scratch reset to a fresh value's bits
before each use. Every change was proposed with that argument and attacked
by two reviewers before it was written (one on float semantics, one on
state, ordering and side effects; 6 of 37 proposals fell). The new tests
compare the old and new paths bit for bit (`splitting_in_place_matches_split_top`,
`indexed_parts_match_the_search`, `hiding_of_matches_pigment_over_bit_for_bit`,
`floor_and_ceil_match_libm_bit_for_bit`; the ignored
`floor_and_ceil_match_libm_on_every_f32` checks all 2^32 floats).

One edge is not fixed and never was: when both operands of a float sum are
NaN, which payload survives depends on operand order, which the compiler may
choose (Rust doesn't fix NaN payloads). A painting's tube amounts and shares
are never NaN (a checkpoint refuses them); for every finite value the new
paths match bit for bit.

## Measured

Instructions (callgrind, one thread), unchanged build -> now:

| workload | before | after | |
|---|---|---|---|
| paint: engine 9 passes over wet paint, 200 px | 471.1 G | 263.1 G | -44% (1.79x) |
| dry: engine 9 passes and long waits, 200 px | 200.0 G | 121.2 G | -39% (1.65x) |
| mix: engine 9 tools, 200 px | 89.2 G | 59.8 G | -33% (1.49x) |
| ground: a brushed ground, 150 px | 35.7 G | 25.1 G | -30% (1.42x) |
| bunny: an engine-3 log with pencil, 200 px | 27.9 G | 21.3 G | -24% (1.31x) |
| legacy: legacy verbs, engine 1, 128 px | 65.7 G | 53.5 G | -19% (1.23x) |


Checked: `scripts/exact_ab` (31 scenes, engines 1 to 9, 128 to 1200 px, one
to four threads) byte for byte against the unchanged build; the paint
crate's unit tests (the one failure, `knife_skim::a_skimming_knife_keeps_its_light_on_wet_dark_paint`,
is the gate's registered known failure and fails with the same numbers
before and after); and `scripts/test`.

## Not done

- **Parallel strokes.** A brushed ground lays one stroke after another (each
  depends on the paste the last left), so it uses one core. Running a
  stroke's bristle contacts as a dependency graph over the pixels they touch
  would keep the bits, but a simulation of that graph found about 1.5-fold
  parallelism on the ground and none elsewhere, for a large and delicate
  change. Not attempted.
- **AVX for the 48-lane lerp.** Exact (the same lane-wise operations), but
  x86 only and behind a runtime check; the paintings run on Apple silicon.
- Smaller items under 0.1% each (a prepared spectral ground, the wait
  path's scratch, one `seen` per look request).

## Doing more of this

`scripts/exact_ab <base easel> <candidate easel> [--full]` is the check: it
replays the scenes with both builds and compares every byte. Profile with
callgrind on a build with line tables
(`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only cargo build --release -p easel`),
one thread (`RAYON_NUM_THREADS=1`), no ground cache
(`EASEL_GROUND_CACHE=off`): instruction counts are the same however busy the
machine is.
