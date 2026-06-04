# GP2 Workshop — Physics & Magic Data Editor/Patcher — Design

**Date:** 2026-06-04
**Status:** Validated design, ready for implementation planning
**Supersedes:** the C# WinForms "GP2 Slot and Tyre Editor" (https://github.com/rremedio/gp2_slot_and_tyre_editor)

## 1. Purpose & scope

A cross-platform (Linux + Windows) editor/patcher for **statically patchable** data in
Microprose *Grand Prix 2* (`GP2.EXE`). It is a focused rebuild of the old "Slot and Tyre
Editor", dropping the track-management and saved-games features. **Two sections only:**

1. **Magic Data** — the 24 per-track-slot tuning tables (16 slots).
2. **Physics** — a rich tabbed section absorbing engine, power curve, aero/wings, brakes,
   mass & grip, **tyre-compound tables**, and **player + AI slipstream**.

A **Misc** section (human grip, overall CC grip, session duration, refuel, track-size limit,
simultaneous-cars limit) is explicitly **deferred** to a future version. Track management and
saved-game editing are **out of scope entirely**.

### Hard constraints
- **Magic data must remain `.m2d`-compatible** (24-line decimal text, one file per slot).
- **No per-car fields** (no per-car HP table, no per-car base-grip). Engine power is tuned only
  via the global torque curve and global scales.
- Must run on Linux and Windows; **cross-compile to Windows from this Linux box**.
- Only edit fields that are meaningful. Skip dead data (magic-data table 6), runtime-only
  values (`d_FrameTime`/`D6046`, `w_tirefactor`, the rebuilt AI decel table `D44CC`), and
  cosmetic-but-pointless fields. (RPM Lights ARE kept — see §6.)

## 2. Reference: GP2.EXE addressing (verified against the real undoctored EXE)

Source docs: `physics-patch-reference.md`, `physics.md`, `magic-data.md`, `slipstream.md`,
`tyres.md` (in `/home/rremedio/vaults/gp2/docs`).

- **DATA file offset = IDA + 0x63254** (decimal +406100).
- **CODE file offset = IDA + 0x78254**; for `mov reg,imm32` / `add reg,imm32` the operand is at
  `IDA + 1 + 0x78254`.
- Fixed point: **Q.14** (`0x4000` = 1.0); angles `0x10000` = 360°.

**Verified** against `/home/rremedio/vaults/gp2/gp2/GP2.EXE` (5,702,937 bytes, undoctored):
`off_D53DC`=0x40000 @1279536, `word_D5D96`=0x0B00 @1282026, `t_tiretypetab[A]`=0x48A0 @1282114,
`t_TyretypeGrip[A]`=0x3000 @1282098, magic T1/slot1 @1280584, power curve @1282824.
**There is NO data-segment shift for the standard DOS EXE** — the doc offsets, the real EXE, and
the old editor's offsets all coincide (Δ=0). (An earlier apparent ~0x200 shift was an arithmetic
error in deriving decimal offsets.)

**Power curve note:** the old editor's "power curve" and the docs' `t_Engrpm` are the **same
table** at file offset 1282824. They differ **only in the bias constant**: old subtracts
`0xF282` (62082 → entries 0,0,25,50,…), the docs subtract `0xF269` (62057 → 25,25,50,75,…).
**We use the old bias (`0xF282`) for now** — it is proven and produces good results. The doc bias
is parked for parallel verification; the curve descriptor is structured so it can be swapped later.

`GP2.EXE` is uncompressed and edited raw at file offsets — **no decompression / native DLL is
needed** (the old `gp2lib.dll` was only for saved-game decrunching, which is out of scope).

## 3. Tech stack & architecture

**Rust + `egui`/`eframe`** (glow backend), chosen for pure-Rust cross-compilation (no system GUI
deps; `cargo build --target x86_64-pc-windows-gnu` with mingw-w64 → single `.exe`) and immediate-
mode UI that suits a read→edit→write tool. Power-curve plot via `egui_plot`. Physics TOML via
`serde` + `toml`.

**Workspace, two crates:**

- **`gp2ws-core`** (library, no GUI, fully unit-testable):
  - Field registry (declarative descriptors — see §4).
  - EXE I/O: open, read/write `u8`/`u16`/`u32` at file offsets.
  - **Address calibration** (see §5).
  - Magic-data model + `.m2d` load/save + EXE import/export.
  - Physics model + TOML load/save + EXE import/export.
  - Backup-before-write and verify-after-write.
- **`gp2ws-gui`** (eframe binary): tabs and widgets driven entirely by the core's descriptors.

**Dev workflow:** native Linux build + tests for iteration; release also cross-compiled to
`x86_64-pc-windows-gnu`. `.cargo/config.toml` sets the mingw linker; README documents
`rustup target add x86_64-pc-windows-gnu` and `apt install mingw-w64`.

## 4. Field model

Every patchable value is a declarative descriptor — adding/removing a knob is a one-line edit:

```rust
struct FieldDesc {
    id: &'static str,
    label: &'static str,
    help: &'static str,
    section: Section,          // MagicData | Physics
    subtab: SubTab,            // Engine | PowerCurve | Aero | Brakes | MassGrip | Tyres | Slipstream
    tier: Tier,                // Basic | Advanced
    target: Target,            // Data(ida) | Code(ida) | Direct(file_offset)
    width: Width,              // U8 | U16 | U32
    signed: bool,
    encoding: Encoding,        // Raw | Fixed{q} | Bias{n} | Hex
    stock_value: u32,          // calibration anchor + "reset to stock"
    range: Option<(i64,i64)>,
}
```

- `encoding` drives both the widget and the EXE conversion: `Raw` (int), `Fixed{q}` (shown as a
  float, `0x4000`=1.0), `Bias{n}` (curve entries; n = 0xF282 for now), `Hex`.
- Tables (torque curve, tyre compound A–D, wear-curve breakpoints) are descriptors carrying a
  count + stride.
- Conversions (bias, fixed-point, hidden segment offsets) happen **only at EXE read/write**; the
  UI and TOML store decoded human values.

## 5. Address calibration (correctness backbone)

Although Δ=0 for the standard EXE, calibration guards against variant/patched EXEs and prevents
writing to the wrong bytes:

- A small set of **anchors** per segment with known stock values: `off_D53DC`=0x40000,
  `word_D5D96`=0x0B00, `t_tiretypetab[A]`=0x48A0, etc.
- On opening a `GP2.EXE`: read each anchor at its computed offset. If all match → Δ=0,
  **calibrated**. If not, scan a small window for the known value to derive a per-segment Δ;
  require multiple anchors to agree.
- If calibration fails (unknown variant) → mark **uncalibrated**, warn, and **disable writes**
  (user may override explicitly).
- **Safety:** auto-backup to `GP2.EXE.bak` before the first write; **verify-after-write** by
  reading the bytes back and comparing.

## 6. Field inventory (curated)

### 6.1 Magic Data (per slot, 24 `u16` tables — `.m2d` compatible)
Order preserved from the old editor (tables 1–24). Improved labels/help from `magic-data.md`:
- T1 tyre/track grip · T2/3/4 cornering-grip (always/qual/race) · T5 out-lap grip bias ·
  **T6 dead data (shown greyed, written through for file fidelity)** · T7/8 per-driver pace
  (qual/race) · T9/10 lap-clock rate (qual/race) · T11/12 difficulty grip (SemiPro/Rookie) ·
  T13 CC mistake rate · **T14–17 pit entry/exit geometry** (labeled per corrected record) ·
  T18 pit-approach zone · T19/20 pit-in/out distance · T21 pit-in speed · T22/23 fuel burn
  (human/CC) · T24 reference lap time.

Note: the old editor's per-slot word layout already matches the engine's reads for 9/10 and 11/12
(separate low/high words). T14–17 keep the old positional mapping for `.m2d` compatibility, but
labels reflect the corrected meanings.

### 6.2 Physics — sub-tabs

**Engine** (`A.3` + RPM lights):
- Rev limiter (hard, 0x4268=17000) · Auto-shift / Max RPM (`w_rpm15000`=15000) ·
  Soft power-cut RPM (`dword_D6010`=15200) · **RPM Lights 1–4** (cosmetic but kept — required so
  rev-limiter / max-RPM changes don't break the dashboard) · global engine/power scales as
  documented (no per-car HP).

**Power Curve** (own sub-tab + line plot): the torque table at 1282824, old bias `0xF282`.

**Aero / Wings** (`A.1`):
- `d_wingfactor` (global downforce scale) · rear-wing downforce **slope/floor** · rear-wing drag
  **slope/floor** · front-wing slope/floor · `dword_D5EA4` (front-aero scale) ·
  `off_D5FD4`/`off_D5FD8` (lateral drag, Advanced) · `off_D53E0` (rear-wing drag factor, Advanced).
  (Enables the §11 "wing-1-always" rebalance.)

**Brakes** (`A.4`):
- `dword_D53FC` (brake force) · `off_D56A8` (brake-failure mult, Advanced) · `off_D5400` (ABS
  threshold, Advanced) · AI braking lookahead `off_C9970`/`off_C9974` (Advanced).
  (`word_D44CC` excluded — rebuilt at session start, not static.)

**Mass & Grip** (`A.5`/`A.6`):
- `d_carstdweight` · `d_NormCarWeight` · `dword_D5EA0` (gravity, −32.0 Q14) · `d_fuelfactor` ·
  `d_packerfactor` · `d_reboundfactor` · `off_D5E3E` (min grip clamp) ·
  `off_D5974`/`off_D5978` (global CC-grip scale qual/race) · grip-μ code immediate (Advanced).

**Tyres** (`tyres.md`, all static):
- `t_tiretypetab` (base grip A–D) · `t_TyretypeGrip` (wear sensitivity A–D) · `dword_D5DFE`
  (worn floor) · `dword_D5E0A` (wear→grip breakpoints, Advanced) · `t_TyreUsgofTrck` (per-track
  abrasion, 16 dwords, Advanced). (`w_tirefactor` excluded — runtime-set. `b_TireType` excluded —
  it's track-file data, not EXE.)

**Slipstream** (`slipstream.md` §9 tiers):
- *Basic:* Player tow strength (`off_D53DC`, 0=off) · Player tow reach (`word_C9750`) · Player
  tow alignment width (`word_C9748`) · AI tow strength (`dword_D5FF4`, dormant 0 → set >0).
- *Advanced:* `off_C9752` (max wake) · `off_C9766` (max range) · `word_D5D96` (min speed) · AI
  follow-threshold bases `C96CA/CC/D2/D4/D8/DA/DC` + floors `D5FE8/EC/F0`.
- *Never exposed:* `dword_D6046` (frame-time scale — runtime, global).

## 7. File formats

- **`.m2d`** — unchanged: 24 lines, one decimal `u16` per line, table order 1–24, one file per
  slot. Byte-for-byte compatible with the old editor.
- **Physics — new versioned TOML**, one file for the whole physics set. Stored values are decoded
  human numbers; bias/fixed-point/segment conversions happen only at EXE I/O. Unknown/missing keys
  load as stock defaults with a warning (forward-compatible). Sections: `[meta]`, `[engine]`,
  `[aero]`, `[brakes]`, `[mass_grip]`, `[tyres]`, `[slipstream]`.
- **No old `.g2p` import** — only ~5 of 52 old fields map cleanly; trivial to re-enter.
- **Backup:** `GP2.EXE.bak` before the first patch.

## 8. UI

- **Top bar:** GP2.EXE path picker · calibration status (`✓ calibrated` / `⚠ uncalibrated`) ·
  backup indicator. Writes disabled until loaded + calibrated.
- **Tabs:** `Magic Data` (slot selector + 24-field grid; Import/Export EXE, Load/Save `.m2d`) and
  `Physics` (sub-tabs per §6.2; Basic shown, Advanced collapsible; Import/Export EXE, Load/Save
  TOML at the Physics level).
- **Widgets** from `encoding`: validated numeric; fixed-point as float with hex tooltip; per-field
  **reset-to-stock**.

## 9. Testing & build

- **TDD on `gp2ws-core`:** encoding round-trips (Q14, bias, code-immediate), `.m2d` round-trip,
  TOML round-trip, calibration logic, EXE read/write at offsets.
- **Fixture:** synthetic byte buffer seeded with anchor stock values (GP2.EXE is copyrighted, not
  committed). Optional gitignored `GP2WS_TEST_EXE` env var points at the real EXE
  (`/home/rremedio/vaults/gp2/gp2/GP2.EXE`) for integration tests.
- **Cross-compile:** `.cargo/config.toml` + mingw-w64; Linux dev build, Windows release `.exe`.

## 10. Out of scope (future)
Misc section (human/CC grip, session duration, refuel, track size, simultaneous cars), doc-bias
power curve (parallel verification), per-track abrasion editing UX polish, doc-corrected stride
editing for magic data beyond labels, track management, saved-game editing.
