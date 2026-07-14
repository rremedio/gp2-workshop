# New editor field candidates — from the physics-tick RE (2026-07)

Gap analysis between the vault's `physics-*` docs (`~/vaults/gp2/docs/`) and the
fields the editor currently exposes (`physics_fields.rs`, `tyre_fields.rs`,
`power_curve.rs`, `magic.rs`). Everything listed is **statically patchable**
(data constant or code immediate) and absent from the current registries.

Key enabler: the derived-params init (`sub_0_17A52`) runs at session start and
reads *static* data constants — so chassis geometry, inertias, CoG, etc. are
patchable even though the tick consumes runtime-derived values.

## Address convention

**All addresses below are IDA virtual addresses** (same convention as the vault
RE docs), NOT file offsets.

- Data address → file offset = **IDA + 0x63254** (`Target::Data`)
- Code immediate → file offset = **IDA + 1 + 0x78254** (`Target::Code`; the +1
  skips the 1-byte opcode of `mov/add reg, imm32` — matches the existing wing
  fields)
- **Exception:** the tyre coefficient block at 0x1A93F–0x1A9B6 is *data stored
  in the code segment* — file offset = **IDA + 0x78254 with NO +1**. Needs a
  new target variant (or `Direct` with precomputed offsets).

Source docs per section are cited inline; verify each value against the EXE
bytes before wiring (the vault docs record verified stock values).

---

## 1. Chassis & Geometry — new sub-tab

From `physics-derived-params-init.md` §8.

| Field | Addr (IDA) | Stock | Meaning / caveat |
|---|---|---|---|
| CoG → rear axle distance | Data 0xD5E50 (u32) | 65955 | Rear moment arm. With the front arm this IS the wheelbase (D3F50 = sum) AND the static weight distribution (lever rule: front load fraction = rearArm/wheelbase). One knob, three effects — needs a help note. |
| CoG → front axle distance | Data 0xD5E54 (u32) | 94928 | Front moment arm. Also feeds the steering-assist bicycle model. |
| Rear track width | Data 0xD5E58 (u32) | 86596 | Halved into roll arms and wheel mounts. |
| Front track width | Data 0xD5E5C (u32) | 90113 | |
| CoG height | Data 0xD5E64 (u32) | 13438 | Baseline for pitch/roll torque arms (`1B97A` adds live suspension travel on top). The classic "lower CoG" knob. |
| Yaw gyration radius | Data 0xD5E90 (d_pmi) | 247 | → yaw inertia (r²·mass). Lower = pointier car. |
| Pitch gyration radius | Data 0xD5E94 | 220 | → pitch inertia. |
| Roll gyration radius | Data 0xD5E98 | 104 | → roll inertia. |
| Rear unsprung mass | Data 0xD5E88 | 0x37 | Per-wheel; feeds sprung mass and the axle ODE. |
| Front unsprung mass | Data 0xD5E84 | 0x2C | |
| Inertia reference fuel | Data 0xD5C3A | 60000 | Nominal fuel used only in the inertia mass (never live fuel). |
| **Per-wheel camber [4]** | Data 0xD5EC4 (4×i32) | 0 | **Dormant knob** — camber grip machinery (`1A9E7`) is fully wired but ships with camber = 0 and no runtime writer. A static patch activates real per-wheel camber grip effects. |

## 2. Differential & wheel dynamics — new sub-tab

From `physics-wheel-speed-update.md`.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| **Rear diff lock (viscous coupling)** | Data 0xD53C4 | 0x6000 (×1.5 Q14) | The only differential in GP2: speed-difference coupling pulling rear wheel speeds together. 0 ≈ open, higher = spool. Prime modder knob. |
| Driven wheelspin gain [4] | Data 0xD5318 | 0x18000×2, 0x20000×2 | How fast the rears light up under power (advanced). |
| Lock-up / undriven spin gain [4] | Data 0xD5328 | same | Brake lock-up aggressiveness (advanced). |
| Slip decay gain [4] | Data 0xD5338 | 0x140000×2, 0x220000×2 | Recovery rate from spin/lock — effective wheel inertia at the patch (advanced). |

Note: the load-based open-diff split in `1B3C3` (lightly-loaded wheel gets more
drive) is code logic, not a constant — D53C4 is the tunable part.

## 3. Engine & transmission — additions to Engine tab

From `physics-drivetrain.md` §14.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| **Engine force scale** | Data 0xD60B0 | 15728 | Global multiplier in `rEngPowerRpm` — overall power knob independent of curve and per-car HP. |
| **Engine braking** | Data 0xD5FE4 | 0xA00 (0.156) | Fraction of engine force applied as retardation off-throttle. Very feelable. |
| Engine-brake pitch factor | Data 0x1731C0 | 0xC00 | How much engine braking pitches the car (anti-squat quirk) — advanced. |
| Shift cut duration (player) | Data 0xD5E6C | 0xA00 | Gear-change power-cut time. |
| Shift cut duration (AI) | Data 0xD5E70 | 0x1000 | |
| Downshift over-rev guard | Data 0xD6020 (w_maxrpm) | 14800 | Auto-shift refuses downshifts that would exceed this. |
| Downshift table margin | Data 0xD6024 | 800 | RPM margin in the auto-downshift table build. |
| Min speed for auto upshift | Data 0xD602E | 0x1200 | |
| Idle RPM floor | Data 0x174040 | 3712 | Idle target (±128 jitter). |
| Misfire probability | Data 0xD56B0 | 0x80 (50%) | Failure-mode misfire rate. |
| **Gearing base constants** | Data 0xD5FC0 / 0xD5FC4 / 0xD5FC8 | 0x130 / 0x6C0 / 0x1B3A2 | Set the speed↔rpm conversion (D3548) — effectively final drive / wheel circumference. A single "final drive" field scaling D5FC8 is the friendly version. |
| Clutch engagement gain / rpm lag / slip decay / spin-down | Data 0xD5408 / 0xD6036 / 0xD6032 / 0xD603A | 0x40000 / 0x8000 / 0x1000 / 4000 | Analog-clutch feel set (advanced). |

Hard caps (20000 rev clamp, 17000 curve clamp, 15200-bounce range) are code
immediates — patchable via `Code` targets for a "hard rev ceiling" field.

## 4. Tyre model — additions to Tyres tab

From `physics-tyre-model.md`. The 14×2 rear/front coefficient block is at
**CODE 0x1A93F–0x1A9B6** (dwords, data-in-code-segment — see address
convention above). Modder-meaningful entries:

| Field | Addr (rear/front pair) | Stock | Meaning |
|---|---|---|---|
| Base lateral grip rear / front (k3) | 0x1A95F | 0x4FFC0 / 0x3D800 | Main front↔rear grip balance — "oversteer/understeer" knob. |
| Load sensitivity slope (k1) | 0x1A94F | −0x1551F / −0x1CB93 | How grip falls off with load. |
| Load sensitivity base (k2) | 0x1A957 | 0x8189 / 0x89EB | |
| Optimal load (k5) | 0x1A967 | 0x428 / 0x3CC | Load at which the grip bell peaks (peak = k5×256). |
| Curve shape slope / intercept (k6/k7) | 0x1A977 / 0x1A97F | per-axle | Peakiness of the slip curve. |
| Rear pure-lateral blend | Data 0xD5354 | 0x1800 (0.375) | Rear-only blend of pure vs combined slip — rear breakaway character. |
| Slip pre-scale | Data 0xD5F5C | 0x2AA | Global slip-angle sensitivity (advanced). |
| **Tyre wear rate rear / front** | Data 0xD5524 [4] | 0x280×2, 0x400×2 | Fronts wear 1.6× faster stock. Pairs with existing wear-sensitivity fields. |
| Per-track tyre abrasion [16] | Data 0xD5D9E (16×u32) | per slot | Currently "read-only" in the Tyres tab; 16-slot table fits the Magic-tab slot UI. |
| Segment grip-boost value | Data 0xD5704 | 0x4400 | Grip multiplier on segments with flag 0x40 (+6.25% stock). |

## 5. Suspension — new sub-tab

From `physics-suspension-substep.md` / `physics-setup-apply.md` /
`physics-chassis-ode.md`.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| Tyre spring rate | Data 0xD5508 (+ init copies D54E8[4] = 0xC8 each) | 24000 | Tyre carcass stiffness — two locations must move together. |
| Tyre damping | Data 0xD550C [4] | 0x100 | |
| Spring rate scale | Data 0xD54C4 (d_springfactor) | 0x1E0000 (×120/click) | Global scale on setup spring clicks. |
| ARB scale | Data 0xD54CC (d_antirollfacto) | 0x30000 (×12) | Global anti-roll-bar strength. |
| ARB step tables rear/front [11 each] | Data 0x177100 / 0x17712C | 0..1250 / 0..12500 | The values behind the setup-screen indices. |
| Suspension travel (free length) [4] | Data 0xD5544 | 0x45550×2, 0x44000×2 | Total travel before the limit. |
| Bump-stop rate [4] | Data 0xD5570 | 0x3E8 (→120000) | Travel-limit stiffness. |
| Packer travel caps [4] | Data 0xD5590 (t_packervaltab) | 0x10CC0×2, 0x8660×2 | Max packer effect per wheel. |
| Droop stiffness add | Data 0xD5540 | 0x2580 | Extra rate in droop. |
| Bump/rebound ratio | Data 0xD55BC | 0x2000 (0.5) | Bump = rebound × this. |
| Damper knee velocity | Data 0xD55AC / 0xD55B0 | ±0xAAAA | Slow↔fast damper crossover (advanced-setup mode). |
| Bottoming stiffness | Data 0xD5584 | 0x3A980 | Plank-contact spring. |
| Plank wear rate | Data 0xD5588 | 0x1000 | Speed-scaled plank abrasion. |
| Heave/pitch/roll soft-limit knees + gains | Data 0xD5624/0xD5620/0xD561C, 0xD5630/0xD562C/0xD5628 | various | Chassis-motion soft stops (advanced). |

## 6. Aero — ground effect / rake additions

From `physics-aero-damage.md` §A4 — the ground-effect model is entirely
untouched in the editor.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| **Reference rake** | Data 0xD5750 | 0x35C0 | Rake offset the downforce correction is centred on. |
| Rake sensitivity (total / split) | Data 0xD5EC0 / 0xD5EB8 | 3121 / 3121 | Cost/shift of downforce with rake deviation. |
| Front-ride sensitivity | Data 0xD5EBC (ref 0xD5748 = 0x2F08) | 1561 | Front ride height → downforce gain. |
| GE ride clamps rear/front | Data 0xD55D4 / 0xD55D0 | 0x14FF0 / 0xC990 | Ride heights where ground effect saturates. |
| GE master scale | Data 0xD5EE8 | 0x4000 (×1.0) | Shipped as a no-op — dormant global ground-effect multiplier. |
| Wing-damage downforce loss / front boost | Code imms in `CalcBothWings` @0x16AA5 | 0x3000 / 0x5000 | −25% damaged wing, +25% front compensation. |

## 7. Surfaces — new sub-tab

From `physics-surface-sampling.md` §2 — grip/traction/roughness for the 5
surface classes at Data 0xD5CF4 (contiguous dword tables
`t_gripmax/accmax/roughnmax`; wet `*min` tables are dead but static).

| Field | Stock | Meaning |
|---|---|---|
| Grass grip / gravel grip | 0x2800 / 0x3000 | Off-track punishment — popular mod target. |
| Kerb traction (low/high) | 0x400 | Why kerbs kill acceleration. |
| Grass/gravel roughness | 0x800 / 0x3000 | Rolling-resistance-like demand. |
| Grass/gravel bump amplitude | Data 0xD7E1C / 0xD7E20 | 0xC0000 / 0x140000 | Noise height off-track (track bump scale = 0xD7E24). |
| Kerb profile defaults (CA/CB widths+heights) | Code imms @CompileLoadTrack ~0x73557 | (0x6E,0x12C,0x12,0x1A / 0xB2,0x164,0x0E,0x18) | Default kerb shape when the track doesn't override. |

## 8. Walls & damage — new sub-tab

From `physics-walls.md` §9 and `physics-aero-damage.md` §B7.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| **Wall restitution** | Data 0xC6A2C | 0x1000 (0.25) | Bounce-back off barriers. |
| **Wall friction** | Data 0xC6A30 | 0x3A00 (0.906) | Along-wall speed retention (the scrape). |
| Wall yaw-kick gain / clamp | Data 0xC6A34 / 0xC6A3C | 0x20000 / 0x1800 | How much a clip spins you. |
| **Engine-kill impact threshold** | Data 0xCBD24 | 0x1D00 | Hit harder than this ⇒ engine stops. Raise = forgiving walls. |
| Suspension damage floor **[4]** | Data 0xC7A60 | {0x100000, 0x100000, 0xE0000, 0xE0000} | Min corner load before any damage roll. **Per wheel (RL,RR,FL,FR)**, read as `osCarDamageTrack[ecx*4]` @ 0x32B09 — not the scalar 0xE0000 this table used to claim. The scalar reading came from EXEs where the array is flattened to 0xE0000 ×4 (which makes indexed vs scalar indistinguishable by value); a pristine GP2.EXE has the rear/front split, matching every neighbouring table. |
| Spring-break thresholds [4] | Data 0xC7A70 | ~0x70A0000 | |
| Wheel/wing damage threshold tables | Data 0xC7AA0/0xC7AC0/0xC7A80/0xC7AE0 | pairs | Per-wheel load→damage-bit thresholds — "car fragility". |
| Damage probability | Data 0xC7B00 | 0x100 (always) | Lower ⇒ damage becomes probabilistic. |
| Broken-spring ride drop | Data 0xD55E8 | 0x8000 | |

## 9. Fuel — additions

From `physics-fuel.md`.

| Field | Addr | Stock | Meaning |
|---|---|---|---|
| **Fuel burn base** | Data 0xD57DC | 0x800 | Global consumption multiplier (chains with magic T22/T23). |
| Fuel weight (density) | Data 0xD57CC / 0xD57D4 | 563 / 437318 | → the ≈776 fuel→lbs slope; how heavy a full tank feels. |
| Qualifying fuel laps | Data 0xD3550 (d_QualFuelLaps) | 4 | +1 applied in code. |
| Practice fuel laps | Code imm @0x2C543 | 12 | |

## 10. Default setups — potential new tab

`physics-setup-apply.md` §13: the 48-byte setup records are static EXE data —
player defaults @0x1771B8, AI/standard @0x177158, per-car @0x177218. Editing
the AI record changes every AI car's springs, ride heights, dampers, brake
balance, ARBs and pit strategy. Caveat: wings/gears/tyre bytes (+0x00..+0x08)
are overwritten by track data at load — display-only. Full record layout is in
the vault doc §2.

## 11. Smaller / advanced

- **Max steering lock** — Data 0xD61C4 (0x18E4 master clamp), manual base lock
  Data 0x1731E4 (0x800).
- **Traction-control ramp rate** — Data 0xC9722 (0x1000).
- Per-car HP (`GetALsHorsepwr` table → car+0xA2) and per-car grip
  (word_1745E8 qual/race + word_CCA9E correction) — classic per-car editor
  territory, currently absent entirely.

## Do NOT add (runtime-set; static patch is a trap)

`D6046` (frame-time scale), `word_C7E54`/`C7E52` (per-track from track file),
`word_D44CC` (AI decel table — already solved via the builder-shift patch),
`d_acttrackGrip`, `w_tirefactor`, `D3548`, `d_fuelwghtfacto`, wheelbase
`D3F50`, inertias `D3FB0..` (patch the §1 inputs instead), `car+0x86`
(pit jack, not ride height).

## Corrections to existing Slipstream fields (Systems B and C)

From `slipstream.md` (incl. the 2026-07-10 AI-RE addendum),
`ai-mover-longitudinal.md` §4.8 and `ai-racecraft-infra-follow.md` §9.1.
Help strings updated in `physics_fields.rs` accordingly (2026-07-14).

GP2 has two unrelated "AI vs car ahead" systems, and the editor's Slipstream
tab used to conflate them:

**System B — the wake (drag cut).** The addendum superseded the old
"player-only" finding: `sub_0_288CA` writes the wake field `car+0x78` for
computer cars using the SAME constants as the player path, and the AI mover
consumes it in its drag term. So **Tow Strength (D53DC), Tow Reach (C9750),
Tow Align Width (C9748) and Tow Max Wake (C9752) affect BOTH player and AI.**
Tow Max Range (C9766) / Tow Min Speed (D5D96) gate the player's own car scan;
the AI wake path selects its target differently.

**System C — `sub_0_223CE`, the traffic-BRAKING controller.** Not slipstream:
its output is consumed only as a CAP on the AI's per-tick speed delta —
"never as a boost" (racecraft doc). The "AI Follow" fields are its bases
(runtime value = base × D6046, recomputed at session init):

| Field | Base → runtime | Actual role |
|---|---|---|
| AI Follow Base 1 (C96CA→C96EE) | −4096 | Avoidance engage threshold (at/above ⇒ ignore car ahead) |
| AI Follow Base 2 (C96CC→C96F0) | −3072 | Leader-decel matching threshold (leader braking harder ⇒ copy his decel) |
| AI Follow Base 3 (C96D2→C96F6) | −24576 | Close-follow selector: energy model vs gap servo |
| AI Follow Base 4 (C96D4→C96F8) | −1024 | Same selector, damaged/out-of-shape branch |
| AI Follow Base 5 (C96D8→C96FC) | −2048 | Per-tick brake ceiling: hold-back / yellow-flag state (65 b1) |
| AI Follow Base 6 (C96DA→C96FE) | −4096 | Per-tick brake ceiling: sliding state (64 b1) |
| AI Follow Base 7 (C96DC→C9700) | −512 | Per-tick brake ceiling: corner-squeeze state (64 b7 — "can't steer around him, brake") |
| AI Follow Floor 1 (D5FE8→C7D20) | −1024 | Heavy-braking status-flag threshold |
| AI Follow Floor 2 (D5FEC→C7D22) | −12288 | Global max braking per tick for all follow caps |
| AI Follow Floor 3 (D5FF0→C7D24) | +20480 | Clamp on the avoidance metric (gap/closing-speed servo) |

Base 5–7 are signed NEGATIVE deceleration ceilings; pushed toward 0/positive
they become acceleration limits whenever their state flag fires — which
presents as a mysterious "AI max speed" change (the effect observed in the
legacy editor).

**"AI Tow Strength" (D5FF4) is mislabeled.** The old slipstream.md §4 called
it a dormant speed-scaled slingshot; the newer AI RE supersedes that: all
four read sites scale System C's braking/avoidance caps with speed, so >0
makes the AI brake HARDER for traffic at speed — it can never add speed. The
real AI slipstream is System B / Tow Strength. Candidate follow-up: rename
the field (e.g. "AI speed-scaled braking") or drop it; consider moving the
System C fields out of Slipstream into an "AI Racecraft braking" group,
next to AI Brake Strength. Note the three "floors" are stored/edited as u32
but are semantically signed s16 bases.

## Legacy editor cross-reference

Mapping of the decimal file offsets used in Roberto's old editor
(gp2_slot_and_tyre_editor; source recovered 2026-07-14 from
`/media/rremedio/Roberto/csharp/GP2 Slot and Tyre Editor/` — names from
`Form1.Designer.cs`, addresses from `GP2Addresses.cs`) to the RE
identifications (IDA = file − 0x63254). Every entry resolved.

| Old editor (dec) | Old editor name | IDA | RE identification |
|---|---|---|---|
| 1282660 | Rev Limiter | 0xD6010 | Soft power-cut RPM (15200) — in editor ("Rev Limiter") |
| 1282672 | Max RPM | 0xD601C | Auto-shift RPM (15000) — in editor ("Max RPM") |
| 1282820 | Power Factor | 0xD60B0 | Engine force scale (15728) — `rEngPowerRpm` multiplier |
| 1284436–1284448 | RPM Lights 1–4 | 0xD6700–0xD670C | Cockpit shift-light RPMs (display-only) — in editor |
| 1282580 | Differential Final Ratio — Factor 1 | 0xD5FC0 | Gearing base constant (0x130) — see §3 |
| 1282584 | Differential Final Ratio — Factor 2 | 0xD5FC4 | Gearing base constant (0x6C0) |
| (not in old editor) | — | 0xD5FC8 | Third gearing constant (0x1B3A2) — file 1282588 |
| 1282241 | Upshift Penalty (Humans) | 0xD5E6D | Byte 1 of D5E6C — player shift-cut duration |
| 1282245 | Upshift Penalty (CCs) | 0xD5E71 | Byte 1 of D5E70 — AI shift-cut duration |
| 1282824–1282894 | Power Curve 1–36 | 0xD60B4–0xD60FA | `t_Engrpm` power curve, bias 62082 = 0xF282 — in editor (Power Curve tab) |
| 1282300 | Downforce | 0xD5EA8 | `d_wingfactor` (44369) — in editor ("Downforce Scale") |
| 1279570 | Breaking Force | 0xD53FE | Byte 2 of D53FC — brake force scale — in editor as full dword ("Brake Force") |
| 1282276 | Polar Moment of Inertia | 0xD5E90 | `d_pmi` = yaw gyration radius (247) → yaw inertia (§1 candidate) — the annotator's symbol agrees with the old name |
| 1281904 | Asphalt Acceleration 1 | 0xD5D1C | `t_accmax[0]` — track-surface traction coefficient (0x4000); scales driven wheelspin via `ws[0xD4]` |
| 1281924 | Asphalt Acceleration 2 | 0xD5D30 | `t_accmin[0]` — WET track traction. **Placebo**: the wet/dry blend `D5D6C` is constant 0 with no writer, so `*min` tables never blend in |
| 1281864 | Human Grip (Misc tab) | 0xD5CF4 | `t_gripmax[0]` — track-surface grip (§7 candidate `surf_grip_track`) |
| 1282099+4i | Tire Wear (A–D) | ~0xD5DDE | old editor was one byte past the field start; ours (0xD5DDE+4i, file 1282098+4i) is listing-verified — in editor ("Tyre Wear Sensitivity A–D") |
| 1282114+4i | Tire Grip (A–D) | 0xD5DEE | in editor ("Tyre Base Grip A–D") |
| 1282130 | Min (worn tyre) | 0xD5DFE | in editor ("Worn Tyre Floor") |

Notes:
- Three legacy fields patched odd single bytes inside dwords (0x1390C1,
  0x1390C5, 0x138652 — the lone non-zero byte of each stock value). The
  workshop should define these at the dword starts (0xD5E6C, 0xD5E70,
  0xD53FC).
- The four shift-light RPMs are the only entries absent from the physics RE
  docs (cockpit display, not physics).

## Suggested priority (modder value)

1. Rear diff lock (D53C4)
2. Engine braking (D5FE4)
3. CoG height + arms + gyration radii (§1 block)
4. Wall restitution/friction + engine-kill threshold
5. Surface grip table (§7)
6. Engine force scale (D60B0)
7. Tyre k3/k1/k5 balance knobs
8. Fuel burn base (D57DC)
9. Default-setup editor (§10)
10. Dormant camber array (D5EC4) — the "hidden feature" of the batch
