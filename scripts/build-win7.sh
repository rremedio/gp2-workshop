#!/usr/bin/env bash
# Build GP2-Workshop for Windows 7 (x86_64).
#
# Rust dropped Windows 7/8 support in 1.78, so a normal build imports Win10-only
# APIs. This script uses the Tier-3 `x86_64-win7-windows-gnu` target (which needs
# nightly + build-std to compile std for it) and then redirects the one remaining
# Win8+ import (combase.dll -> ole32.dll). The arboard PathCch import and the
# accesskit WinRT imports are handled at the source level (see docs/WINDOWS7-BUILD.md).
#
# Prereqs (one-time):
#   rustup toolchain install nightly
#   rustup component add rust-src --toolchain nightly
#   mingw-w64 (x86_64-w64-mingw32-gcc) + python3
set -euo pipefail

cd "$(dirname "$0")/.."

TARGET=x86_64-win7-windows-gnu

CARGO_TARGET_X86_64_WIN7_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
cargo +nightly build --release -p gp2ws-gui \
  --target "$TARGET" \
  -Z build-std=std,panic_abort "$@"

EXE="target/$TARGET/release/gp2ws-gui.exe"
python3 scripts/win7-redirect-combase.py "$EXE"

echo "Win7 build ready: $EXE"
