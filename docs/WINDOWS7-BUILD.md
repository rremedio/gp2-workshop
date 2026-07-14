# Building GP2-Workshop for Windows 7

The default Windows build does **not** run on Windows 7. It fails at launch with
errors such as:

> The program can't start because **api-ms-win-core-path-l1-1-0.dll** is missing…

This is not a bug in our code. **Rust dropped Windows 7/8 support in 1.78 (May
2024).** Since then the standard library and several common crates import APIs
that only exist on Windows 8/10+. To ship a Win7-compatible `.exe` we address
four separate sources of Win8+ imports.

## The one-command build

```bash
# one-time prerequisites
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
# also need: mingw-w64 (x86_64-w64-mingw32-gcc) and python3

./scripts/build-win7.sh
# -> target/x86_64-win7-windows-gnu/release/gp2ws-gui.exe
```

Verify the result imports only Win7-available DLLs:

```bash
x86_64-w64-mingw32-objdump -p target/x86_64-win7-windows-gnu/release/gp2ws-gui.exe \
  | grep -i 'DLL Name' | sort -uf
# must NOT contain: api-ms-win-*, combase.dll, or any *winrt* DLL
```

## What each piece fixes

| # | Win8+ import | Source | Fix |
|---|--------------|--------|-----|
| 1 | `api-ms-win-core-path`, `GetTempPath2W`, etc. (from `std`) | Rust std ≥ 1.78 | Build against the Tier-3 **`x86_64-win7-windows-gnu`** target with `-Z build-std` (nightly). std then uses Win7-safe fallbacks. |
| 2 | `api-ms-win-core-winrt-error` (`RoOriginateErrorW`), `propsys`, `uiautomationcore` | `accesskit_windows` via eframe's default **`accesskit`** feature | Disable it: `eframe` is declared with `default-features = false` and only the features we use (`glow`, `x11`, `wayland`, `default_fonts`, `persistence`). See `crates/gp2ws-gui/Cargo.toml`. Cost: no screen-reader/UI-Automation accessibility. |
| 3 | `api-ms-win-core-path` (`PathCchStripPrefix`) | `arboard` (clipboard backend, force-enabled by eframe → egui-winit) | Vendored patch: `third_party/arboard` replaces the `PathCchStripPrefix` call with a pure-Rust `\\?\` prefix strip. Wired via `[patch.crates-io]` in the root `Cargo.toml`. `PathCchStripPrefix` is a Win8 function that exists in **no** Win7 DLL, so the import must be removed, not redirected. |
| 4 | `combase.dll` (`CoTaskMemFree`) | `windows-sys` via rfd/eframe/home | Post-build PE patch: `scripts/win7-redirect-combase.py` rewrites the import DLL name to **`ole32.dll`**, which exports `CoTaskMemFree` on every Windows version. This function *is* used at runtime (file dialogs), so it must be redirected, not removed. |

## Renderer

eframe uses the **glow (OpenGL)** backend here (wgpu is not enabled). That is the
right choice for Win7 — but it needs a real OpenGL 2.1+/3.x driver. A Win7 box
with only the default Microsoft software GL 1.1 (no GPU driver installed) will
still fail to render. This is a target-machine driver requirement, independent of
the build.

## Verified how far?

The fixes above were verified by **static import analysis** of the produced
`.exe` (no `api-ms-win-*`, `combase`, or WinRT imports; all 15 imported DLLs and
every imported function name are Win7-era). End-to-end runtime testing on an
actual Windows 7 machine has **not** been performed here — do a smoke test on
Win7 SP1 before release.

## Maintenance note

`third_party/arboard` is pinned to arboard **3.6.1**. If eframe/egui is upgraded
and pulls a different arboard version, re-vendor that version and re-apply the
one-line `PathCchStripPrefix` change (or drop the patch if upstream stops using
it), then update the version in this table.
