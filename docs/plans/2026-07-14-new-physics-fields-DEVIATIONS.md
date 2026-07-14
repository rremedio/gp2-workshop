# Plan deviations — `2026-07-14-new-physics-fields`

Everything that went differently from
[`2026-07-14-new-physics-fields.md`](2026-07-14-new-physics-fields.md), why, and
what was done instead. Covers all phases, from pre-flight through Task 16.

**Outcome:** plan complete. 19 commits, registry **56 → 234 fields** across 14
sub-tabs, every stock byte-verified against a pristine GP2.EXE. Final:
`55 passed; 0 failed` (core), `15 passed` (gui), 0 build warnings.

**Legend:** 🔴 CHANGED (plan was wrong) · ⬜ SKIPPED · ➕ ADDED (not in plan) ·
🔧 PROCESS (how, not what) · ✅ PREDICTED (plan called it)

---

## Summary table

| # | Task | Kind | What the plan said | What shipped |
|---|---|---|---|---|
| 1 | pre-flight | 🔴 | verify against `/home/rremedio/GP2/GP2.EXE` | vault pristine copy — the plan's EXE is a patched mule |
| 2 | pre-flight | 🔧 | (worktree/branch per skill) | committed on `main`, matching repo history |
| 3 | 0 | 🔧 | spot-check 2–3 magic labels | checked all 15 + all 24 layout bases |
| 4 | 0 | 🔧 | — | `cargo fmt` reverted; repo isn't rustfmt-clean |
| 5 | 1 | 🔧 | run the GUI, eyeball the tab row | couldn't drive GUI (no `xdotool`); Roberto eyeballed |
| 6 | 2 | 🔧 | unit-test `CodeData` | also byte-verified all 12 Task 14 stocks early |
| 7 | 3 | ➕ | "update PARAMETER-GUIDE if it lists tabs" | also fixed two factually wrong sections |
| 8 | 5 | 🔧 | byte-verify stocks | camber verified *structurally* — stock 0 is unverifiable by value |
| 9 | 6 | 🔴 | `damage_load_floor`, 1 field, stock 917504 | **4 per-wheel fields**, stocks `{0x100000,0x100000,0xE0000,0xE0000}` |
| 10 | 7 | 🔴 | "wet tables are dead — do NOT port" | true for 12 fields, **wrong for 3** — fixed after shipping |
| 11 | 8 | ➕ | — | test fixtures 1.4 MB → 2 MB (7 files) |
| 12 | 13 | 🔴 | `steer_master_clamp` width `2?` | **`dd` → width 4** |
| 13 | 15 | 🔴 | wings are code immediates | **`Target::Data`** — no immediates exist there |
| 14 | 15 | 🔴 | 2 wing fields | **3** — plan omitted front-wing damage |
| 15 | 15 | ✅ | "`Target::Code` (+1) may be wrong" | **wrong for 3 of 9 sites** (+6/+7) |
| 16 | 15 | ⬜ | `freerev_cap` | skipped — 2 sites, unrepresentable |
| 17 | 15 | ⬜ | `clutch_engage_rpm` | skipped — 3 sites, unrepresentable |
| 18 | 15 | ✅⬜ | `limiter_bounce_base` "may re-read D6010; if so SKIP" | skipped — it does |
| 19 | 15 | ✅⬜ | `softcut_fade_base` — same check | skipped — it does |
| 20 | 16 | 🔧 | "update with all new sub-tabs/fields" | Basic in tables, Advanced in prose — 234 rows won't fit |
| 21 | — | ➕ | (out of scope) | T14–17 RE detour; vault errata; `D5D6C` candidate |

---

## 1. 🔴 The verification EXE (affects every task)

**Plan:** every task's verify command is
`GP2WS_TEST_EXE=/home/rremedio/GP2/GP2.EXE cargo test -p gp2ws-core`.

**Problem:** that file is Roberto's live test mule — **1324 bytes off stock**
(e.g. `rev_limiter` 6200 vs 15200, from low-rev engine experiments). The harness
**failed at baseline**, before any change.

**Instead:** `/home/rremedio/vaults/gp2/gp2/GP2.EXE` — byte-identical to an
independent copy on the external drive, both carrying the original 1996-07-19
timestamp, and it passes clean. Confirmed stock independently: the plan's own
Task 15 states `practice_fuel_laps` stock = 12; the vault EXE has 12, `GP2.bak`
has 10. `GP2.bak` also contains 62 NOP (`0x90`) runs and `jz`→`jmp` patches where
the vault has real code.

**Why it mattered:** this is not bookkeeping. Deviation #9 (`damage_load_floor`)
would have **passed green on the mule and shipped wrong.**

> The plan doc still hardcodes the mule path in every task. Left unedited —
> recorded here and in memory instead.

---

## 2. 🔴 `damage_load_floor` — 1 field → 4 (Task 6)

**Plan:** `damage_load_floor | Data 0xC7A60 | 4 | n | 917504` — one scalar.

**Reality:** a 4-entry per-wheel array. The code is unambiguous:

```
00032B02  mov edi, [esi+ecx*4+208h]
00032B09  cmp edi, osCarDamageTrack[ecx*4]   ; INDEXED
00032B16  cmp edi, dword_0_C7A70[ecx*4]      ; same shape, next line
```

Pristine stock is a rear/front split `{0x100000, 0x100000, 0xE0000, 0xE0000}`,
matching the `rear,rear,front,front` pattern of **every** neighbouring table.

**Root cause (the important part):** EXEs in the `GP2.bak`/mule lineage have this
array **flattened to `0xE0000` ×4 by GP2Edit**. When all four entries are equal,
**indexed and scalar are indistinguishable by value** — so the flattening didn't
just corrupt the number, it *hid the indexing*. That reading went into
`physics-aero-damage.md` §B7 (which writes `C7A60` plain while writing every
other table in the same loop as `[i]`), into the listing's data section, and from
there into the plan.

**Instead** (agreed with Roberto): shipped `damage_load_floor_rl/rr/fl/fr`,
mirroring `spring_break_*` which the plan already models as four. One field would
have silently written only the rear-left of a four-wheel array — the same class
of bug as the magic T14–17 mis-striding.

**Also fixed:** `NEW-FIELD-CANDIDATES.md:164` (it marked the neighbour `[4]` but
not this one); vault erratum added to `physics-aero-damage.md`.

---

## 3. 🔴 The "dead wet tables" — right for 12 fields, wrong for 3 (Task 7)

**Plan:** "The wet (`*min`) tables are dead (blend constant 0) — do NOT port."

**Shipped on that basis**, with every Surfaces help saying its wet twin "is dead
and does nothing". **That was wrong for three fields**, and shipped in `9340734`
before being caught.

`physics-surface-sampling.md` (Roberto's own vault) records that **class 5 (kerb
back-apron) reads one dword past each 5-entry max table** —
`mov eax, t_gripmax[ecx*4]`, class from `D5692 & 0xF` — landing on `min[0]`.
Verified against the EXE: `gripmax[5]`/`accmax[5]`/`roughmax[5]` resolve to
`0xD5D08`/`0xD5D30`/`0xD5D58` = `0x3800`/`0x2000`/`0x100`.

So `gripmin[0]`, `accmin[0]`, `roughmin[0]` are **live today at w=0** — not as wet
values, but as class 5's dry coefficients. Only entries 1..4 are dormant.

**Instead:** fixed in `35f6cd2`. The three track helps now say their twin is the
kerb back-apron's live value; the section comment records the blend
(`coef = (max*(0x4000-w) + min*w) >> 14` @ `0x1FB67`) and the aliasing.

**Note:** the vault caught this, not the plan and not me. The plan's shorthand
was a simplification that happened to be false at the boundary.

---

## 4. 🔴 The wing fields — immediates → Data, and 2 → 3 (Task 15)

**Plan:** `wing_dmg_loss | imm 0x3000 in CalcBothWings @0x16AA5` and
`wing_dmg_front_boost | imm 0x5000`, both to be added as `Target::Direct` after
verifying operand offsets.

**Reality:** CalcBothWings contains **no immediates at all**:

```
00016ABE  imul dword_0_D5EAC    ; 0x3000 — rear-wing damage      x0.75
00016ADE  imul dword_0_D5EB0    ; 0x5000 — front BOOST when rear damaged  x1.25
00016B01  imul dword_0_D5EB4    ; 0x3000 — front-wing damage     x0.75
```

Plain data dwords → `Target::Data`, removing the operand-offset guesswork
entirely. `physics-setup-apply.md:395` already documented `D5EAC` as
"rear-wing dmg".

**And there are three, not two.** `D5EB4` (front-wing damage) is absent from the
plan's table. Shipping two would leave front-wing damage uneditable while its
rear twin is exposed.

**Instead** (agreed with Roberto): all three as `Target::Data` —
`wing_dmg_loss`, `wing_dmg_front_boost`, `wing_dmg_front_loss`.

---

## 5. ⬜ Multi-site immediates — skipped (Task 15)

**Plan:** `freerev_cap` and `clutch_engage_rpm` as single `Target::Direct` fields.

**Problem:** each constant lives at **several sites that must move together**, and
`FieldDesc` holds **one** target.

```
00019F44  3D 38 4A 00 00   cmp eax, 19000   \
00019F49  7E 05            jle +5            }  clamp
00019F4B  B8 38 4A 00 00   mov eax, 19000   /
```

| field | sites | failure if only one is patched |
|---|---|---|
| `freerev_cap` | `0x92199`, `0x921A0` | patch the `mov` → values between old and new cap get **raised**; patch the `cmp` → cap stops matching its test |
| `clutch_engage_rpm` | `0x9249D` (gate), `0x924E5`+`0x924EC` (clamp) | gate desynchronises from clamp |

**Instead** (agreed with Roberto): **skipped both**, recorded in
`NEW-FIELD-CANDIDATES.md` as blocked on multi-target `FieldDesc` support. One
field per site would work mechanically but hands the user a silent footgun — the
exact shape that made T14–17 undiagnosable for years.

---

## 6. ✅⬜ The two D6010 skips — plan predicted correctly (Task 15)

**Plan:** "`limiter_bounce_base` — 15200 site in 19F51 — may re-read D6010; if
so, SKIP (covered by rev_limiter)… If the 15200 sites turn out to re-read D6010,
skip those two fields — that outcome is success, not failure."

**Confirmed: both re-read D6010. Neither has an immediate.**

```
00019F51  cmp eax, dword_0_D6010      ; limiter_bounce_base
00019F66  add eax, dword_0_D6010
0001A381  sub edx, dword_0_D6010      ; softcut_fade_base — "; edx := edx - 15200"
```

The "15200" was the **listing's comment** annotating D6010's value, not a
constant in the code. Both **skipped**; already covered by `rev_limiter`.

---

## 7. ✅🔴 `Target::Code`'s `+1` rule — wrong for 3 of 9 sites (Task 15)

The plan made Task 15 verify-first precisely because "the operand offset inside
the instruction varies by opcode, so `Target::Code` (+1) may be wrong". It was
right:

| site | bytes | operand | `+1` ok? |
|---|---|---|---|
| `practice_fuel_laps` @ `0x2C543` | `B8 0C 00 00 00` | +1 | ✓ |
| `upshift_block_rpm` @ `0x19E4A` | `66 81 3D 24 40 01 00 | 28 23` | **+7** (w=2) | ✗ |
| `reengage_rpm_lo` @ `0x1A1E7` | `81 3D 24 40 01 00 | 40 1F 00 00` | **+6** | ✗ |
| `reengage_rpm_hi` @ `0x1A1FE` | `81 3D 24 40 01 00 | 28 23 00 00` | **+6** | ✗ |
| `kerb_*` ×8 @ `0x73557…` | `C7 05 <disp32> | <imm32>` | **+6** | ✗ |

`cmp [mem], imm` encodes a disp32 before the immediate. `Target::Code` would have
read garbage. All 12 single-site offsets confirmed byte-for-byte before any field
existed; each records its instruction, IDA address and byte pattern in a comment.

---

## 8. 🔴 `steer_master_clamp` width (Task 13)

**Plan:** width `2?`. **Reality:** `000D61C4 dword_0_D61C4 dd 18E4h` → **width 4**.
The block is genuinely mixed (`D61C0 dw` / `D61C4 dd` / `D61C8 dw`), which is
presumably where the uncertainty came from.

**Why the listing check wasn't optional:** at `0xD61C4` the high word is zero, so
a **width-2 read also yields 6372** and `stock_values_match_real_exe` passes
either way. The harness cannot distinguish widths; only the data directive can.
Same trap resolved for `D6020`/`D6024`/`D602E` (`dw`) and `D56B0` (`dd`).

---

## 9. 🔧 Camber — verification method changed (Task 5)

Stock camber is **0**, which matches at *any* zeroed address — the byte test is
worthless here. Verified **structurally** instead: `dword_0_D5EC4 dd 0` read as
`mov eax, dword_0_D5EC4[ecx*4]` (`0x1A9F7`/`0x1AA28`/`0x1AA3F`), pinning base,
stride and width. Those readers sit inside the tyre-coefficient code, which
independently corroborates the plan's claim that non-zero camber wakes dormant
grip machinery. Reasoning recorded next to `camber_rl`.

---

## 10. ➕ Test fixtures 1.4 MB → 2 MB (Task 8)

Not in the plan. The synthetic `ExeImage` fixtures were `1_400_000` bytes; every
field so far happened to fit, so the limit was never exercised.
`engine_brake_pitch` (IDA `0x1731C0` → file `1926164`) is the first above it and
panicked the roundtrip tests outright:

```
ExeImage::write out of bounds: off=1926164 width=4 len=1400000
```

Raised to `2_000_000` (registry max end offset: 1929880, `idle_rpm`). Task 13's
`steer_base_lock` (file 1926200) would have hit the same wall. Committed
separately (`8ada2d8`) as test infrastructure, touching 7 files. Real EXE is
5702937 bytes and was never affected.

---

## 11. ➕ PARAMETER-GUIDE scope (Task 3)

**Plan (Task 3, step 5):** "update `docs/PARAMETER-GUIDE.md` if it lists tabs."

It listed tabs — and was **factually wrong** in two ways this task created:

1. Described the tow settings as "**Your slipstream (player only)**",
   contradicting the helps decoded in `6980585` — the AI wake reads the same
   constants.
2. Quick recipe: "**Turn on AI slipstreaming → raise AI Tow Strength from 0**."
   That field can only ever make the AI **brake harder**. The recipe told modders
   to do the opposite of what they wanted.

**Instead:** fixed both, flagged as scope the plan didn't authorise. Roberto
approved.

---

## 12. 🔧 Process deviations

- **Branch:** the executing-plans skill says never start on `main` without
  consent. Raised; Roberto committed the working tree and said proceed. Worked on
  `main`, matching how all prior feature work in this repo was committed. No
  remote exists, so nothing was ever pushed.
- **`cargo fmt`:** an early run reformatted 10 unrelated files — the repo is not
  rustfmt-clean. Reverted; additions hand-formatted thereafter. No stray churn
  committed.
- **GUI verification:** no `xdotool`, so the Physics sub-tabs couldn't be driven
  from here. Reported once per batch until Roberto pointed out that his "go
  ahead" already meant he'd eyeballed it. Stopped.
- **Task 0 spot-check exceeded:** the plan asked for 2–3 magic labels checked
  against Designer positions. Checked **all 15** (every one sits beside
  `textBoxN` where N = table number) **and** found the old editor's
  `SlotAddresses` slot-1 row is byte-identical to our `MAGIC_LAYOUT` bases for all
  24 tables. The ordinal assumption was correct.
- **Task 2 exceeded:** rather than only unit-testing `CodeData`, byte-verified all
  12 Task 14 coefficients at `CodeData` offsets immediately — de-risking Task 14
  twelve tasks early.
- **Harness proven, not assumed:** the plan warns the EXE test silently passes when
  the env var is unset. Confirmed it bites by corrupting a stock and watching it
  fail — on a `Data` field (`diff_lock`) and a `CodeData` field (`tyre_k3_rear`).

---

## 13. ➕ Out-of-plan work (Roberto-directed)

- **T14–17 pit-render RE detour** → `docs/MAGIC-T14-17-PIT-RENDER.md`. Established
  the 6-field record, A/B as a wedge with asymmetric jobs (B flips draw order, A
  gates the span nudge), conditionally-live offsets, the `.m2d` permutation, and
  the custom-track mechanism. **`MAGIC_LAYOUT` and the T14–17 help text are
  knowingly left wrong**, blocked on the `.m2d` format decision pending with the
  x86GP2 author (§9 of that doc).
- **Vault errata applied** (authorised): `physics-aero-damage.md` (C7A60 per-wheel
  + GP2Edit root cause), `magic-data.md` (Erratum 2), `weather-remnants.md`
  (static vs runtime patch; class-5 aliasing).
- **`D5D6C` recorded as a candidate:** the wet/dry blend is **statically
  patchable** — nothing writes it, so an EXE patch persists and GP2Lap is only
  needed to *vary* `w`. Not added: it's not in the plan and the class-5 aliasing
  means it needs its own design.
- **A superseded claim of mine**, recorded so it isn't re-derived: "A == B means
  no fallback, so degenerate slots are fragile" — wrong on both counts. B is not a
  fallback, and degeneracy is the norm (13/16 slots).

---

## 14. What the plan got right

Worth stating, since this doc is a list of errors:

- **Verify-first on Task 15 was the correct call** — the `+1` rule really was
  wrong for a third of the sites.
- **Both D6010 predictions landed exactly**, including "that outcome is success,
  not failure".
- **The width-caution rule (ground rule 3)** caught a real error
  (`steer_master_clamp`) that the test suite structurally could not.
- **"Never change field ids"** — honoured; verified by diffing every id and every
  target/stock against HEAD on the one task that moved fields (Task 3).
- **The stride-8 `(threshold, flag)` warning** was correct and is recorded in code.
- **~180 of ~185 planned values were right** on the first byte-check. The failures
  cluster entirely around one bad source binary.

---

## 15. Still owed

- **Nothing is tested in-game.** All 234 rows in `FIELD-TESTING.md` say
  `untested`. Byte-verification proves we write the right address — not that the
  help describes what a driver feels.
- **`MAGIC_LAYOUT`** still uses the community stride-6 permutation (blocked on the
  `.m2d` decision).
- **Multi-target `FieldDesc`** would unblock `freerev_cap` and
  `clutch_engage_rpm`.
- **The plan doc's verify commands** still name the patched mule.
- Remaining candidates: default setups, per-car tables, per-track tyre abrasion
  [16], ARB step tables [11×2], wet/dry blend `w`.
