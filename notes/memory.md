# Memory and time: what a painting costs, and how the easel says so

## Where the memory goes

A canvas keeps every pixel's paint in memory. On an engine-9 painting
(every new one), after a sitting or two, a pixel holds about 700 bytes:

| part | bytes a pixel | what it is |
|---|---|---|
| wet paint | 292 | the wet film's volume, its 48-lane pigment latent, its ten properties, cover and floor |
| spectral backing | 176 | the dry picture under the wet paint as a 38-band spectrum (engine 9) |
| tubes and layers | 110–180, growing | each pixel's list of the tubes its paint came from, and any skim layers it keeps |
| the rest | ~100 | the picture, relief, contact level, stroke ids, solvent, drying state |

So a 2400 × 1800 px canvas holds about 3 GiB once painted, a 4800 px one
about four times that. The tube lists grow as more tubes and layers reach a
pixel. `Canvas::memory` (crates/paint/src/memory.rs) lists the parts of a
real canvas; `status` prints it.

While a chunk runs the easel needs more beside the canvas: a pass's scratch,
the drying's buffers, a look's lit copy, the Lua state and its masks (a mask
is a whole-canvas float buffer Lua sees only as a small userdata). Measured
on the sampler's study, the peak is 1.5 to 1.65 times the canvas, plus about
55 MiB for the process.

## What changed (and what each saved)

All of these keep every result as it was: on the sampler's study, the
checkpoint, the PNG and every chunk's state digest are byte for byte the
original build's, and the gate's goldens pass.

- **`easel run` no longer clones the finished canvas** to render and save it.
  It borrows it. One canvas less at the end of every replay.
- **The ground cache streams.** It gathered the whole prepared canvas in
  memory to digest it on both the write and the read, 80% above the canvas
  at the moment a painting begins. It now streams through the digest; the
  file format is the same.
- **Tube lists keep to their size.** They grew by doubling from four slots,
  so 40% of their room was unused, a third of the canvas's heap. A list now
  grows once per mix by exactly the tubes the mix brings (and a checkpoint
  builds its lists to size). Fewer reallocations made passes a little faster.
- **The rollback's buffers go into its packed file.** Before every live chunk
  the canvas is packed into a compressed temporary file so a failed chunk can
  be put back; the buffers the checkpoint leaves out were copied in memory
  beside the canvas for the whole chunk: floors, ceilings and solvent (16
  bytes a pixel), all property lanes without the drain switch (40), and the
  whole drawing (about 80). They are now written raw ahead of the state in
  the same file (`checkpoint::tests::a_packed_canvas_unpacks_bit_for_bit`).
- **Lua collects every 128 MiB of masks**, not every 400 MiB (24 masks of a
  2400 px canvas waited for the collector). When Lua collects is invisible
  to a painting: `__gc` and `__mode` are refused, and every chunk ends with
  a full collection.

## Measured

On this sandbox (4 cores), each run against the original build (4bbbbec):

| run | original | now | output |
|---|---|---|---|
| `easel run`, the sampler's study, 2 chunks at 600 px | 497 MiB peak, 83 s | 330 MiB, 79 s | checkpoint, PNG, state digests identical |
| `easel run`, its first chunk at 1200 px | 1496 MiB peak | 1011 MiB (before the tube lists) | checkpoint, PNG identical |
| starting a painting at 600 px from the ground cache (miss / hit) | 275 / 251 MiB | 153 / 187 MiB | identical to no cache |
| live sitting at 2400 × 1600: brushed ground, pencil, a chunk, a failed chunk, a chunk stopped at the 600 s limit, a look | 5.3 GiB peak, 3.9 GiB after | 4.6 GiB peak, 3.6 GiB after | live checkpoints (2.6 GB) and logs identical |

In that live sitting the sampler's second chunk ran past the easel's 600 s
limit at 2400 px, in both builds, and was put back. Its passes are what
take the time; `status` now says while it runs how much of its current pass
is left.

## What the easel says

- `status`: a `memory:` line with the canvas buffer by buffer, the easel
  process's resident memory and its peak, and the limit it runs under (the
  tightest of the machine's RAM and its cgroups' `memory.max` or
  `memory.limit_in_bytes`). The first line is unchanged.
- While a chunk runs, `status` says how much of its current pass is painted
  and about how long is left at its rate so far (`paint::progress`). The
  painters' clients already drop the running time from that line.
- The chunk that makes a canvas: what it will need once painted and near
  what while a chunk runs, beside the limit. A developer's easel puts it in
  the reply; a painter's only when the peak may pass 80% of the limit. The
  server log and `easel run` always print it.
- `easel estimate --aspect A [--width W]`: the same before a painting is
  begun, and how long its brushed ground takes to lay.
- `EASEL_MEMORY_TRACE=1 easel run …`: each chunk's canvas memory and the
  process's peak during it.

The estimates (crates/easel/src/resources.rs) are the buffers a canvas
allocates (`paint::memory::estimate`) plus measured allowances: 220 bytes a
pixel for painted paint's tube lists and a pencil underdrawing, 56 MiB for
the process, 0.75 of the canvas for a chunk's working memory. Checked at
2400 × 1600 px in a live sitting (below): the canvas held 2.7 GiB and the
easel peaked at 4.6 GiB; the estimate now says 2.8 and 4.9. A brushed ground takes about 3 s plus
50 µs a pixel to lay (measured 6 s at 300 px across, 15 at 600, 61 at 1200,
186 at 2400, on 4 cores of which it uses one: its strokes depend on each
other). A knife or roller ground takes seconds, and a ground in the cache
(same size, seed and build) is read back in a second or two.

## Not changed, and why

- **The physics kernel's time.** Done since, bit for bit: see
  `notes/simulation-speed.md` (20 to 45% fewer instructions). The brushed
  ground still runs one stroke after another, by design.
- **The per-pixel layout.** The 48-lane latent (192 bytes) and the 38-band
  backing (176) are most of a pixel. A sparse layout would cut them for
  pixels without wet paint, but every pixel's stale latent still shapes the
  bits of the next paint laid there, so it is a format change, not a
  cleanup.
