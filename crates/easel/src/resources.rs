//! What the easel uses of the machine, and what a canvas will need of it:
//! the process's memory now and at its peak, the memory the machine (or the
//! sandbox's cgroup) allows, and an estimate of a painting's memory from its
//! size, so a painter hears what a canvas costs before it runs out.
//!
//! Reading only: nothing here changes what is painted or what a log replays to.

/// The process's resident memory now and its peak, bytes (Linux; None elsewhere).
pub fn process_memory() -> Option<(u64, u64)> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let kib = |key: &str| status.lines().find_map(|l| l.strip_prefix(key)).and_then(|v| v.split_whitespace().next()?.parse::<u64>().ok()).map(|k| k * 1024);
    Some((kib("VmRSS:")?, kib("VmHWM:")?))
}

/// The most memory this process may use, bytes: the tightest of the machine's
/// RAM and the memory limits of the cgroups it runs in (a container or sandbox).
pub fn memory_limit() -> Option<u64> {
    let mut limit = meminfo("MemTotal:");
    let tighter = |limit: &mut Option<u64>, v: u64| {
        // (an unset cgroup limit reads as "max" or as a number near 2^63)
        if v < 1 << 60 {
            *limit = Some(limit.map_or(v, |l| l.min(v)));
        }
    };
    for path in cgroup_files() {
        if let Some(v) = std::fs::read_to_string(&path).ok().and_then(|s| s.trim().parse::<u64>().ok()) {
            tighter(&mut limit, v);
        }
    }
    limit
}

fn meminfo(key: &str) -> Option<u64> {
    let info = std::fs::read_to_string("/proc/meminfo").ok()?;
    info.lines().find_map(|l| l.strip_prefix(key)).and_then(|v| v.split_whitespace().next()?.parse::<u64>().ok()).map(|k| k * 1024)
}

/// The cgroup memory limit files that apply to this process: its own cgroup's
/// and each ancestor's, for cgroup v2 (`memory.max`) and v1
/// (`memory.limit_in_bytes`).
fn cgroup_files() -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let groups = std::fs::read_to_string("/proc/self/cgroup").unwrap_or_default();
    for line in groups.lines() {
        let mut parts = line.splitn(3, ':');
        let (_, controllers, path) = (parts.next(), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        let (base, file) = if controllers.is_empty() {
            ("/sys/fs/cgroup", "memory.max")
        } else if controllers.split(',').any(|c| c == "memory") {
            ("/sys/fs/cgroup/memory", "memory.limit_in_bytes")
        } else {
            continue;
        };
        let mut p = std::path::PathBuf::from(path);
        loop {
            files.push(std::path::Path::new(base).join(p.strip_prefix("/").unwrap_or(&p)).join(file));
            if !p.pop() {
                break;
            }
        }
    }
    files
}

/// `412 MiB`, `1.6 GiB`.
pub fn size(bytes: u64) -> String {
    paint::memory::mib(bytes as usize)
}

/// The easel process with no canvas yet: Lua, the tube box's tables, the
/// threads' stacks (measured: 53 MiB).
pub const PROCESS_BASE: u64 = 56 << 20;

/// Bytes a painted pixel holds beyond the buffers every canvas allocates: the
/// list of tubes its paint came from, any wet layers it keeps, and a pencil
/// underdrawing (measured on the sampler's study: tubes and layers 110 B/px
/// after one sitting of three passes, 130 to 180 after two; a drawing 84; the
/// tubes grow as more of them and more layers reach a pixel).
pub const PAINTED_EXTRA: u64 = 220;

/// What a chunk needs beside the canvas while it paints, looks and saves (a
/// pass's scratch, the drying's buffers, a look's lit copy, the rollback's
/// packing, memory the allocator keeps for the next pass), as a share of the
/// canvas (measured: 0.5 to 0.65 replaying, 0.7 in a live sitting at 2400 px
/// with a failed chunk and one stopped at the time limit).
pub const WORKING_SHARE: f64 = 0.75;

/// What a painting of `w × h` px will need: what its canvas holds once painted,
/// and the easel's peak while a chunk runs, beside the memory it may use.
pub struct Estimate {
    pub w: usize,
    pub h: usize,
    pub held: u64,
    pub peak: u64,
    pub limit: Option<u64>,
}

impl Estimate {
    pub fn new(w: usize, h: usize, engine: u32) -> Self {
        let canvas = paint::memory::estimate(w, h, engine) as u64 + (w * h) as u64 * PAINTED_EXTRA;
        Estimate { w, h, held: canvas, peak: PROCESS_BASE + canvas + (canvas as f64 * WORKING_SHARE) as u64, limit: memory_limit() }
    }

    /// Whether the peak may not fit: above 80% of the limit (false if the limit is unknown).
    pub fn tight(&self) -> bool {
        self.limit.is_some_and(|l| self.peak as f64 > 0.8 * l as f64)
    }

    /// `a 2400×1600 px canvas holds about 2.8 GiB once painted, and the easel peaks near
    /// 4.9 GiB while a chunk runs; this machine allows the easel 13.4 GiB`.
    pub fn describe(&self) -> String {
        let limit = self.limit.map_or(String::new(), |l| format!("; this machine allows the easel {}", size(l)));
        let warn = if self.tight() { " (it may run out of memory)" } else { "" };
        format!("a {}×{} px canvas holds about {} once painted, and the easel peaks near {} while a chunk runs{limit}{warn}", self.w, self.h, size(self.held), size(self.peak))
    }
}

/// One line for `status`: the canvas's memory, the process's, and the limit.
pub fn memory_line(canvas: Option<&paint::memory::Memory>) -> String {
    let mut parts = Vec::new();
    if let Some(m) = canvas {
        parts.push(format!("canvas {}", m.summary()));
    }
    if let Some((now, peak)) = process_memory() {
        parts.push(format!("easel process {} (peak {})", size(now), size(peak)));
    }
    if let Some(limit) = memory_limit() {
        parts.push(format!("limit {}", size(limit)));
    }
    format!("memory: {}", if parts.is_empty() { "unknown on this system".into() } else { parts.join(" · ") })
}

/// How long laying a brushed ground on a `w × h` px canvas takes, in words:
/// a hog brush's crossing strokes over the whole canvas, painted one after
/// another (each depends on the paste the last left). Measured on 4 cores
/// (it uses about one): 6 s at 300 px across, 15 s at 600, 61 s at 1200 and
/// 186 s at 2400 (aspect 1.5), so about 3 s plus 50 µs a pixel. A knife or
/// roller ground takes seconds; a ground prepared before at the same size,
/// seed and build is read back from the ground cache in a second or two.
pub fn ground_time(w: usize, h: usize) -> String {
    let secs = 3.0 + 50e-6 * (w * h) as f64;
    let about = if secs < 90.0 { format!("{:.0} s", (secs / 5.0).ceil() * 5.0) } else { format!("{:.0} min", (secs / 60.0).round()) };
    format!("a brushed ground at {w}×{h} px takes about {about} to lay (a knife or roller ground, or one in the ground cache, takes seconds); the painting's chunks take what their passes and waits take, and `status` says how far a running chunk's pass has got and about how long it has left")
}
