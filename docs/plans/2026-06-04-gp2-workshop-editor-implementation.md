# GP2 Workshop Editor — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.
> Use @superpowers:test-driven-development for every code task (RED → GREEN → commit).
> Design reference: `docs/plans/2026-06-04-gp2-workshop-editor-design.md`.

**Goal:** Build a cross-platform Rust/egui editor/patcher for GP2.EXE that edits the 24 per-slot
"magic data" tables and a curated set of statically-patchable physics values, saving/loading
`.m2d` (magic data, legacy-compatible) and a new TOML physics format.

**Architecture:** Cargo workspace with `gp2ws-core` (no GUI, all logic, fully unit-tested) and
`gp2ws-gui` (eframe app driven by the core's declarative field registry). GP2.EXE is patched raw
at file offsets (uncompressed; no native deps). An address-calibration step verifies known stock
values before any write.

**Tech Stack:** Rust (stable), `eframe`/`egui` + `egui_plot`, `serde` + `toml`, `thiserror`.
Cross-compile to `x86_64-pc-windows-gnu` via mingw-w64.

---

## Domain primer (read before starting)

- GP2.EXE is a DOS executable, uncompressed; we edit it raw at **file offsets**.
- Two address spaces from the disassembly docs:
  - **DATA file offset = IDA + 0x63254** (decimal **+406100**).
  - **CODE file offset = IDA + 0x78254**; for `mov reg,imm32`/`add reg,imm32` the 32-bit operand
    is at **IDA + 1 + 0x78254**.
- **Verified: no segment shift** for the standard EXE (doc offsets == real EXE == old editor).
- **Fixed point Q.14:** `0x4000` = 1.0.
- All multi-byte values are **little-endian**.
- Real reference EXE for integration tests: `/home/rremedio/vaults/gp2/gp2/GP2.EXE`
  (5,702,937 bytes). Never commit it; tests reach it only via the `GP2WS_TEST_EXE` env var.

---

## Task 0: Workspace scaffolding & feature branch

**Files:**
- Create: `Cargo.toml` (workspace), `crates/gp2ws-core/Cargo.toml`,
  `crates/gp2ws-core/src/lib.rs`, `crates/gp2ws-gui/Cargo.toml`,
  `crates/gp2ws-gui/src/main.rs`, `.cargo/config.toml`, `rust-toolchain.toml`.

**Step 1:** Create a feature branch (do not work on `main`):
```bash
git checkout -b feat/editor-core
```

**Step 2:** Create the workspace `Cargo.toml`:
```toml
[workspace]
resolver = "2"
members = ["crates/gp2ws-core", "crates/gp2ws-gui"]

[workspace.package]
edition = "2021"
version = "0.1.0"
```

**Step 3:** `crates/gp2ws-core/Cargo.toml`:
```toml
[package]
name = "gp2ws-core"
edition.workspace = true
version.workspace = true

[dependencies]
serde = { version = "1", features = ["derive"] }
toml = "0.8"
thiserror = "1"
```
`crates/gp2ws-core/src/lib.rs`: `pub mod encoding;` (stub modules added per task).

**Step 4:** `crates/gp2ws-gui/Cargo.toml`:
```toml
[package]
name = "gp2ws-gui"
edition.workspace = true
version.workspace = true

[dependencies]
gp2ws-core = { path = "../gp2ws-core" }
eframe = "0.29"
egui_plot = "0.29"
rfd = "0.15"   # native file dialogs
```
`crates/gp2ws-gui/src/main.rs`: minimal `fn main()` printing a placeholder (replaced in Task 12).

**Step 5:** `.cargo/config.toml` for cross-compile:
```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```
`rust-toolchain.toml`:
```toml
[toolchain]
channel = "stable"
```

**Step 6:** Verify it builds and commit:
```bash
cargo build
git add -A && git commit -m "chore: scaffold cargo workspace and cross-compile config"
```
Expected: clean build of both crates.

---

## Task 1: Value encoding/decoding

The registry stores **decoded human values**; the EXE stores **raw words**. This module converts.

**Files:**
- Create: `crates/gp2ws-core/src/encoding.rs`
- Test: same file (`#[cfg(test)]`).

**Step 1: Write failing tests.**
```rust
// encoding.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Encoding {
    Raw,          // value shown == raw
    Bias(i64),    // shown = raw - bias  (power curve: bias = 0xF282)
    Fixed(u32),   // Q.x fixed point; shown as raw (UI formats the float separately)
    Hex,          // shown as raw, UI renders hex
}

/// raw stored in the EXE -> human value shown in the UI
pub fn decode(raw: i64, enc: Encoding) -> i64 {
    match enc {
        Encoding::Bias(b) => raw - b,
        _ => raw,
    }
}
/// human value -> raw to write back
pub fn encode(value: i64, enc: Encoding) -> i64 {
    match enc {
        Encoding::Bias(b) => value + b,
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bias_roundtrip_power_curve() {
        let bias = 0xF282;
        // stock raw 0xF29B decodes to 25 with old bias, re-encodes to 0xF29B
        let raw = 0xF29B;
        let shown = decode(raw, Encoding::Bias(bias));
        assert_eq!(shown, 25);
        assert_eq!(encode(shown, Encoding::Bias(bias)), raw);
    }
    #[test]
    fn raw_is_identity() {
        assert_eq!(decode(1313, Encoding::Raw), 1313);
        assert_eq!(encode(1313, Encoding::Raw), 1313);
    }
}
```
**Step 2:** `cargo test -p gp2ws-core encoding` → expect FAIL (module not wired) then implement.
**Step 3:** Implement as above; add `pub mod encoding;` to `lib.rs`.
**Step 4:** `cargo test -p gp2ws-core encoding` → PASS.
**Step 5:** Commit: `feat(core): value encoding/decoding with bias support`.

---

## Task 2: EXE byte I/O

**Files:**
- Create: `crates/gp2ws-core/src/exe.rs`; add `pub mod exe;` to `lib.rs`.

**Step 1: Failing tests** (operate on an in-memory `Vec<u8>` so no real EXE needed):
```rust
pub struct ExeImage { pub bytes: Vec<u8> }

impl ExeImage {
    pub fn from_bytes(b: Vec<u8>) -> Self { Self { bytes: b } }
    pub fn read(&self, off: usize, width: u8) -> u64 {
        let s = &self.bytes[off..off + width as usize];
        let mut v = 0u64;
        for (i, &b) in s.iter().enumerate() { v |= (b as u64) << (8 * i); } // little-endian
        v
    }
    pub fn write(&mut self, off: usize, width: u8, val: u64) {
        for i in 0..width as usize {
            self.bytes[off + i] = ((val >> (8 * i)) & 0xFF) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_write_u16_le() {
        let mut img = ExeImage::from_bytes(vec![0; 16]);
        img.write(4, 2, 0x48A0);
        assert_eq!(img.bytes[4], 0xA0);
        assert_eq!(img.bytes[5], 0x48);
        assert_eq!(img.read(4, 2), 0x48A0);
    }
    #[test]
    fn read_u32_le() {
        let img = ExeImage::from_bytes(vec![0,0,0,0, 0x00,0x00,0x04,0x00]);
        assert_eq!(img.read(4, 4), 0x40000);
    }
}
```
**Step 2–4:** RED → implement → GREEN (`cargo test -p gp2ws-core exe`).
**Step 5:** Add file load/save:
```rust
use std::path::Path;
impl ExeImage {
    pub fn load(path: &Path) -> std::io::Result<Self> {
        Ok(Self::from_bytes(std::fs::read(path)?))
    }
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        std::fs::write(path, &self.bytes)
    }
    /// copy to `<path>.bak` if no backup exists yet
    pub fn backup(path: &Path) -> std::io::Result<()> {
        let bak = path.with_extension("bak");
        if !bak.exists() { std::fs::copy(path, &bak)?; }
        Ok(())
    }
}
```
**Step 6:** Commit: `feat(core): little-endian EXE byte I/O with backup`.

---

## Task 3: Target resolution (IDA → file offset)

**Files:** Create `crates/gp2ws-core/src/target.rs`; `pub mod target;`.

**Step 1: Failing tests.**
```rust
pub const DATA_BASE: usize = 0x63254;   // +406100
pub const CODE_BASE: usize = 0x78254;

#[derive(Clone, Copy, Debug)]
pub enum Target {
    Data(usize),       // IDA address of a data word
    Code(usize),       // IDA address of an instruction; operand is at +1
    Direct(usize),     // already a file offset (legacy old-editor offsets)
}

impl Target {
    /// file offset before calibration delta is applied
    pub fn base_offset(&self) -> usize {
        match *self {
            Target::Data(ida) => ida + DATA_BASE,
            Target::Code(ida) => ida + 1 + CODE_BASE,
            Target::Direct(off) => off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn data_offset() { assert_eq!(Target::Data(0xD53DC).base_offset(), 1279536); }
    #[test] fn direct_offset() { assert_eq!(Target::Direct(1282824).base_offset(), 1282824); }
    #[test] fn code_operand_after_opcode() {
        // DF slope: instruction IDA 0x1682D, operand file 0x8EA82
        assert_eq!(Target::Code(0x1682D).base_offset(), 0x8EA82);
    }
}
```
**Step 2–4:** RED → implement → GREEN.
**Step 5:** Commit: `feat(core): IDA->file-offset target resolution`.

---

## Task 4: Calibration

Calibration reads known **anchors**; if all match at Δ=0 the EXE is the standard one. Otherwise it
scans a small window for the known value per segment to derive Δ, requiring agreement.

**Files:** Create `crates/gp2ws-core/src/calibration.rs`; `pub mod calibration;`.

**Step 1: Failing tests** (synthetic image with anchors placed at the standard offsets):
```rust
use crate::exe::ExeImage;
use crate::target::Target;

pub struct Anchor { pub target: Target, pub width: u8, pub stock: u64 }

pub const ANCHORS: &[Anchor] = &[
    Anchor { target: Target::Data(0xD53DC), width: 4, stock: 0x40000 }, // off_D53DC
    Anchor { target: Target::Data(0xD5D96), width: 2, stock: 0x0B00 },  // word_D5D96
    Anchor { target: Target::Data(0xD5DEE), width: 2, stock: 0x48A0 },  // t_tiretypetab[A]
];

#[derive(Debug, PartialEq)]
pub enum Calibration { Calibrated { delta: i64 }, Failed }

pub fn calibrate(img: &ExeImage) -> Calibration {
    // try delta = 0 first
    if ANCHORS.iter().all(|a| img.read(a.target.base_offset(), a.width) == a.stock) {
        return Calibration::Calibrated { delta: 0 };
    }
    // (window-scan fallback added in Step 5)
    Calibration::Failed
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synthetic() -> ExeImage {
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        for a in ANCHORS { img.write(a.target.base_offset(), a.width, a.stock); }
        img
    }
    #[test] fn calibrates_standard_exe() {
        assert_eq!(calibrate(&synthetic()), Calibration::Calibrated { delta: 0 });
    }
    #[test] fn fails_on_garbage() {
        let img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        assert_eq!(calibrate(&img), Calibration::Failed);
    }
}
```
**Step 2–4:** RED → implement → GREEN.
**Step 5:** Add a window-scan fallback (search ±0x400 around the data-segment anchors for the stock
values; if a single consistent Δ satisfies all data anchors, return it; else `Failed`). Add a test
that shifts every anchor by +0x10 in a synthetic image and asserts `Calibrated { delta: 0x10 }`.
**Step 6:** Commit: `feat(core): EXE address calibration with window-scan fallback`.

---

## Task 5: Field registry types

**Files:** Create `crates/gp2ws-core/src/field.rs`; `pub mod field;`.

**Step 1:** Define types (no tests yet — exercised in later tasks):
```rust
use crate::encoding::Encoding;
use crate::target::Target;

#[derive(Clone, Copy, Debug, PartialEq)] pub enum Section { MagicData, Physics }
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubTab { Engine, PowerCurve, Aero, Brakes, MassGrip, Tyres, Slipstream }
#[derive(Clone, Copy, Debug, PartialEq)] pub enum Tier { Basic, Advanced }

#[derive(Clone, Debug)]
pub struct FieldDesc {
    pub id: &'static str,
    pub label: &'static str,
    pub help: &'static str,
    pub subtab: SubTab,
    pub tier: Tier,
    pub target: Target,
    pub width: u8,
    pub signed: bool,
    pub encoding: Encoding,
    pub stock: i64,
    pub range: Option<(i64, i64)>,
}

impl FieldDesc {
    pub fn read(&self, img: &crate::exe::ExeImage, delta: i64) -> i64 {
        let off = (self.target.base_offset() as i64 + delta) as usize;
        let raw = img.read(off, self.width) as i64;
        let raw = if self.signed { sign_extend(raw, self.width) } else { raw };
        crate::encoding::decode(raw, self.encoding)
    }
    pub fn write(&self, img: &mut crate::exe::ExeImage, delta: i64, value: i64) {
        let off = (self.target.base_offset() as i64 + delta) as usize;
        let raw = crate::encoding::encode(value, self.encoding);
        img.write(off, self.width, mask(raw, self.width));
    }
}

fn sign_extend(v: i64, width: u8) -> i64 {
    let bits = width as u32 * 8;
    let shift = 64 - bits;
    (v << shift) >> shift
}
fn mask(v: i64, width: u8) -> u64 { (v as u64) & ((1u128 << (width*8)) - 1) as u64 }
```
**Step 2:** Add a round-trip test: build a synthetic image, write a `FieldDesc` value, read it back, assert equality (including a signed Q14 value and a `Bias` curve value).
**Step 3–4:** RED → GREEN.
**Step 5:** Commit: `feat(core): field descriptor with read/write + sign handling`.

---

## Task 6: Physics field tables (data)

Encode the curated inventory from the design §6.2 as `static` slices. **All offsets below are
verified file offsets** (use `Target::Data` with the IDA address, or `Target::Code` where noted).

**Files:** Create `crates/gp2ws-core/src/physics_fields.rs`; `pub mod physics_fields;`.

Populate `pub static PHYSICS_FIELDS: &[FieldDesc]` from this table (IDA addresses unless "Direct"):

| id | subtab | tier | target | width | signed | enc | stock | notes |
|---|---|---|---|---|---|---|---|---|
| rev_limiter | Engine | Basic | Direct(1281904) | 2 | n | Raw | 0x4268 | legacy old-editor offset |
| max_rpm | Engine | Basic | Direct(1281924) | 2 | n | Raw | 15000 | legacy |
| rpm_light_1 | Engine | Basic | Direct(1284436) | 4 | n | Raw | — | legacy; **verify the 4 light offsets** vs old Designer |
| rpm_light_2 | Engine | Basic | Direct(1284440) | 4 | n | Raw | — | legacy |
| rpm_light_3 | Engine | Basic | Direct(1284444) | 4 | n | Raw | — | legacy |
| rpm_light_4 | Engine | Basic | Direct(1284448) | 4 | n | Raw | — | legacy |
| soft_powercut_rpm | Engine | Advanced | Data(0xD6010) | 4 | n | Raw | 15200 | |
| df_scale | Aero | Basic | Data(0xD5EA8) | 4 | n | Raw | 44369 | d_wingfactor (= old "Downforce") |
| rear_df_slope | Aero | Basic | Code(0x1682D) | 4 | n | Raw | 430 | |
| rear_df_floor | Aero | Basic | Code(0x1683B) | 4 | n | Raw | 2064 | |
| rear_drag_slope | Aero | Basic | Code(0x168EC) | 4 | n | Raw | 1259 | |
| rear_drag_floor | Aero | Basic | Code(0x168FA) | 4 | n | Raw | 1792 | |
| front_wing_slope | Aero | Advanced | Code(0x1685F) | 4 | n | Raw | 0x1AE | |
| front_wing_floor | Aero | Advanced | Code(0x1686D) | 4 | n | Raw | 0x810 | |
| front_aero_scale | Aero | Advanced | Data(0xD5EA4) | 4 | n | Raw | 0x18F5 | |
| lateral_drag_x | Aero | Advanced | Data(0xD5FD4) | 4 | n | Raw | 0x1A00 | |
| lateral_drag_y | Aero | Advanced | Data(0xD5FD8) | 4 | n | Raw | 0x5800 | |
| rear_wing_drag_factor | Aero | Advanced | Data(0xD53E0) | 4 | n | Raw | 0x1000 | |
| brake_force | Brakes | Basic | Data(0xD53FC) | 4 | n | Raw | 0x160000 | |
| brake_fail_mult | Brakes | Advanced | Data(0xD56A8) | 4 | n | Raw | 0x1000 | |
| abs_threshold | Brakes | Advanced | Data(0xD5400) | 4 | n | Raw | 0x3E800 | |
| ai_brake_lookahead_1 | Brakes | Advanced | Data(0xC9970) | 4 | n | Raw | 0x3C00 | |
| ai_brake_lookahead_2 | Brakes | Advanced | Data(0xC9974) | 4 | n | Raw | 0x3C00 | |
| std_weight | MassGrip | Basic | Data(0xD5E74) | 4 | n | Raw | 1313 | |
| norm_weight | MassGrip | Basic | Data(0xD5E78) | 4 | n | Raw | 1313 | |
| gravity | MassGrip | Advanced | Data(0xD5EA0) | 4 | y | Raw | -0x80000 | signed |
| fuel_factor | MassGrip | Advanced | Data(0xD57D8) | 4 | n | Raw | 1627167 | |
| packer_factor | MassGrip | Advanced | Data(0xD54B4) | 4 | n | Raw | 0xD70340 | |
| rebound_factor | MassGrip | Advanced | Data(0xD54C8) | 4 | n | Raw | 0x546000 | |
| min_grip_clamp | MassGrip | Advanced | Data(0xD5E3E) | 2 | n | Hex | 0x2C00 | |
| cc_grip_qual | MassGrip | Basic | Data(0xD5974) | 2 | n | Raw | 0x40CC | overall CC grip qual |
| cc_grip_race | MassGrip | Basic | Data(0xD5978) | 2 | n | Raw | 0x40CC | overall CC grip race |
| tow_strength | Slipstream | Basic | Data(0xD53DC) | 4 | n | Raw | 0x40000 | 0 = off |
| tow_reach | Slipstream | Basic | Data(0xC9750) | 2 | n | Raw | 0x180 | |
| tow_align_width | Slipstream | Basic | Data(0xC9748) | 2 | n | Raw | 0x200 | |
| ai_tow_strength | Slipstream | Basic | Data(0xD5FF4) | 4 | n | Raw | 0 | dormant; set >0 |
| tow_max_wake | Slipstream | Advanced | Data(0xC9752) | 2 | n | Raw | 0x100 | |
| tow_max_range | Slipstream | Advanced | Data(0xC9766) | 2 | n | Raw | 7 | |
| tow_min_speed | Slipstream | Advanced | Data(0xD5D96) | 2 | n | Raw | 0xB00 | |
| ai_follow_base_1..7 | Slipstream | Advanced | Data(0xC96CA/CC/D2/D4/D8/DA/DC) | 2 | y | Hex | F000/F400/A000/FC00/F800/F000/FE00 | 7 signed thresholds |
| ai_follow_floor_1..3 | Slipstream | Advanced | Data(0xD5FE8/EC/F0) | 4 | n | Hex | FC00/D000/5000 | |

> Tyres and power-curve are tables, handled in Tasks 7 and 8 (not in `PHYSICS_FIELDS`).
> Grip-μ and rev-limit/min-rpm **code immediates** are left for a follow-up; not in v1 inventory.

**Step 1:** Write the static slice from the table above.
**Step 2 (integration test, env-gated):**
```rust
#[test]
fn stock_values_match_real_exe() {
    let Ok(p) = std::env::var("GP2WS_TEST_EXE") else { return; }; // skip if unset
    let img = crate::exe::ExeImage::load(std::path::Path::new(&p)).unwrap();
    let crate::calibration::Calibration::Calibrated { delta } =
        crate::calibration::calibrate(&img) else { panic!("not calibrated") };
    for f in PHYSICS_FIELDS {
        if f.stock == i64::MIN { continue; } // sentinel for "unknown stock"
        assert_eq!(f.read(&img, delta),
                   crate::encoding::decode(f.stock, f.encoding),
                   "field {} mismatch", f.id);
    }
}
```
**Step 3:** Run with the real EXE:
`GP2WS_TEST_EXE=/home/rremedio/vaults/gp2/gp2/GP2.EXE cargo test -p gp2ws-core stock_values_match_real_exe`
Expected: PASS. **If any field mismatches, fix its offset/width/sign before continuing** (this is
the cheap way to catch a wrong address). Mark `rpm_light_*` stocks as the sentinel until verified.
**Step 4:** Commit: `feat(core): curated physics field inventory + real-EXE verification test`.

---

## Task 7: Power curve table

**Files:** Create `crates/gp2ws-core/src/power_curve.rs`; `pub mod power_curve;`.

**Spec:** 36 `u16` entries at file offset **1282824**, stride 2 (`Direct`). Decoded value =
`raw - 0xF282` (old proven bias). Reuse `FieldDesc`-style read/write or a dedicated helper:
```rust
pub const POWER_CURVE_BASE: usize = 1282824;
pub const POWER_CURVE_LEN: usize = 36;
pub const POWER_CURVE_BIAS: i64 = 0xF282;
```
**Step 1: Failing test** — read the curve from a synthetic image with known raw words, assert
decoded values; write decoded values back, assert raw round-trips.
**Step 2–4:** RED → implement `read_curve(&img, delta) -> [i64; 36]` and
`write_curve(&mut img, delta, &[i64; 36])` → GREEN.
**Step 5 (env-gated):** load the real EXE, read the curve, assert the first decoded entries are
`[0, 0, 25, 50, ...]` (old bias) and that the curve is monotically rising into the mid-range
(sanity check, not exact values).
**Step 6:** Commit: `feat(core): power-curve table read/write (old bias)`.

---

## Task 8: Tyre-compound tables

**Files:** Create `crates/gp2ws-core/src/tyre_fields.rs`; `pub mod tyre_fields;`.

Tables (all `Data`, little-endian):
- `t_TyretypeGrip` (wear sensitivity) — IDA 0xD5DDE, 4 × u16, stock A=0x3000 B=0x4000 C=0x6000 D=0xC000.
- `t_tiretypetab` (base grip) — IDA 0xD5DEE, 4 × u16, stock A=0x48A0 B=0x49A0 C=0x4AA0 D=0x4BA0.
- `worn_floor` `dword_D5DFE` — IDA 0xD5DFE, u32, stock 0x4600.
- `wear_curve` `dword_D5E0A` — IDA 0xD5E0A, Advanced (sequence of (wear,grip-delta) i32 pairs
  terminated by 0xFFFFFFFF). Expose read-only summary in v1; editing deferred — **note in UI**.
- `track_abrasion` `t_TyreUsgofTrck` — IDA 0xD5D9E, 16 × u32, Advanced.

**Step 1:** Define static descriptors for the 4+4+1 scalar tyre fields (reuse `FieldDesc`).
**Step 2:** Round-trip unit test + env-gated real-EXE stock check (A/B/C/D grip & wear, worn floor).
**Step 3–4:** RED → GREEN.
**Step 5:** Commit: `feat(core): tyre-compound field inventory + verification`.

---

## Task 9: Magic-data model + `.m2d` I/O

**Files:** Create `crates/gp2ws-core/src/magic.rs`; `pub mod magic;`.

**Layout (verified base+stride pairs; addr = base + slot*stride, slot 0..15):**
```rust
// (base_file_offset, slot_stride) for tables 1..24
pub const MAGIC_LAYOUT: [(usize, usize); 24] = [
 (1280584,2),(1280616,4),(1280680,4),(1280744,4),(1280840,2),(1280872,2),
 (1280904,2),(1280936,2),(1280976,4),(1280978,4),(1281040,2),(1281072,2),
 (1281134,2),(1281230,6),(1281232,6),(1281326,6),(1281328,6),(1281422,2),
 (1281454,6),(1281456,6),(1281458,6),(1281550,2),(1281582,2),(1281614,4),
];
pub const MAGIC_DEAD_TABLE: usize = 6; // T6 = dead data (still written for file fidelity)
```
All 24 are read/written as `u16`. Apply the calibration `delta`.

**Step 1: Failing tests.**
```rust
pub fn read_slot(img: &ExeImage, delta: i64, slot: usize) -> [u16; 24] { /* ... */ }
pub fn write_slot(img: &mut ExeImage, delta: i64, slot: usize, vals: &[u16; 24]) { /* ... */ }
/// .m2d = 24 decimal lines, table order 1..24
pub fn parse_m2d(text: &str) -> Result<[u16; 24], MagicError> { /* ... */ }
pub fn to_m2d(vals: &[u16; 24]) -> String { /* 24 lines, trailing newline per line */ }

#[test] fn m2d_roundtrip() {
    let v: [u16;24] = core::array::from_fn(|i| (i as u16)*100 + 7);
    assert_eq!(parse_m2d(&to_m2d(&v)).unwrap(), v);
}
#[test] fn slot_roundtrip_synthetic() {
    let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
    let v: [u16;24] = core::array::from_fn(|i| (i as u16)*3 + 1);
    write_slot(&mut img, 0, 5, &v);
    assert_eq!(read_slot(&img, 0, 5), v);
}
```
**Step 2:** Confirm `to_m2d` matches the old format: 24 lines, decimal, `\r\n` or `\n` — the old
editor used `StreamWriter.WriteLine` (platform newline). Use `\r\n` for max Windows compatibility;
`parse_m2d` must tolerate both and ignore blank trailing lines.
**Step 3–4:** RED → GREEN.
**Step 5 (env-gated):** read slot 0 from the real EXE, write it to a temp file as `.m2d`,
re-parse, assert equality.
**Step 6:** Commit: `feat(core): magic-data slot I/O and .m2d format`.

---

## Task 10: Physics TOML model

**Files:** Create `crates/gp2ws-core/src/physics_io.rs`; `pub mod physics_io;`.

Represent the whole physics set as a `BTreeMap<String, i64>` keyed by `FieldDesc.id`, plus the
power curve (`Vec<i64>`) and tyre tables, serialized to/from versioned TOML.

**Step 1: Failing tests.**
```rust
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct PhysicsDoc {
    pub meta: Meta,                       // { format: String, version: u32 }
    pub fields: std::collections::BTreeMap<String, i64>,
    pub power_curve: Vec<i64>,
}
pub fn to_toml(doc: &PhysicsDoc) -> String { /* toml::to_string */ }
pub fn from_toml(s: &str) -> Result<PhysicsDoc, PhysicsError> { /* validate meta.format */ }

#[test] fn toml_roundtrip() {
    let doc = PhysicsDoc { /* a couple fields + a 36-len curve */ };
    assert_eq!(from_toml(&to_toml(&doc)).unwrap(), doc);
}
#[test] fn rejects_wrong_format() { assert!(from_toml("[meta]\nformat='x'\nversion=1\n").is_err()); }
```
**Step 2:** Add `import_from_exe(img, delta) -> PhysicsDoc` (reads every `PHYSICS_FIELDS` + curve +
tyres) and `export_to_exe(img, delta, &PhysicsDoc)` (writes them; missing keys left untouched +
collected into a returned warning list).
**Step 3–4:** RED → GREEN; add a synthetic round-trip test (import → export → import == identity).
**Step 5:** Commit: `feat(core): physics TOML model + EXE import/export`.

---

## Task 11: Core public API surface

**Files:** Modify `crates/gp2ws-core/src/lib.rs`.

Expose a small façade the GUI uses (so the GUI holds no offset logic):
```rust
pub struct Session {
    pub path: std::path::PathBuf,
    pub img: exe::ExeImage,
    pub calibration: calibration::Calibration,
}
impl Session {
    pub fn open(path: &Path) -> Result<Session, Error> { /* load + calibrate */ }
    pub fn delta(&self) -> Option<i64> { /* Some if calibrated */ }
    pub fn save_backup_and_write(&mut self) -> Result<(), Error> { /* backup + save + verify */ }
    // magic + physics helpers delegate to the modules
}
```
**Step 1–2:** Add a doc-test or unit test for `Session::open` against a synthetic-file temp path.
**Step 3:** Commit: `feat(core): Session façade for the GUI`.

---

## Task 12: GUI skeleton (eframe)

GUI logic is thin; **manual verification** via @superpowers:verification-before-completion using
the `run` skill. Keep all conversions in core.

**Files:** Rewrite `crates/gp2ws-gui/src/main.rs` + add `crates/gp2ws-gui/src/app.rs`.

**Step 1:** eframe app struct holding `Option<Session>` and UI state (current slot, magic field
buffer `[String;24]`, physics field buffers, current tab/subtab, "show advanced" bool).
**Step 2:** Top bar: "Open GP2.EXE" (rfd file dialog) → `Session::open`; show calibration status
(`✓ Δ=0x{:X}` / `⚠ uncalibrated`); disable Export buttons unless calibrated.
**Step 3:** Tab bar with `Magic Data` / `Physics`. Render placeholder bodies.
**Step 4:** Build + run on Linux: `cargo run -p gp2ws-gui`. Manually open the real EXE; confirm the
calibration badge reads `✓ Δ=0x0`. Screenshot via the `run`/`verify` skill.
**Step 5:** Commit: `feat(gui): eframe skeleton with EXE open + calibration badge`.

---

## Task 13: Magic Data tab

**Files:** Add `crates/gp2ws-gui/src/magic_tab.rs`.

**Step 1:** Slot `ComboBox` (1..16). On change, `read_slot` into the 24 string buffers.
**Step 2:** Grid of 24 labeled fields (labels/help from `magic-data.md`); T6 rendered disabled with
"(ignored by game)" tooltip. Per-field validation (parse `u16`).
**Step 3:** Buttons: Import (read slot from EXE), Export (write slot → `save_backup_and_write`),
Load `.m2d`, Save `.m2d` (rfd dialogs; default filename `md-slot-{n}.m2d`).
**Step 4:** Manual verification: load real EXE, edit a slot, Export, reopen, confirm persisted;
Save `.m2d` and diff against an old-editor `.m2d` (byte-identical aside from newline style).
**Step 5:** Commit: `feat(gui): magic data tab with .m2d load/save`.

---

## Task 14: Physics tab + sub-tabs + power-curve plot

**Files:** Add `crates/gp2ws-gui/src/physics_tab.rs`.

**Step 1:** Sub-tab selector (Engine/PowerCurve/Aero/Brakes/MassGrip/Tyres/Slipstream). Render
`PHYSICS_FIELDS` filtered by subtab; Basic always shown, Advanced behind a `CollapsingHeader`.
Fixed-point fields show a float + hex tooltip; each field has a "↺ stock" button.
**Step 2:** Power Curve sub-tab: editable table of 36 entries + an `egui_plot::Line` of the decoded
values vs index.
**Step 3:** Physics-level buttons: Import EXE, Export EXE, Load TOML, Save TOML.
**Step 4:** Manual verification: load real EXE → Import → values match expected stock (e.g.
tow_strength shows 262144, brake_force 1441792); change tow_strength, Export, reopen, confirm.
Save TOML, inspect it's human-readable; Load it back.
**Step 5:** Commit: `feat(gui): physics tab, sub-tabs, power-curve plot, TOML I/O`.

---

## Task 15: Cross-compile, docs, finish

**Step 1:** Install target + linker (document in README):
```bash
rustup target add x86_64-pc-windows-gnu
sudo apt-get install -y mingw-w64
cargo build --release --target x86_64-pc-windows-gnu -p gp2ws-gui
```
Expected: `target/x86_64-pc-windows-gnu/release/gp2ws-gui.exe` produced.
**Step 2:** Write `README.md`: what it does, scope, build (native + cross), the `GP2WS_TEST_EXE`
test env var, and the **"back up your GP2.EXE"** warning.
**Step 3:** Run the full suite: `cargo test` and
`GP2WS_TEST_EXE=/home/rremedio/vaults/gp2/gp2/GP2.EXE cargo test`. All green.
**Step 4:** Use @superpowers:finishing-a-development-branch to decide merge/PR.
**Step 5:** Commit: `docs: README + cross-compile instructions`.

---

## Notes / open verification items (carry into execution)
- **rpm_light_1..4, rev_limiter, max_rpm offsets** are ported from the old editor's
  `GP2Addresses.PhysicsAddresses`; confirm the exact `ptextBox`→label→address mapping from the old
  `Form1.Designer.cs` (or by reading the values from the real EXE) before trusting them. Use the
  sentinel-stock pattern in Task 6 so the verification test doesn't false-fail.
- **Power-curve length (36 vs doc's 35)**: keep 36 (old proven range); the 36th word may be a guard
  entry. Revisit only if the parallel doc-bias verification says otherwise.
- **Doc-bias power curve (`0xF269`)**: parked. If adopted later, add a second `Encoding::Bias`
  option and a UI toggle — descriptor is already structured for it.
- `wear_curve` and `track_abrasion` editing UX is minimal in v1 (read/Advanced); deepen later.
