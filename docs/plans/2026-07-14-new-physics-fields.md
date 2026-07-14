# New Physics Fields + AI Racecraft Regroup — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add every statically-patchable field from `docs/NEW-FIELD-CANDIDATES.md` to the editor and move the mislabeled "AI Follow" fields out of Slipstream into a new AI Racecraft sub-tab.

**Architecture:** All fields are `FieldDesc` entries in `crates/gp2ws-core/src/physics_fields.rs` (declarative registry; GUI renders automatically per `SubTab`). New sub-tab enum variants + one new `Target::CodeData` variant (data stored in the code segment, no +1 opcode skip). Every batch is verified against the real EXE by the existing `stock_values_match_real_exe` test.

**Tech Stack:** Rust, egui (gp2ws-gui), gp2ws-core registry. Reference RE docs live in `~/vaults/gp2/docs/` (`physics-*.md`); the annotated listing is `~/vaults/gp2/docs/archive/gp2_new_export_annotated.lst` (binary-ish: always `grep -a`).

---

## Ground rules (read before any task)

1. **Field ids are immutable** — they key the TOML import/export (`physics_io`). Moving/relabeling a field NEVER changes its `id`.
2. **Verification harness:** `GP2WS_TEST_EXE=/home/rremedio/GP2/GP2.EXE cargo test -p gp2ws-core` byte-verifies every field's `stock` against the real EXE. This is the failing-test step for each batch: add the fields with the RE-doc stock value → if address/width/stock disagree with the EXE, the test fails. Run it after EVERY batch.
3. **Width caution:** the RE docs don't always mark word vs dword. Before adding a field whose width is uncertain, check the data directive in the listing:
   `grep -a "D6020" ~/vaults/gp2/docs/archive/gp2_new_export_annotated.lst | head` (`dw` = width 2, `dd` = width 4). The stock-match test then confirms.
4. **Help-text style:** 2–4 plain-language sentences: what it does, which direction does what, who it affects (player/AI/both), end with `Stock N.`. Signed braking-style values state "keep it NEGATIVE" where relevant.
5. **Old-editor names:** any field that existed in Roberto's previous editor (gp2_slot_and_tyre_editor) under a different name must say `Old editor: "<name>".` in its help — including already-implemented fields and the relabeled AI fields. See Task 0.
6. **Testing status:** manual in-game testing of CURRENT fields is still incomplete. Every batch must update `docs/FIELD-TESTING.md` (created in Task 1) marking its fields `untested`. Batches are independent and individually committable so testing can proceed incrementally.
7. **Git:** commit per task with conventional-commit messages. NEVER push (repo policy — push only on explicit user request).

---

## Notes for the executor (read twice)

1. **The RE docs and the real EXE are the only sources of truth.** Never
   "correct" an address, stock value, or width from memory of GP2 modding
   lore — everything in this plan was reverse-engineered from the annotated
   listing and byte-verified. If something looks wrong, check the listing
   (`grep -a "<SYM>" ~/vaults/gp2/docs/archive/gp2_new_export_annotated.lst`),
   don't guess. Never open/Read that file whole — it is huge; grep only.
2. **The EXE verification test silently passes when the env var is unset.**
   `stock_values_match_real_exe` early-returns without `GP2WS_TEST_EXE`. A
   green `cargo test` without `GP2WS_TEST_EXE=/home/rremedio/GP2/GP2.EXE` has
   verified NOTHING about your new addresses. Run the env-var form after
   every batch and confirm the test actually executed (it fails loudly on a
   mismatch; if unsure, temporarily break one stock value and watch it fail).
3. **On a stock mismatch: stop, don't paper over.** Do NOT just edit `stock`
   to whatever the EXE returned — a wrong address can still read a plausible
   value. Re-derive the address and width from the listing first. Only after
   confirming the address is right may you correct `stock` (and then also
   fix `docs/NEW-FIELD-CANDIDATES.md` and note it in the commit message).
4. **Never change existing field `id`s, the order/meaning of `magic.rs`
   layout, or the TOML schema.** Labels and help are free; ids are wire
   format.
5. **Target variants are load-bearing:** `Data` = IDA+0x63254; `Code` =
   IDA+1+0x78254 (immediate after a 1-byte opcode); `CodeData` (Task 2) =
   IDA+0x78254 with NO +1; `Direct` = raw file offset (Task 15 only). Do not
   hand-convert IDA→file offsets into `Direct` for normal fields.
6. **Stocks in this plan are decimal decoded values.** Do not enter hex into
   `stock`; do not invent encodings — every new field is `Encoding::Raw`.
   `signed: true` ONLY where the tables mark it (camber, tyre_k1_*,
   damper_knee_neg).
7. **Task 15 is verify-first.** No field may be created until the operand's
   file offset has been confirmed by reading the actual stock bytes from
   GP2.EXE (e.g.
   `python3 -c "d=open('/home/rremedio/GP2/GP2.EXE','rb').read(); import struct; print(hex(struct.unpack_from('<I', d, OFFSET)[0]))"`).
   If the 15200 sites turn out to re-read D6010, skip those two fields —
   that outcome is success, not failure.
8. **Scope discipline.** Implement exactly the plan: no extra fields, no
   refactors of the registry, no UI redesign, no renaming beyond the tables,
   no touching unrelated help strings. If a table row seems wrong, flag it
   in the commit/summary rather than silently deviating.
9. **Every batch updates `docs/FIELD-TESTING.md`** (new fields = `untested`).
   Check the "Reset all to stock" GUI feature picks up new fields (it should
   iterate the registry — verify once in Task 4, then trust it).
10. **Git:** one commit per task, repo-local author config already set,
    NEVER push. Some tasks touch only docs — commit those too.
11. **Help strings are for modders, not for you.** Plain language, say which
    direction does what and who it affects (player/AI/both), end with
    `Stock N.`; include the exact `Old editor: "<name>".` sentences from the
    tables. Read three existing helps in `physics_fields.rs` first and match
    their voice.
12. When a width is marked uncertain (`2?`), resolve it via the listing's
    `dw`/`dd` directive BEFORE adding the field, not after a test failure.

---

## Task 0: Old-editor name cross-references — **RESOLVED** (source recovered)

Source of truth: `/media/rremedio/Roberto/csharp/GP2 Slot and Tyre Editor/` —
`GP2Addresses.cs` (address arrays) + `Form1.Designer.cs` (labels, paired to
`ptextBox1..52` by Designer positions; `PhysicsAddresses[i-1]` ↔ `ptextBoxi`).
Full physics mapping (verified by control coordinates):

| ptextBox | offset (dec) | Old editor name | Maps to (new/existing field) |
|---|---|---|---|
| 1 | 1282660 | Rev Limiter | `rev_limiter` (already credited) |
| 2 | 1282672 | Max RPM | `max_rpm` (already credited) |
| 3 | 1282820 | **Power Factor** | new `engine_force_scale` |
| 4–7 | 1284436–48 | RPM Lights 1–4 | `rpm_light_1..4` — add `Old editor: "RPM Lights N".` |
| 8 | 1282580 | **Differential Final Ratio — Factor 1** | new `gearing_base_1` |
| 9 | 1282584 | **Differential Final Ratio — Factor 2** | new `gearing_base_2` |
| 10 | 1282241 | **Upshift Penalty (Humans)** | new `shift_cut_player` |
| 11 | 1282245 | **Upshift Penalty (CCs)** | new `shift_cut_ai` |
| 12–47 | 1282824–94 | Power Curve 1–36 (bias 62082 = 0xF282, matches ours) | Power Curve tab (already same concept) |
| 48 | 1282300 | Downforce | `df_scale` (already credited) |
| 49 | 1279570 | **Breaking Force** | existing `brake_force` — add old-name ref |
| 50 | 1282276 | **Polar Moment of Inertia** | new `gyr_yaw` (the RE symbol is literally `d_pmi`!) |
| 51 | 1281904 | **Asphalt Acceleration 1** | new `surf_traction_track` |
| 52 | 1281924 | **Asphalt Acceleration 2** | dead wet-table twin — NOT ported; mention in `surf_traction_track` help |

Other tabs:

- **Misc "Human Grip"** (1281864 = IDA 0xD5CF4 = `t_gripmax[0]`) → new
  `surf_grip_track` gets `Old editor: "Human Grip".`
- **Misc "Overall CC Grip Level"** → existing `cc_grip_qual/race` (already credited).
- **Tire Types tab**: columns "Wear" / "Grip" / "Min" for compounds 52(A)–55(D).
  Maps to existing `tyre_grip_a..d` (old "Wear"), `tyre_base_a..d` (old
  "Grip"), `tyre_worn_floor` (old "Min") — add refs. NOTE: the old editor's
  Wear addresses (1282099+4i) are **one byte past** ours (1282098+4i =
  0xD5DDE) — the old editor edited the value's high byte / was off-by-one;
  ours is listing-verified. Mention nothing in help; keep this note here.
- **Magic Data tab** old labels, in T1..T24 order: Tire Wear, Slot Grip,
  Qual Grip 1, Race Grip 1, Unknown 5, Unknown 6, Qual Grip 2, Race Grip 2,
  Qual Timing, Race Timing, Semi-Pro Grip, Unknown 12, CC Mistake Rate,
  Unknown 14–18, Pit-in Distance, Pit-out Distance, Pit-in Speed, Human Fuel,
  CC Fuel, Unknown 24. Add `Old editor: "<name>".` to `MAGIC_LABELS` help in
  `crates/gp2ws-gui/src/magic_tab.rs` (skip the "Unknown N" ones — no value).
  Label→table pairing assumed ordinal; spot-check 2–3 against Designer
  positions before committing.

**Step 1:** Apply the old-name refs to the EXISTING fields listed above
(brake_force, rpm_light_1..4, tyre_* ×9, magic labels).
**Step 2:** `cargo test -p gp2ws-core -p gp2ws-gui` → PASS (data-only change).
**Step 3:** Commit: `docs(fields): add old-editor name cross-references`.

New fields pick up their old names from the batch tables below (already
annotated). Old-editor Misc features NOT in workshop scope (candidates for a
future plan): Simultaneous cars limit, Track Size Limit, Refuel patch,
Session Duration, Pit Stop Patch.

---

## Task 1: Sub-tab infrastructure + testing checklist doc

**Files:**
- Modify: `crates/gp2ws-core/src/field.rs` (SubTab enum, ~line 11)
- Modify: `crates/gp2ws-gui/src/physics_tab.rs` (SUBTABS const, ~line 17)
- Create: `docs/FIELD-TESTING.md`

**Step 1:** Extend the enum:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SubTab {
    Engine,
    PowerCurve,
    Drivetrain,
    Chassis,
    Aero,
    Brakes,
    Suspension,
    MassGrip,
    Tyres,
    Surfaces,
    Slipstream,
    AiRacecraft,
    Walls,
    Steering,
}
```

**Step 2:** Update the GUI list (order = tab order):

```rust
const SUBTABS: [(SubTab, &str); 14] = [
    (SubTab::Engine, "Engine"),
    (SubTab::PowerCurve, "Power Curve"),
    (SubTab::Drivetrain, "Drivetrain"),
    (SubTab::Chassis, "Chassis"),
    (SubTab::Aero, "Aero"),
    (SubTab::Brakes, "Brakes"),
    (SubTab::Suspension, "Suspension"),
    (SubTab::MassGrip, "Mass/Grip"),
    (SubTab::Tyres, "Tyres"),
    (SubTab::Surfaces, "Surfaces"),
    (SubTab::Slipstream, "Slipstream"),
    (SubTab::AiRacecraft, "AI Racecraft"),
    (SubTab::Walls, "Walls & Damage"),
    (SubTab::Steering, "Steering"),
];
```

14 tabs won't fit one row: wrap the selector in `ui.horizontal_wrapped(...)` if it isn't already.

**Step 3:** Create `docs/FIELD-TESTING.md` — a checklist table (field id | label | subtab | status) seeded with every CURRENT field marked with its present testing status (`untested` unless Roberto has confirmed otherwise — seed all as `untested` and let him upgrade), to be extended by every batch below.

**Step 4:** `cargo test -p gp2ws-core -p gp2ws-gui` → PASS (empty subtabs are fine). Run the GUI briefly (`cargo run -p gp2ws-gui`) to eyeball the wrapped tab row.

**Step 5:** Commit: `feat(gui): add Drivetrain/Chassis/Suspension/Surfaces/AI Racecraft/Walls/Steering sub-tabs + field testing checklist`.

---

## Task 2: `Target::CodeData` variant

Data constants stored inside the code segment (the tyre coefficient block 0x1A93F–0x1A9B6) need `file = IDA + CODE_BASE` with **no** +1.

**Files:**
- Modify: `crates/gp2ws-core/src/target.rs`

**Step 1 (failing test first):**

```rust
#[test]
fn code_data_no_opcode_skip() {
    // tyre k1 rear @ code-segment data 0x1A94F -> file 0x92BA3
    assert_eq!(Target::CodeData(0x1A94F).base_offset(), 0x1A94F + CODE_BASE);
}
```

Run: `cargo test -p gp2ws-core code_data` → FAIL (variant missing).

**Step 2:** Add the variant:

```rust
pub enum Target {
    Data(usize),     // IDA address of a data word
    Code(usize),     // IDA address of an instruction; operand is at +1
    CodeData(usize), // IDA address of DATA stored in the code segment (no +1)
    Direct(usize),   // already a file offset (legacy old-editor offsets)
}
// in base_offset():
Target::CodeData(ida) => ida + CODE_BASE,
```

**Step 3:** `cargo test -p gp2ws-core` → PASS.
**Step 4:** Commit: `feat(core): add Target::CodeData for data constants in the code segment`.

---

## Task 3: Move + relabel the AI Racecraft fields (no new fields, no new values to test)

**Files:** `crates/gp2ws-core/src/physics_fields.rs` (the 11 fields `ai_tow_strength`, `ai_follow_base_1..7`, `ai_follow_floor_1..3`)

**Step 1:** For all 11: `subtab: SubTab::Slipstream` → `SubTab::AiRacecraft`. Keep ids.
**Step 2:** Relabel + add old-name refs (help bodies already decoded 2026-07-14 — keep them, adjust the first sentence where the old label is now the "old name"):

| id | new label | help must include |
|---|---|---|
| ai_tow_strength | AI Speed-Scaled Braking | `Old editor: "AI Tow Strength".` |
| ai_follow_base_1 | AI Avoidance Engage | `Old editor: "AI Follow Base 1".` |
| ai_follow_base_2 | AI Leader-Decel Match | `Old editor: "AI Follow Base 2".` |
| ai_follow_base_3 | AI Close-Follow Select | `Old editor: "AI Follow Base 3".` |
| ai_follow_base_4 | AI Close-Follow (Damaged) | `Old editor: "AI Follow Base 4".` |
| ai_follow_base_5 | AI Brake Cap: Hold-Back | `Old editor: "AI Follow Base 5".` |
| ai_follow_base_6 | AI Brake Cap: Sliding | `Old editor: "AI Follow Base 6".` |
| ai_follow_base_7 | AI Brake Cap: Corner Squeeze | `Old editor: "AI Follow Base 7".` |
| ai_follow_floor_1 | AI Heavy-Braking Flag | `Old editor: "AI Follow Floor 1".` |
| ai_follow_floor_2 | AI Max Braking / Tick | `Old editor: "AI Follow Floor 2".` |
| ai_follow_floor_3 | AI Avoidance Clamp | `Old editor: "AI Follow Floor 3".` |

Since the "NOT slipstream" preamble is redundant inside an AI Racecraft tab, rephrase to "Part of the AI's traffic-braking controller (System C — not the slipstream)."

**Step 3:** Consider moving `ai_brake_strength` (Brakes tab) here too — DECISION: leave it in Brakes (users know it there); add a cross-mention in its help.
**Step 4:** `GP2WS_TEST_EXE=/home/rremedio/GP2/GP2.EXE cargo test -p gp2ws-core -p gp2ws-gui` → PASS. GUI smoke-check both tabs.
**Step 5:** Update `docs/FIELD-TESTING.md` (fields moved, testing status unchanged) and `docs/PARAMETER-GUIDE.md` if it lists tabs.
**Step 6:** Commit: `refactor(fields): move AI traffic-braking fields to AI Racecraft tab with decoded labels`.

---

## Field-batch task template (Tasks 4–14)

Every batch below follows the same steps — they are not repeated per task:

1. **Verify widths** for any symbol not clearly `dd`/`dw` in the RE doc: grep the listing (`grep -a "<SYM>" ~/vaults/gp2/docs/archive/gp2_new_export_annotated.lst | head`).
2. **Add the `FieldDesc` entries** from the batch table to `PHYSICS_FIELDS` (keep the file's per-subtab comment sections). One fully-worked example:

```rust
    // ---- Drivetrain ----
    FieldDesc {
        id: "diff_lock",
        label: "Rear Diff Lock",
        help: "The rear differential's viscous coupling - how strongly the two \
               rear wheels are pulled to the same speed. 0 behaves like a fully \
               open diff (inside wheel spins up easily); higher values act like \
               a locked diff / spool (more traction, more understeer on power). \
               Stock 24576 (x1.5).",
        subtab: SubTab::Drivetrain,
        tier: Tier::Basic,
        target: Target::Data(0xD53C4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 24576,
        range: None,
    },
```

3. **Run the EXE verification:** `GP2WS_TEST_EXE=/home/rremedio/GP2/GP2.EXE cargo test -p gp2ws-core` → the batch's stock values must match the real EXE. A mismatch means wrong address/width/stock — fix against the listing before proceeding, and correct `docs/NEW-FIELD-CANDIDATES.md` if the doc value was wrong.
4. `cargo test -p gp2ws-gui` + GUI smoke-check the sub-tab.
5. Append the batch's fields to `docs/FIELD-TESTING.md` as `untested`.
6. Commit: `feat(fields): add <batch> fields`.

Column key below: **W** = width bytes, **S** = signed, **T** = tier (B/A). Encoding is `Raw` unless stated. Help column is the gist — write full help per style rule 4.

---

## Task 4: Drivetrain batch (top priority)

| id | label | target | W | S | stock | T | help gist |
|---|---|---|---|---|---|---|---|
| diff_lock | Rear Diff Lock | Data 0xD53C4 | 4 | n | 24576 | B | see example above |
| final_drive | Final Drive Divisor | Data 0xD5FC8 | 4 | n | 111522 | B | higher = longer gearing (fewer RPM per speed) in every gear; the knob to re-center the gear-ratio range after RPM mods |
| gearing_base_1 | Gearing Base 1 | Data 0xD5FC0 | 4 | n | 304 | A | speed↔rpm conversion input; only the ratio of the three bases matters. `Old editor: "Differential Final Ratio - Factor 1".` |
| gearing_base_2 | Gearing Base 2 | Data 0xD5FC4 | 4 | n | 1728 | A | same; higher = shorter gearing. `Old editor: "Differential Final Ratio - Factor 2".` |
| shift_cut_player | Shift Cut Duration | Data 0xD5E6C | 4 | n | 2560 | B | engine power-cut time on every gear change (player). `Old editor: "Upshift Penalty (Humans)" (single-byte edit at 1390C1).` |
| shift_cut_ai | Shift Cut Duration (AI) | Data 0xD5E70 | 4 | n | 4096 | B | AI twin. `Old editor: "Upshift Penalty (CCs)".` |
| downshift_guard | Downshift Over-Rev Guard | Data 0xD6020 | 2 | n | 14800 | A | auto-shift refuses downshifts that would exceed this RPM |
| downshift_margin | Downshift Table Margin | Data 0xD6024 | 2 | n | 800 | A | RPM margin in the auto-downshift table build |
| min_upshift_speed | Min Auto-Upshift Speed | Data 0xD602E | 2 | n | 4608 | A | speed floor for auto upshifts |
| clutch_blend_gain | Clutch Engagement Gain | Data 0xD5408 | 4 | n | 262144 | A | analog-clutch force blend |
| clutch_rpm_lag | Clutch RPM Lag | Data 0xD6036 | 4 | n | 32768 | A | rev-matching rate with analog clutch |
| clutch_slip_decay | Clutch Slip Decay | Data 0xD6032 | 4 | n | 4096 | A | how fast slipping revs settle to wheel RPM |
| engine_spin_down | Engine-Off Spin-Down | Data 0xD603A | 4 | n | 4000 | A | rev decay rate with the engine off |
| spin_gain_driven_rl/rr/fl/fr | Wheelspin Gain (RL/RR/FL/FR) | Data 0xD5318+4i | 4 | n | 98304/98304/131072/131072 | A | how fast driven wheels light up under power |
| spin_gain_lock_rl/rr/fl/fr | Lock-Up Gain (RL/RR/FL/FR) | Data 0xD5328+4i | 4 | n | 98304/98304/131072/131072 | A | braking lock-up aggressiveness |
| slip_decay_rl/rr/fl/fr | Slip Decay Gain (RL/RR/FL/FR) | Data 0xD5338+4i | 4 | n | 1310720/1310720/2228224/2228224 | A | recovery rate from spin/lock (effective wheel inertia) |

---

## Task 5: Chassis & Geometry batch

| id | label | target | W | S | stock | T | help gist |
|---|---|---|---|---|---|---|---|
| cog_rear_arm | CoG → Rear Axle | Data 0xD5E50 | 4 | n | 65955 | B | rear moment arm; with the front arm sets wheelbase AND weight distribution (lever rule) — warn: three effects |
| cog_front_arm | CoG → Front Axle | Data 0xD5E54 | 4 | n | 94928 | B | front moment arm; also feeds the steering-assist bicycle model |
| track_rear | Rear Track Width | Data 0xD5E58 | 4 | n | 86596 | B | |
| track_front | Front Track Width | Data 0xD5E5C | 4 | n | 90113 | B | |
| cog_height | CoG Height | Data 0xD5E64 | 4 | n | 13438 | B | baseline pitch/roll torque arm — the classic "lower CoG" knob |
| gyr_yaw | Yaw Gyration Radius | Data 0xD5E90 | 4 | n | 247 | B | → yaw inertia; lower = pointier. `Old editor: "Polar Moment of Inertia".` |
| gyr_pitch | Pitch Gyration Radius | Data 0xD5E94 | 4 | n | 220 | A | |
| gyr_roll | Roll Gyration Radius | Data 0xD5E98 | 4 | n | 104 | A | |
| unsprung_rear | Rear Unsprung Mass | Data 0xD5E88 | 4 | n | 55 | A | per wheel |
| unsprung_front | Front Unsprung Mass | Data 0xD5E84 | 4 | n | 44 | A | |
| inertia_fuel | Inertia Reference Fuel | Data 0xD5C3A | 4 | n | 60000 | A | nominal fuel in the inertia mass only |
| camber_rl/rr/fl/fr | Camber (RL/RR/FL/FR) | Data 0xD5EC4+4i | 4 | **y** | 0 | A | DORMANT: static camber activates real grip machinery; sign = direction; help must say it's a hidden stock-zero feature |

All are session-init inputs (derived-params init reads them at track entry) — note in a shared help sentence.

---

## Task 6: Walls & Damage batch

| id | label | target | W | S | stock | T | help gist |
|---|---|---|---|---|---|---|---|
| wall_restitution | Wall Restitution | Data 0xC6A2C | 4 | n | 4096 | B | bounce-back off barriers (stock = 0.25) |
| wall_friction | Wall Friction | Data 0xC6A30 | 4 | n | 14848 | B | along-wall speed retention (the scrape) |
| wall_yaw_gain | Wall Yaw-Kick Gain | Data 0xC6A34 | 4 | n | 131072 | A | how much a clip spins you |
| wall_yaw_clamp | Wall Yaw-Kick Clamp | Data 0xC6A3C | 4 | n | 6144 | A | |
| engine_kill_threshold | Engine-Kill Impact | Data 0xCBD24 | 2 | n | 7424 | B | hit harder ⇒ engine stops; raise for forgiving walls |
| damage_load_floor | Damage Load Floor | Data 0xC7A60 | 4 | n | 917504 | A | min corner load before any damage roll |
| spring_break_rl/rr | Spring Break Load (Rear) | Data 0xC7A70+4i (i=0,1) | 4 | n | 118095872 | A | |
| spring_break_fl/fr | Spring Break Load (Front) | Data 0xC7A70+4i (i=2,3) | 4 | n | 118030336 | A | |
| dmg_thr_a_rl/rr/fl/fr | Damage Threshold A (…) | Data 0xC7AA0+8i | 4 | n | 1835008/1835008/1703936/1703936 | A | per-wheel load→damage-bit thresholds; do NOT touch the interleaved flag dwords |
| dmg_thr_b_… | Damage Threshold B (…) | Data 0xC7AC0+8i | 4 | n | 2097152/2097152/1835008/1835008 | A | |
| dmg_thr_c_… | Damage Threshold C (…) | Data 0xC7A80+8i | 4 | n | 2359296/2359296/1048576/1048576 | A | |
| dmg_thr_d_… | Damage Threshold D (…) | Data 0xC7AE0+8i | 4 | n | 3145728/3145728/1310720/1310720 | A | |
| damage_probability | Damage Probability | Data 0xC7B00 | 4 | n | 256 | A | 256 = always; lower ⇒ probabilistic damage |
| spring_break_drop | Broken-Spring Ride Drop | Data 0xD55E8 | 4 | n | 32768 | A | |

Threshold tables are (threshold, flag) dword pairs at stride 8 — fields target ONLY the thresholds (+0/+8/+16/+24); verify against `physics-aero-damage.md` §B7 values during step 3.

---

## Task 7: Surfaces batch

Table layout (all dwords): gripmax @0xD5CF4+4c, accmax @0xD5D1C+4c, roughmax @0xD5D44+4c, class c = 0 track, 1 kerb-low, 2 kerb-high, 3 grass, 4 gravel. The wet (`*min`) tables are dead (blend constant 0) — do NOT port; note in the sub-tab header text if easy, else in each help.

| id pattern | labels | stocks (c=0..4) | T |
|---|---|---|---|
| surf_grip_{track,kerb_low,kerb_high,grass,gravel} | Grip: Track/Low Kerb/High Kerb/Grass/Gravel | 16384,16384,16384,10240,12288 | B (grass/gravel), A (rest); track one: `Old editor: "Human Grip" (Misc tab).` |
| surf_traction_{…} | Traction: … | 16384,1024,1024,16384,16384 | B (kerbs), A (rest); track one: `Old editor: "Asphalt Acceleration 1" ("Asphalt Acceleration 2" was the dead wet twin, not ported).` |
| surf_rough_{…} | Roughness: … | 256,256,256,2048,12288 | A |
| bump_track_scale | Bump Amplitude: Track | Data 0xD7E24, stock 65536 | A |
| bump_grass | Bump Amplitude: Grass | Data 0xD7E1C, stock 786432 | A |
| bump_gravel | Bump Amplitude: Gravel | Data 0xD7E20, stock 1310720 | A |

All width 4, unsigned.

---

## Task 8: Engine additions batch (Engine sub-tab)

| id | label | target | W | S | stock | T | help gist |
|---|---|---|---|---|---|---|---|
| engine_force_scale | Engine Force Scale | Data 0xD60B0 | 4 | n | 15728 | B | global power multiplier independent of curve & per-car HP. `Old editor: "Power Factor".` |
| engine_braking | Engine Braking | Data 0xD5FE4 | 4 | n | 2560 | B | off-throttle retardation fraction |
| engine_brake_pitch | Engine-Brake Pitch Factor | Data 0x1731C0 | 4 | n | 3072 | A | anti-squat quirk |
| idle_rpm | Idle RPM | Data 0x174040 | 4 | n | 3712 | A | idle target (±128 jitter) |
| misfire_probability | Misfire Probability | Data 0xD56B0 | 4 | n | 128 | A | failure-mode misfire rate (0–255 scale) — verify width in listing first |

---

## Task 9: Fuel batch (Mass/Grip sub-tab)

| id | label | target | W | S | stock | T |
|---|---|---|---|---|---|---|
| fuel_burn_base | Fuel Burn Base | Data 0xD57DC | 4 | n | 2048 | B |
| fuel_weight_div | Fuel Weight Divisor | Data 0xD57CC | 4 | n | 563 | A |
| fuel_weight_mult | Fuel Weight Multiplier | Data 0xD57D4 | 4 | n | 437318 | A |
| qual_fuel_laps | Qualifying Fuel Laps | Data 0xD3550 | 4 | n | 4 | A |

Help: burn chains with magic T22/T23; weight pair sets the ≈776 fuel→lbs slope (~4.7 lbs/lap); qual laps gets +1 in code.

---

## Task 10: Tyres additions batch (Tyres sub-tab, data-segment only)

| id | label | target | W | S | stock | T |
|---|---|---|---|---|---|---|
| rear_lateral_blend | Rear Pure-Lateral Blend | Data 0xD5354 | 4 | n | 6144 | B |
| slip_prescale | Slip Sensitivity | Data 0xD5F5C | 4 | n | 682 | A |
| wear_rate_rl/rr | Tyre Wear Rate Rear (L/R) | Data 0xD5524+4i (i=0,1) | 4 | n | 640 | B |
| wear_rate_fl/fr | Tyre Wear Rate Front (L/R) | Data 0xD5524+4i (i=2,3) | 4 | n | 1024 | B |
| segment_grip_boost | Segment Grip Boost | Data 0xD5704 | 4 | n | 17408 | A |

`rear_lateral_blend` help: 0.375 pure + 0.625 combined — rear breakaway character.

---

## Task 11: Suspension batch

| id | label | target | W | S | stock | T |
|---|---|---|---|---|---|---|
| tyre_spring | Tyre Spring Rate | Data 0xD5508 | 4 | n | 24000 | B |
| tyre_spring_init_rl/rr/fl/fr | Tyre Spring (Init, ×120) | Data 0xD54E8+4i | 4 | n | 200 | A |
| tyre_damping_rl/rr/fl/fr | Tyre Damping | Data 0xD550C+4i | 4 | n | 256 | A |
| spring_factor | Spring Rate Scale | Data 0xD54C4 | 4 | n | 1966080 | B |
| arb_factor | Anti-Roll Bar Scale | Data 0xD54CC | 4 | n | 196608 | B |
| free_length_rl/rr | Suspension Travel Rear | Data 0xD5544+4i (0,1) | 4 | n | 283984 | A |
| free_length_fl/fr | Suspension Travel Front | Data 0xD5544+4i (2,3) | 4 | n | 278528 | A |
| bumpstop_rate_rl/rr/fl/fr | Bump-Stop Rate | Data 0xD5570+4i | 4 | n | 1000 | A |
| packer_cap_rl/rr | Packer Cap Rear | Data 0xD5590+4i (0,1) | 4 | n | 68800 | A |
| packer_cap_fl/fr | Packer Cap Front | Data 0xD5590+4i (2,3) | 4 | n | 34400 | A |
| droop_stiffness | Droop Stiffness Add | Data 0xD5540 | 4 | n | 9600 | A |
| bump_rebound_ratio | Bump/Rebound Ratio | Data 0xD55BC | 4 | n | 8192 | A |
| damper_knee_pos | Damper Knee (+) | Data 0xD55AC | 4 | n | 43690 | A |
| damper_knee_neg | Damper Knee (−) | Data 0xD55B0 | 4 | **y** | -43690 | A |
| bottoming_stiffness | Bottoming Stiffness | Data 0xD5584 | 4 | n | 240000 | A |
| plank_wear_rate | Plank Wear Rate | Data 0xD5588 | 4 | n | 4096 | A |
| heave_knee / pitch_knee / roll_knee | Soft-Limit Knee (…) | Data 0xD5624/0xD5620/0xD561C | 4 | n | 65536/134217728/134217728 | A |
| heave_gain / pitch_gain / roll_gain | Soft-Limit Gain (…) | Data 0xD5630/0xD562C/0xD5628 | 4 | n | 65536/16384/16384 | A |

`tyre_spring` + `tyre_spring_init_*` helps must cross-reference each other (both must move together: init × 120 must equal the substep rate). `damper_knee_*` helps: keep the pair mirrored (+/−).

---

## Task 12: Aero ground-effect batch (Aero sub-tab)

| id | label | target | W | S | stock | T |
|---|---|---|---|---|---|---|
| rake_reference | Reference Rake | Data 0xD5750 | 4 | n | 13760 | B |
| rake_sens_total | Rake Sensitivity (Total) | Data 0xD5EC0 | 4 | n | 3121 | A |
| rake_sens_split | Rake Sensitivity (Split) | Data 0xD5EB8 | 4 | n | 3121 | A |
| front_ride_sens | Front Ride Sensitivity | Data 0xD5EBC | 4 | n | 1561 | A |
| front_ride_ref | Front Ride Reference | Data 0xD5748 | 4 | n | 12040 | A |
| ge_clamp_rear | GE Ride Clamp Rear | Data 0xD55D4 | 4 | n | 86000 | A |
| ge_clamp_front | GE Ride Clamp Front | Data 0xD55D0 | 4 | n | 51600 | A |
| ge_master_scale | Ground-Effect Master | Data 0xD5EE8 | 4 | n | 16384 | B |

`ge_master_scale` help: dormant ×1.0 multiplier on the whole ride-height correction — a hidden global GE knob.

---

## Task 13: Steering batch

Verify widths in the listing first (the D6180–D61F4 block may be word-sized).

| id | label | target | W | S | stock | T |
|---|---|---|---|---|---|---|
| steer_master_clamp | Max Steering Lock | Data 0xD61C4 | 2? | n | 6372 | B |
| steer_base_lock | Manual Base Lock | Data 0x1731E4 | 4 | n | 2048 | A |
| tc_ramp_rate | Traction-Control Ramp | Data 0xC9722 | 2? | n | 4096 | A |

---

## Task 14: Tyre coefficient block (CodeData batch — needs Task 2)

Block at code-segment 0x1A93F–0x1A9B6; each pair = rear dword @addr, front dword @addr+4. All `Target::CodeData`, width 4.

| id | label | addr (rear/front) | S | stock r / f | T |
|---|---|---|---|---|---|
| tyre_k3_rear / _front | Base Lateral Grip R / F | 0x1A95F / 0x1A963 | n | 327616 / 251904 | B |
| tyre_k1_rear / _front | Load Sensitivity Slope R / F | 0x1A94F / 0x1A953 | **y** | -87327 / -117651 | A |
| tyre_k2_rear / _front | Load Sensitivity Base R / F | 0x1A957 / 0x1A95B | n | 33161 / 35307 | A |
| tyre_k5_rear / _front | Optimal Load R / F | 0x1A967 / 0x1A96B | n | 1064 / 972 | A |
| tyre_k6_rear / _front | Curve Shape Slope R / F | 0x1A977 / 0x1A97B | n | 18292 / 70249 | A |
| tyre_k7_rear / _front | Curve Shape Base R / F | 0x1A97F / 0x1A983 | n | 4000 / 6404 | A |

`tyre_k3_*` help: the front↔rear grip balance — the oversteer/understeer knob. Step 3 (EXE verification) is CRITICAL here — it proves the CodeData offsets are right before anything ships.

---

## Task 15: Code-immediate batch — offset verification REQUIRED first

These live as instruction immediates; the operand offset inside the instruction varies by opcode, so `Target::Code` (+1) may be wrong. For each: (a) grep the annotated listing for the instruction to get its IDA address and byte pattern; (b) compute the operand's file offset; (c) confirm by reading the stock bytes from GP2.EXE (python one-liner); (d) add as `Target::Direct(verified_file_offset)` with a comment recording instruction + IDA addr.

| id | label | site | stock | T |
|---|---|---|---|---|
| wing_dmg_loss | Wing Damage DF Loss | imm 0x3000 in CalcBothWings @0x16AA5 | 12288 | A |
| wing_dmg_front_boost | Wing Damage Front Boost | imm 0x5000 in CalcBothWings | 20480 | A |
| practice_fuel_laps | Practice Fuel Laps | imm 12 @0x2C543 | 12 | A |
| kerb_a_x1/x2/h1/h2 + kerb_b_x1/x2/h1/h2 | Kerb Profile Defaults (A/B) | imms @CompileLoadTrack ~0x73557 | 0x6E,0x12C,0x12,0x1A / 0xB2,0x164,0x0E,0x18 | A |
| upshift_block_rpm | Upshift Block Wheel-RPM | imm 9000 in 19DE4 | 9000 | A |
| reengage_rpm_hi / _lo | Shift Re-Engage RPM (1st / 2nd+) | imms 9000/8000 in 1A05F | 9000/8000 | A |
| clutch_engage_rpm | Clutch Engage Wheel-RPM | imm 3000 in 1A05F | 3000 | A |
| freerev_cap | Free-Rev Cap | imm 19000 in 19ECE | 19000 | A |
| limiter_bounce_base | Limiter Bounce Base | 15200 site in 19F51 — may re-read D6010; if so, SKIP (covered by rev_limiter) | 15200 | A |
| softcut_fade_base | Soft-Cut Fade Base | 15200 site in 1A359 — same check | 15200 | A |

These are the fields that make low-rev engines (e.g. 6500 rpm) possible — help texts should cross-reference Rev Limiter. Put the RPM ones in Engine, wings in Aero, kerbs in Surfaces, fuel in Mass/Grip.

---

## Task 16: Docs wrap-up

1. Update `docs/PARAMETER-GUIDE.md` with all new sub-tabs/fields (follow its existing format).
2. Update `docs/NEW-FIELD-CANDIDATES.md`: mark each shipped section "✅ implemented", keeping §10 (default setups), §11 per-car tables, per-track tyre abrasion, and ARB step tables as remaining candidates (each needs its own table-editor design — out of scope here).
3. Confirm `docs/FIELD-TESTING.md` lists every field (grep count vs registry count).
4. Commit: `docs: parameter guide + testing checklist for new physics fields`.

---

## Explicitly OUT of scope (future plans)

- Default-setup editor tab (48-byte records; display-only wing/gear/tyre bytes) — needs its own UI design.
- Per-car HP + per-car grip tables — per-car editor territory.
- Per-track tyre abrasion [16] and ARB step tables [11×2] — need slot/table UI like Magic Data.
- Making `ai_follow_floor_*` signed (behavior/format change; revisit with testing).
- Any change to field ids or the TOML schema.
