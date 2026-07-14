#!/usr/bin/env python3
"""Redirect the combase.dll import to ole32.dll in a PE executable.

Windows 8 split the classic COM runtime out of ole32.dll into combase.dll.
The `windows-sys` crate (pulled in transitively by rfd/eframe) binds
`CoTaskMemFree` to combase.dll, which does not exist on Windows 7 — the loader
then fails with a missing-DLL error. ole32.dll still exports CoTaskMemFree on
*every* Windows version (Win7 through 11; on Win8+ it forwards to combase), so
rewriting the import DLL name to ole32.dll is safe everywhere.

The names are the same length once NUL-padded ("combase.dll\\0" -> "ole32.dll\\0\\0\\0",
12 bytes each), so no PE offsets shift. We only touch the exact import-table
byte sequence, and refuse to run if it appears anywhere but once.
"""
import sys

OLD = b"combase.dll\x00"          # 12 bytes
NEW = b"ole32.dll\x00\x00\x00"    # 12 bytes (NUL-padded to match)
assert len(OLD) == len(NEW)


def main(path: str) -> int:
    with open(path, "rb") as f:
        data = f.read()

    n = data.count(OLD)
    if n == 0:
        print(f"{path}: no combase.dll import — nothing to do")
        return 0
    if n != 1:
        print(f"{path}: expected exactly 1 combase.dll occurrence, found {n}; aborting")
        return 1

    data = data.replace(OLD, NEW, 1)
    with open(path, "wb") as f:
        f.write(data)
    print(f"{path}: redirected combase.dll -> ole32.dll")
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("usage: win7-redirect-combase.py <path-to-exe>", file=sys.stderr)
        sys.exit(2)
    sys.exit(main(sys.argv[1]))
