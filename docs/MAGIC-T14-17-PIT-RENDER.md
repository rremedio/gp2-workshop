# Magic tables 14–17 — the pit-area render record

**Status:** reference / RE findings. **No code changes have been made from this
document.** It exists to inform (a) the `.m2d` format decision currently pending
with the x86GP2 author, and (b) a future fix to `MAGIC_LAYOUT` + the T14–17 help
text, both of which are blocked on that decision.

**Date:** 2026-07-14. **Verified against:** pristine GP2.EXE
(`~/vaults/gp2/gp2/GP2.EXE`, byte-identical to the Jul 19 1996 copy on the
external drive) and the annotated listing
(`~/vaults/gp2/docs/archive/gp2_new_export_annotated.lst`).

Confidence is labelled throughout: **VERIFIED** (read from the instructions),
**INFERRED** (reasoned from surrounding code + the stock data), **SPECULATIVE**.

---

## 1. Summary of what changes

1. Community "tables 14/15/16/17" are **not four tables**. The block is
   **16 slots × one 12-byte record = 6 fields per slot**, read at stride 12 by
   `sub_0_4C1CF`. (VERIFIED — this was already in `magic-data.md`.)
2. The four angle fields per slot are **not** four independent knobs: they are
   **two wedges** (entry and exit), each bounded by two edges A and B, and
   **A and B do different jobs** — see §3. This corrects a "bias A is tried
   first, B is the fallback" reading.
3. The two remaining fields are **segment offsets** that no editor has ever
   exposed. They are **conditionally live**: inert on any end where A == B.
4. The consumers are **draw-order** (painter's algorithm) selection, **not**
   visibility/culling and **not** pit driving.
5. `.m2d` carries all 64 angle words **losslessly but permuted across slots**,
   and carries **none** of the 32 offset words.

---

## 2. The record

Block `IDA 0xD5A7A .. 0xD5B39` (file `+0x63254` → `1281230`), stride `0x0C`,
16 season slots, **192 bytes = 96 words**: 64 angle words + 32 offset words.

| off | field | type | → global | role |
|---|---|---|---|---|
| +0 | entry angle bias **A** | signed, `0x10000` = 360° | `CAA94` | turns the span nudge on |
| +2 | entry angle bias **B** | signed | `CAA96` | **flips the draw order** |
| +4 | entry segment offset | **unsigned**, ×108, no clamp | `CAA8C` | trims the overlap span forward |
| +6 | exit angle bias **A** | signed | `CAA98` | turns the span nudge on |
| +8 | exit angle bias **B** | signed | `CAA9A` | **flips the draw order** |
| +10 | exit segment offset | **unsigned**, ×108, no clamp | `CAA90` | trims the overlap span backward |

Reader (VERIFIED), `sub_0_4C1CF` @ `0x4C27A`:

```
0004C27A mov   ecx, d_actrack0to0F   ; SEASON SLOT index 0..0xF (not track id)
0004C285 mul   ecx                   ; stride 12
0004C289 mov   ax, [ecx+0D5A7Ah] -> word_0_CAA94        ; +0
0004C296 mov   ax, [ecx+0D5A7Ch] -> word_0_CAA96        ; +2
0004C2A3 movzx eax, word [ecx+0D5A7Eh] ; mul 6Ch -> CAA8C  ; +4  UNSIGNED, x108
0004C2B6 mov   ax, [ecx+0D5A80h] -> word_0_CAA98        ; +6
0004C2C3 mov   ax, [ecx+0D5A82h] -> word_0_CAA9A        ; +8
0004C2D0 movzx eax, word [ecx+0D5A84h] ; mul 6Ch -> CAA90  ; +10
```

Note `movzx`: the offsets are **unsigned and unchecked at read time**.

### Stock values (signed; from the pristine EXE)

| # | slot | entry A | entry B | entry off | exit A | exit B | exit off |
|---|---|---|---|---|---|---|---|
| 1 | Brazil | 1792 | −512 | 4 | −2176 | 2048 | 4 |
| 2 | Pacific/Aida | −1088 | 2816 | 4 | 256 | 256 | 4 |
| 3 | Imola | −4096 | 8192 | 4 | 1536 | 1536 | 4 |
| 4 | Monaco | 7680 | 7680 | 4 | −196 | −196 | 4 |
| 5 | Barcelona | −2207 | 2048 | 4 | 512 | 512 | 4 |
| 6 | Montreal | 4096 | 4096 | 4 | −1792 | −1792 | 4 |
| 7 | Magny-Cours | 6616 | 6616 | 4 | −2560 | −2560 | 4 |
| 8 | Silverstone | −598 | 6144 | 4 | 0 | −8192 | 4 |
| 9 | Hockenheim | −608 | −608 | 4 | 455 | −4992 | 4 |
| 10 | Hungaroring | −128 | 2048 | **3** | 1536 | 1536 | 4 |
| 11 | Spa | 119 | 119 | 4 | 1227 | −8192 | **8** |
| 12 | Monza | −276 | 0 | 4 | 1792 | 1792 | 4 |
| 13 | **Estoril** | −2720 | −2720 | 4 | 1280 | 1280 | 4 |
| 14 | Jerez | 367 | −15249 | 4 | 0 | 0 | 4 |
| 15 | Suzuka | 172 | 8192 | 4 | 1864 | 136 | 4 |
| 16 | Adelaide | −2368 | 14848 | 4 | 1536 | 1536 | 4 |

---

## 3. How the angles work (VERIFIED)

Both tests in `sub_0_51094` use the **same pivot** and the **same heading**;
only the bias differs. Two lines through one point at two angles = an angular
sector. The test itself is a signed half-plane (side-of-line) projection:

```
P = cos(θ + bias)·(Xcam − Xref) − sin(θ + bias)·(Ycam − Yref)
```

```
00051121 js    loc_0_5112F          ; P_A <  0 -> test B
00051123 mov   byte_0_CAA88, 0FFh   ; P_A >= 0 -> order FF, done
0005112F loc_0_5112F:               ; ---- B: same pivot, same heading, bias CAA96
000511BB js    loc_0_511D6          ; P_B <  0
000511BD mov   eax, pSegdword_C7D64
000511C2 add   eax, dword_0_CAA8C
000511C8 mov   dword_0_C7E18, eax   ; <-- the ONLY thing region 2 does differently
000511CD mov   byte_0_CAA88, 0FFh   ; ...same CAA88 as region 1
000511D6 loc_0_511D6:
000511D6 mov   byte_0_CAA88, 0
```

| region | test | `CAA88` (draw order) | span nudge |
|---|---|---|---|
| 1 | `P_A ≥ 0` | `0xFF` | no |
| 2 (wedge interior) | `P_A < 0`, `P_B ≥ 0` | `0xFF` | **yes, +`CAA8C`** |
| 3 | both `< 0` | `0` | no |

**Regions 1 and 2 write the identical `CAA88`.** Therefore:

- **B is the line where the draw order flips.**
- **A is the line where the segment-offset nudge switches on.**
- **B is not a fallback for A** — a fallback would produce a different answer.

`sub_0_511F2` is structurally identical for the exit end (`C7E16`,
`CAA98`/`CAA9A`, `C7E0C`/`C7E10`, `sub eax, CAA90` @ `0x51320`).

### What the order actually does

`sub_0_50AD7` picks a painter's-algorithm order from `CAA88`/`CB340`:

```
00050B8B cmp byte_0_CAA88, 0FFh ; jz loc_50BA0
00050B94 call sub_0_50879 ; call sub_0_50A7D   <- pit first, then track
00050BA0 call sub_0_50A7D ; call sub_0_50879   <- track first, then pit
```

Wrong order = the far ribbon paints over the near one. **Nothing is culled** —
the "disappearance" is overpainting.

`[seg+0x64]` bit 3 ("show pit objects through wall") is a **final XOR on the
order toggle** applied *after* the bias tests (`00051086 test [edi+64h],8` →
`0005108C xor byte_0_CB340, 0FFh`) — it inverts the order the biases chose
rather than adding visibility.

---

## 4. What the angles are measured against — all .TRK-derived (VERIFIED)

`sub_0_4C162`, called with `edi = pFirstPitlaneSeg`:

- **Pivot** `C7E04`/`C7E08` = the pit-entry segment centre (`[seg+4]`,
  `[seg+8]`) pushed sideways by its verge width (`[seg+0x28]`/`[0x29]`) plus a
  margin (`word_0_FA894` = 0x20) — i.e. **the pit-side verge edge of the
  pitlane's first segment**. `C7E0C`/`C7E10` = same for `pLastPitlaneSeg`.
- **Heading θ** `C7E14` = `[seg+0]` of `pFirstPitlaneSeg` — **the pitlane's own
  heading**. Not the track segment, not the camera's segment. (`C7E16` = same
  for the exit.)
- **Camera** = `dword_17457C` (X) / `dword_174584` (Z). The vertical
  (`174580`) is deliberately unused — a flat 2D ground-plane test.
- `pFirstPitlaneSeg` = `[pSegSetByCmdD1 + 0x44]`, the spatially-nearest pit
  segment to the 0xD1-tagged track segment (`DeterminPitSegs` @ `0x50F48`).

**This is the crux:** the divider line is drawn through a .TRK-derived point at
a .TRK-derived heading **plus the slot's bias**. The bias is a *relative*
angular correction; every other input comes from the track file. That
relativity is the design, and the trap.

**INFERRED:** the divider is physically the **pit wall / barrier line**, and the
two edges bound it because it is a taper rather than one straight line. The
pivot construction and the relative-angle form are VERIFIED; the "it's the pit
wall" identification is not.

**Not a failure mode:** pit side (left/right) self-corrects. `byte_0_C7E8E`
comes from the .TRK (`0x739B5`, from `w_C5Code & 2`) and is applied consistently
in both the reader (verge-byte choice + `neg eax`/`neg ebx`) and the consumers
(sign XOR).

---

## 5. What the two hidden offsets do (VERIFIED)

```
000511C2 add eax, dword_0_CAA8C ; mov dword_0_C7E18, eax  ; start = D1seg + N*108
00051320 sub eax, dword_0_CAA90 ; mov dword_0_C7E1C, eax  ; end   = DBseg - N*108
```

- `pSegdword_C7D64` = `pSegSetByCmdD1` — the 0xD1-tagged **track** segment
  (overlap start); `p1stSegAfterPitl` = `pSegSetByCmdDB` — the 0xDB-tagged
  **track** segment (overlap end).
- So the offsets **shrink the track/pit overlap span inward from both ends**:
  where along the track the draw handoff between track ribbon and pit ribbon
  happens.
- They are **track** spans, not pit spans: `C7E18` feeds `sub_0_50850`, which
  sets `CB33D = 0` → the track-array branch in `sub_0_63F0C`, bounds-checked
  against `t_TrackSegs .. pPastSegSect1`. (`sub_0_508FB`/`sub_0_50879` set
  `CB33D = 0xFF` = pit mode.)
- Both are initialised **untrimmed** at `0x50F7A`/`0x50F84` and are only
  overwritten inside the wedge — hence "conditionally live".

### The evidence this reading is right

There are exactly **two non-default offsets in all 16 records**, and they land
on **exactly the two ends where the wedge is live**:

| slot | end | A, B | wedge | offset |
|---|---|---|---|---|
| Hungaroring | entry | −128, 2048 | **live** (12°) | **3** |
| Hungaroring | exit | 1536, 1536 | degenerate | 4 (default) |
| Spa | entry | 119, 119 | degenerate | 4 (default) |
| Spa | exit | 1227, −8192 | **live** (51.7°) | **8** |

**No degenerate end anywhere in the table carries a tuned offset.** Under a
reading where the offsets apply unconditionally, that correlation is roughly a
1-in-100 coincidence. Under this reading it is forced: tuning an offset on a
degenerate end does nothing, so nobody ever did.

### The out-of-range abort (VERIFIED path, not stock-reachable)

`C7E18` → `sub_0_50850` → `sub_0_63FB1` → `sub_0_63F0C`, the range validator:

```
00063F17 cmp eax, edx ; jz loc_63F7C     ; empty range        -> STC
00063F31 jnb loc_63F7C                   ; inverted           -> STC
00063F39 jbe / 00063F41 jnb loc_63F7C    ; out of pit bounds  -> STC
00063F60 jb  / 00063F68 jnb loc_63F7C    ; out of track table -> STC
00063F7A clc ; retn    <- valid
00063F7C stc ; retn    <- INVALID: draw nothing
```

Carry propagates; every caller does `call sub_0_50850 / jb loc_0_50DCF`, which
**skips all remaining span draws for the frame, track ribbon included** — a
total blank rather than a swap. Stock offsets (3/4/8) are nowhere near it, but
the fields are unsigned and unclamped, so a bad write can reach it.

---

## 6. The `.m2d` mapping — a lossless permutation

Verified programmatically against the pristine EXE:

- The 64 editor/`.m2d` cells (`T14..T17 × 16 slots`) map **bijectively** onto the
  64 angle words. No aliasing, nothing landing outside the block, nothing lost.
- The **32 offset words are absent from the format entirely.**

```
real slot 1..8  : entry angles -> T14/T15 of m2d slot 2k-1 ; exit -> T14/T15 of m2d slot 2k
real slot 9..16 : entry angles -> T16/T17 of m2d slot 2(k-8)-1 ; exit -> T16/T17 of m2d slot 2(k-8)
```

So **Estoril (real slot 13) lives in T16/T17 of m2d slots 9 and 10**, and m2d
slot 13's own T14–T17 hold **Magny-Cours' and Suzuka's** entry angles.

### Consequences

**The permutation is self-cancelling.** Current `.m2d` + current readers
round-trip perfectly: export all 16 slots, re-import, get byte-identical data.
This is why the format survived ~25 years without anyone noticing, and why it is
**not** corrupting in the whole-EXE case.

It bites in exactly three places:

1. **Single-slot import** writes four angle words belonging to two *other* real
   slots. "I only changed Estoril" silently rewrites Magny-Cours' and Suzuka's
   pit-render angles.
2. **The 32 offset words are unreachable** — including the two that can blank a
   frame.
3. **Per-slot reasoning is impossible**, which is why T14–17 could never be
   correlated to observed behaviour.

---

## 7. The custom-track-in-a-slot bug

**Symptom (reported from testing):** with a custom track in certain slots — most
memorably Estoril — the pits and the track disappear from the in-car view when
approaching or passing the pitlane while still out on the track. Stock tracks in
their own slots are fine. **Magic data untouched.**

**Mechanism (INFERRED, medium-high):** `sub_0_4C1CF` indexes the biases by
**season slot**, but the pivot and heading come from the **loaded .TRK**. The
stock biases were hand-tuned to each slot's *original* track. A custom track
inherits that slot's bias measured against *its own* pit geometry; if that
mis-aims the divider, the camera is classified on the wrong side and
`sub_0_50AD7` paints in the wrong order.

A custom track must differ in at least one of:

1. the pitlane entry/exit **heading relative to its actual dividing wall** (the
   direct mis-aim);
2. where **0xD1 / 0xDB** are placed (moves both the pivot and the overlap zone);
3. the **verge width** at the pit-entry segment (slides the pivot laterally);
4. which segment **`+0x44`** links to (depends on the custom layout).

**On Estoril specifically — honest limits.** Estoril is the *most trivial*
record in the table: both ends degenerate, both offsets inert, two plain
half-planes at −14.9° and +7.0°. That makes it **maximally unforgiving** — a
hard flip at one shallow angle with no transition band, and no offset knob
available (opening the wedge, i.e. making A ≠ B, is a prerequisite for the
offset to do anything at all). But this describes a **class of four** —
Monaco, Montreal, Magny-Cours and Estoril all have both ends degenerate — not
Estoril uniquely. The only thing that singles Estoril out in the data is being
the **only one of the four with a negative entry bias** (−14.9°, vs Monaco
+42.2°, Magny-Cours +36.3°, Montreal +22.5°). Whether that is *why* custom
tracks struggle there is **SPECULATIVE**; settling it needs a custom .TRK
compared against stock Estoril's pit geometry, which the listing cannot answer.

**A superseded claim, recorded so it isn't re-derived:** "A == B means no
fallback branch, so degenerate slots are fragile" is **wrong**. B is not a
fallback (regions 1 and 2 write the same `CAA88`), and degeneracy is the norm —
13 of 16 slots are degenerate at one end, only Brazil, Silverstone and Suzuka
are non-degenerate at both. A degenerate end is the **simple, predictable**
case; what it costs is the tuning knob, not a safety net.

### Testable predictions (none run)

1. **Fix test:** copy stock Estoril's record (`−2720, −2720, 4, 1280, 1280, 4`,
   at file `0x138CCE + 12*12`) into whatever slot you actually run Estoril in.
   If the artifact clears, the slot/track bias mismatch is confirmed.
2. **Reproduce test:** in a slot running Estoril, set `+0`/`+2` to Monaco's
   `7680/7680`. Expect the order to flip near the pit entry.
3. **Positive control for the abort path:** set `+4` to `600` (600×108 ≈ 600
   segs). Expect `63F0C` → STC and the whole ribbon to vanish — a harder, total
   blank than the swap.
4. **Bit-3 interaction:** clearing `[seg+0x64]` bit 3 on the pit-approach
   segments of a misbehaving slot should **invert** the artifact, not remove it.

---

## 8. Implications for the `.m2d` format decision

- A correct model needs **26 values per slot**, not 24: 20 genuine scalar tables
  (1–13, 18–24) plus the 6 record fields.
- **De-permuting T14–17 is a breaking change for GP2Lap and x86GP2 even at 24
  values**, because the same 24 numbers would then mean different things. Holding
  at 24 does **not** buy reader compatibility — it only forgoes the 32 offset
  words.
- The genuinely compatible option is leaving `.m2d` exactly as it is and fixing
  only the editor's presentation, which leaves the offsets unreachable forever.
- **UI caveat for whatever layout is chosen:** the offset fields are
  *conditionally live*. On a degenerate end (13/16 slots at one end, 4/16 at
  both) an exposed offset field is a knob that does nothing until A ≠ B. Any
  26-field UI should say so.

---

## 9. Pending in this repo — **RESOLVED 2026-07-16**

The format decision landed (with Roberto): `.m2d` v2 = 28 lines in strict EXE
address order — dead T6 dropped, the four community T14–17 slices replaced by
the six true record fields (stride 12), and the three 2026-07 RE tables
(D58D4, D5A3A, D5A3C) included. Same `.m2d` extension; version detected by
line count (24 = legacy, 28 = v2). Legacy files still load and export, but the
record + new fields are skipped on export (a single-slot legacy write would
corrupt other slots per §6); files are only ever saved as v2.

- `MAGIC_LAYOUT` (`crates/gp2ws-core/src/magic.rs`) now models the true
  stride-12 record (signed angles, unsigned offsets) — byte-verified against
  the pristine EXE (Brazil record `[1792, −512, 4, −2176, 2048, 4]`).
- `MAGIC_LABELS` rewritten: six "Pit view" fields with the render-side
  reading, including the unchecked-offset blank-screen warning (§5).

## 10. Corrections owed to `~/vaults/gp2/docs/`

Not applied — listed for folding in by hand.

- `magic-data.md` §"Tables 14–17": the record layout and the ×108 claim are
  confirmed, but the offsets are **`movzx` — unsigned, no clamp**. Add that the
  A/B pair is a **wedge with asymmetric roles** (B flips order, A gates the span
  nudge), that the offsets are **conditionally live**, and that the consumers
  select **draw order**, not visibility.
- `t_Sinus??` @ `0x17A898` is a **cosine** table (starts `dw 4000h` = 1.0 in
  Q14 at index 0), not sine.
- `d_ActSegCount?` @ `0x173F10` is **not** a segment count — it is the Y half of
  an (X,Y) scratch pair with `dword_0_173F0C`.

**Unresolved:** `dword_0_CAB00` (`00050AE2 mov dword_0_CAB00, ecx`, ecx =
`C7E18`) has exactly two references in the whole listing — that write and its
data declaration. The `ecx` argument to `sub_0_50AD7` is **dead**. Abandoned
split-point feature or an original-code bug; the listing cannot say.
