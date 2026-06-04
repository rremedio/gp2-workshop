# GP2 Workshop

A cross-platform editor/patcher for **Grand Prix 2**'s `GP2.EXE`. It edits the
24 per-slot "magic data" tuning tables and a curated set of statically-patchable
physics values (engine, power curve, aero, brakes, mass/grip, tyres,
slipstream), and reads/writes the legacy `.m2d` magic-data format plus a new,
human-readable physics TOML format.

> ## ⚠️ BACK UP YOUR `GP2.EXE` FIRST
>
> This tool patches `GP2.EXE` **in place**. Before you write anything, keep your
> own copy of the original executable somewhere safe.
>
> As a safety net, the editor automatically copies the file to `GP2.EXE.bak`
> **once** (the first time you write), and verifies every write by reloading the
> file and comparing bytes. But the `.bak` is only created on the first write and
> is never overwritten — it is **not** a substitute for your own backup. If you
> patch a file you already patched before, the `.bak` still holds the *previous*
> state, not the factory original. Keep a pristine copy yourself.

## What it does

- **Magic Data tab** — pick a slot (1–16), edit the 24 tables in a labelled
  grid, and Import/Export the slot directly to/from the EXE or Load/Save a
  `.m2d` file. The `.m2d` format is unchanged from the old editor (24 decimal
  lines, table order 1–24, one file per slot), so files are interchangeable.
  Table 6 is dead data (ignored by the game) and is shown disabled.
- **Physics tab** — sub-tabs for Engine, Power Curve, Aero, Brakes, Mass/Grip,
  Tyres and Slipstream. Basic fields are always shown; Advanced fields live
  behind a collapsing section. Fixed-point fields show their float value and a
  hex tooltip, and every field has a "↺ stock" reset button. The Power Curve
  sub-tab has an editable 36-entry table plus a line plot. Import/Export the
  whole physics set to/from the EXE, or Load/Save it as TOML.

A **calibration** step verifies known stock values in the EXE before any write.
If the file calibrates you'll see a green `✓ calibrated (Δ=0x…)` badge; if it
can't be calibrated (unknown/garbage image) the badge reads `⚠ uncalibrated`
and all Export-to-EXE actions are disabled (you can still load/save files).

## Scope

In scope: the 24 magic-data tables, the curated physics inventory above, the
power curve, the four tyre compounds' base grip/wear + worn floor. Out of scope
in this version: deep editing of the tyre wear curve and track-abrasion tables
(read-only / Advanced), per-car horsepower, track management, and saved-game
editing.

## Project layout

- `crates/gp2ws-core` — all logic (offsets, calibration, encoding, field
  registry, `.m2d` and TOML I/O). No GUI, fully unit-tested. The GUI holds no
  file offsets — it only talks to the `Session` façade.
- `crates/gp2ws-gui` — the [`eframe`/`egui`](https://github.com/emilk/egui)
  desktop application.

## Building

### Native (Linux / macOS / Windows)

```bash
cargo build --release -p gp2ws-gui
cargo run -p gp2ws-gui
```

### Cross-compiling to Windows from Linux

The Windows binary is built with the GNU toolchain via mingw-w64. One-time
setup:

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt-get install -y mingw-w64          # Debian/Ubuntu
```

`.cargo/config.toml` already points the linker at `x86_64-w64-mingw32-gcc`, so:

```bash
cargo build --release --target x86_64-pc-windows-gnu -p gp2ws-gui
```

produces `target/x86_64-pc-windows-gnu/release/gp2ws-gui.exe`.

## Testing

```bash
cargo test
```

Some integration tests verify offsets/values against a **real** `GP2.EXE`. They
are skipped unless you point them at a reference executable via the
`GP2WS_TEST_EXE` environment variable (the EXE is never committed to this repo):

```bash
GP2WS_TEST_EXE=/path/to/GP2.EXE cargo test
```

## Technical notes

- `GP2.EXE` is an uncompressed DOS executable; it is patched raw at file
  offsets. All multi-byte values are little-endian; fixed-point is Q.14
  (`0x4000` = 1.0).
- The standard EXE has no segment shift (Δ = 0). Calibration first tries Δ = 0,
  then a small window-scan fallback to derive a consistent delta.
