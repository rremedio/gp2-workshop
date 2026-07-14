# GP2 Workshop — Plain-English Guide to Every Setting

This guide explains, in everyday language, what each setting in GP2 Workshop
actually *does* to the game. You don't need to know anything about how the game
stores its data — just read the description, change the number, and try it on
track.

If you only remember one thing: **there is no "correct" number for most of
these.** The car physics in GP2 are a big balancing act, so the honest advice
for almost every setting is *change it a little, drive a few laps, and see how
it feels.*

---

## Before you touch anything

- **Back up your `GP2.EXE` first.** This tool changes your game file directly.
  Make your own copy of the original `GP2.EXE` and keep it somewhere safe. (The
  app also makes a `GP2.EXE.bak` the first time you save, but that safety net is
  only created once — your own backup is what really protects you.)
- **Watch for the green badge.** When you open your `GP2.EXE`, the app checks
  that it recognises the file. A green "calibrated" badge means everything lines
  up and it's safe to save. If it says "uncalibrated", saving is blocked
  (you can still open and inspect files).
- **Every field has a "↺ stock" button** that puts that one setting back to the
  factory value. There's also a **"Reset all to stock"** button if you want to
  start over completely.
- **"Basic" vs "Advanced".** Each section shows the everyday settings first.
  Less-common, more-experimental settings are tucked behind an **Advanced**
  area. If a setting is marked *Advanced* below, expect it to be subtle,
  hard to predict, or best left alone unless you're experimenting.
- **A note on "multipliers".** Some settings are shown as a multiplier where
  **1.0 means "normal"**. Above 1.0 is "more", below 1.0 is "less". The guide
  points these out.

"**Stock**" below means the original factory value — the number the game ships
with and the value the "↺ stock" button restores.

---

## Physics settings

These live on the **Physics** tab, which is split into sub-tabs: Engine, Power
Curve, Drivetrain, Chassis, Aero, Brakes, Suspension, Mass/Grip, Tyres,
Surfaces, Slipstream, AI Racecraft, Walls & Damage and Steering. They affect the
cars themselves (mostly *every* car, unless a description says "player only" or
"AI only").

### Engine

| Setting | Everyday meaning | Stock |
|---|---|---|
| **Rev Limiter** | The RPM where the engine starts cutting power (the soft limiter). Raise it and the engine keeps pulling to higher revs; lower it and power dies sooner. | 15200 |
| **Max RPM** | The nominal top RPM the game uses for automatic shifting. The engine can still rev past this if a gear is geared very tall. Raise it for a higher shift point. | 15000 |
| **RPM Light 1** | The RPM at which the *first* cockpit shift light comes on. Purely cosmetic — bump it up if you raise the rev limiter so the dashboard lights still make sense. | 7146 |
| **RPM Light 2** | RPM for the *second* cockpit shift light. Cosmetic; raise it alongside the rev limiter / max RPM. | 8146 |
| **RPM Light 3** | RPM for the *third* cockpit shift light. Cosmetic. | 8496 |
| **RPM Light 4** | RPM for the *fourth (top)* cockpit shift light. Cosmetic. | 8696 |

### Power Curve

The Power Curve isn't a list of named settings — it's a single **table of 36
numbers** plus a line graph, describing how much pull the engine makes across
the rev range (low revs on the left, high revs on the right).

- **Higher numbers = more power at that point in the range.**
- There's no single "right" shape. Raise or lower points and shape the curve by
  feel, then test in-game.
- The graph updates live so you can see the curve you're drawing.

### Aero (wings and air resistance)

The two big everyday levers here are how much grip the wings give and how much
top-speed-killing drag they cost.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Downforce Scale** | Basic | Overall downforce from the wings. Higher = more cornering grip across the board; lower = less grip but slightly less drag. Affects all cars. | 44369 |
| **Rear Downforce Slope** | Basic | How much cornering grip *each step* of the rear-wing slider (1–20) adds. Raise it so higher wing settings give more grip — the main fix for "wing 1 is always fastest". | 430 |
| **Rear Downforce Floor** | Basic | The cornering grip you already get at the *lowest* rear-wing setting. Stock is high, which is why low wing still corners well; lower it to make running little wing actually cost grip. | 2064 |
| **Rear Drag Slope** | Basic | How much top-speed-killing drag *each step* of the rear-wing slider adds. Raise it to make high wing cost more top speed; lower it to make wing cheaper on the straights. | 1259 |
| **Rear Drag Floor** | Basic | Baseline drag present even at the lowest rear-wing setting. Raise it to slow every car's top speed regardless of wing; lower it for a higher minimum top speed. | 1792 |
| **Front Wing Slope** | Advanced | How much front downforce each step of the *front*-wing slider (1–20) adds. Raise it so higher front wing gives more front grip / turn-in. | 430 |
| **Front Wing Floor** | Advanced | The front downforce already present at the lowest front-wing setting. Lower it to make minimum front wing actually reduce front grip. | 2064 |
| **Front Aero Scale** | Advanced | A front-aero tuning multiplier. The exact effect is subtle and not fully confirmed — higher is *probably* a bit more front aero, but test it in-game. | 6389 |
| **Lateral Drag X** | Advanced | Sideways air resistance along one axis (felt mostly when the car is sliding). Subtle and not well confirmed — test before changing much. | 6656 |
| **Lateral Drag Y** | Advanced | Sideways air resistance along the other axis. Subtle and not well confirmed. | 22528 |
| **Rear Wing Drag Factor** | Advanced | An extra multiplier on rear-wing drag, tied into the slipstream maths. Prefer the Rear Drag Slope/Floor settings for normal tuning. | 4096 |

### Brakes

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Brake Force** | Basic | Overall braking power. Higher = shorter stopping distances. Affects the human player strongly; the AI is barely affected (it brakes to hit a target corner speed either way). | 1441792 |
| **AI Brake Strength** | Basic | **AI only — and unusual, read carefully.** Changes how hard the computer cars brake. It works **backwards and in big steps**: each **+1 roughly *halves*** AI braking (they brake earlier and softer), each **−1 roughly *doubles*** it. Stock is 8; the safe range is about **7–10**. Going below 7 can make AI braking erratic; above 10 the AI may barely brake at all. Experimental — test in-game. | 8 |
| **Brake Fail Multiplier** | Advanced | How much braking is left after a brake failure (stock leaves roughly a sixteenth). Rarely needed; higher = a failed car can still brake more. | 4096 |
| **ABS Threshold** | Advanced | The point at which the game's anti-lock braking path kicks in. Rarely needed and the exact effect isn't confirmed. | 256000 |

### Mass / Grip (weight, fuel, and overall AI grip)

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Standard Weight** | Basic | The car's reference (dry) weight. Heavier cars accelerate and brake more slowly and shift more load in corners. Best left near stock. | 1313 |
| **Normal Weight** | Basic | A second reference weight used alongside Standard Weight for the car's feel. Keep it close to Standard Weight; changing it is experimental. | 1313 |
| **CC Grip (Qualifying)** | Basic | Overall AI grip level in **qualifying**, across every track. Higher = faster AI in qualifying; lower = slower. The main knob for AI pace in quali. | 16588 |
| **CC Grip (Race)** | Basic | Overall AI grip level in the **race**, across every track. Higher = faster AI in the race; lower = slower. The main knob for AI pace in the race. | 16588 |
| **Gravity** | Advanced | The global gravity used for weight transfer. Stronger gravity shifts more load under braking and cornering. Experimental — test carefully. | −524288 |
| **Fuel Factor** | Advanced | How much weight each unit of fuel adds (more fuel = heavier, slower car). Raise it to make a full tank hurt more and fuel strategy matter more; lower it to soften the penalty. | 1627167 |
| **Packer Factor** | Advanced | Scales the suspension bump rubbers and ride heights. Subtle, and only matters with advanced setup on. | 14091072 |
| **Rebound Factor** | Advanced | Scales the suspension dampers (how the springs settle after bumps). Subtle. | 5529600 |
| **Min Grip Clamp** | Advanced | A floor on grip so it never drops below a set minimum. Raising it guarantees more baseline grip; rarely needs touching. | 11264 |

### Tyres

GP2 has **four tyre compounds, A / B / C / D**:

- **A = hardest** — least grip, but lasts the longest.
- **D = softest** — most grip, but wears out the fastest.
- B and C sit in between.

Each tyre setting is a **multiplier where 1.0 = normal.** In the original game
the four compounds are very close together, which is why tyre choice barely
matters at stock. If you want compound choice to be a real strategic decision,
spread the **Base Grip** values further apart (and/or make the soft tyres wear
faster).

**Base Grip** — how much grip a *fresh* tyre of that compound has (higher = more grip):

| Setting | Compound | Stock |
|---|---|---|
| **Tyre Base Grip A** | A (hardest) | ~1.135 |
| **Tyre Base Grip B** | B | ~1.150 |
| **Tyre Base Grip C** | C (soft) | ~1.166 |
| **Tyre Base Grip D** | D (softest) | ~1.182 |

**Wear Sensitivity** — how *fast* that compound wears out (higher = wears faster):

| Setting | Compound | Stock |
|---|---|---|
| **Tyre Wear Sensitivity A** | A (hardest) | 0.75 |
| **Tyre Wear Sensitivity B** | B | 1.0 |
| **Tyre Wear Sensitivity C** | C (soft) | 1.5 |
| **Tyre Wear Sensitivity D** | D (softest) | 3.0 |

**Worn Tyre Floor** *(one shared value, all compounds)* — the grip a fully
worn-out tyre keeps; grip never drops below this. Higher = worn tyres stay more
usable; lower = a heavier penalty for old rubber. Stock ~1.094.

### Slipstream / Tow (the draft behind another car)

The "tow" is the speed boost from tucking in behind another car on a straight.
There is **one** slipstream in GP2 and both the player and the AI use it — the
AI wake reads the same constants, so these settings affect everyone.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Tow Strength** | Basic | How strong the slipstream is when tucked in behind another car. Higher = a bigger speed boost; **0 turns the draft off entirely**. | 262144 |
| **Tow Reach** | Basic | How far back behind another car the draft still works. Higher = you can catch the tow from further away. | 384 |
| **Tow Align Width** | Basic | How directly behind the other car you must be — the *width* of the slipstream cone. Wider = easier to stay in the tow when not perfectly lined up. | 512 |
| **Tow Max Wake** | Advanced | A cap on how much boost you can get, no matter how close you are. Raise it to allow a bigger maximum draft. | 256 |
| **Tow Max Range** | Advanced | The maximum distance over which the draft applies. Higher = the slipstream stretches further down the track. | 7 |
| **Tow Min Speed** | Advanced | The minimum speed you must be going for the draft to work at all (so it only helps on fast sections). | 2816 |

### AI Racecraft (how the AI brakes for traffic)

These used to sit on the Slipstream tab under names like "AI Tow Strength" and
"AI Follow Base 1–7", which badly mis-sold them. They are **not** a slipstream
and they are **AI-only**. They are the AI's traffic-braking controller: every
one of them can only ever *slow an AI car down*, never speed it up. If you came
here looking for AI drafting, the AI already uses **Tow Strength** above.

*Recommended: leave alone unless you are testing.* None of these are verified
in-game yet.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **AI Speed-Scaled Braking** | Basic | Scales the AI's traffic-braking caps with speed. Raise it above 0 and AI cars brake *harder* for the car ahead at high speed. Despite its old name it can never add speed. | 0 (dormant) |
| **AI Avoidance Engage** | Advanced | At/above this, the AI ignores the car ahead. | −4096 |
| **AI Leader-Decel Match** | Advanced | When the car ahead slows harder than this, the follower copies its deceleration instead of predicting its own braking point. | −3072 |
| **AI Close-Follow Select** | Advanced | Picks between the AI's energy model and the tight gap servo when running right behind another car. | −24576 |
| **AI Close-Follow (Damaged)** | Advanced | Same, for the damaged / out-of-shape branch. | −1024 |
| **AI Brake Cap: Hold-Back** | Advanced | Braking ceiling in the hold-back / yellow-flag state. **Keep it negative.** | −2048 |
| **AI Brake Cap: Sliding** | Advanced | Braking ceiling while the AI car is sliding. **Keep it negative.** | −4096 |
| **AI Brake Cap: Corner Squeeze** | Advanced | Braking ceiling when the AI can't steer around the car ahead. The hardest of the three. **Keep it negative.** | −512 |
| **AI Heavy-Braking Flag** | Advanced | Threshold below which a speed drop counts as "heavy braking" (sets a status flag). | 64512 |
| **AI Max Braking / Tick** | Advanced | The floor under *every* traffic-follow braking request. More negative = the AI may brake harder for other cars. | 53248 |
| **AI Avoidance Clamp** | Advanced | The largest slow-down request the gap/closing-speed servo may generate. | 20480 |

The three **Brake Cap** values must stay **negative**. Pushed to 0 or positive
they stop being brake limits and become *acceleration* limits whenever their
state fires — which shows up as a baffling AI top-speed change rather than as a
braking bug.

Related: **AI Brake Strength (shift)** on the Brakes tab is how hard the AI
brakes for the *corner*; the settings here are how it brakes for *traffic*.

---

## Magic Data (per-track and per-driver tuning)

The **Magic Data** tab is a more advanced area. Where the Physics tab changes
the cars themselves, Magic Data holds **24 separate tuning values that the game
keeps for each of 16 entries** (the app calls them **slots 1–16**, roughly
corresponding to the championship's tracks/cars).

How to think about it:

- Pick a **slot (1–16)** at the top, then edit the **24 values** for that slot
  in the grid below.
- You can **Import/Export** a slot straight to/from the game, or **Load/Save**
  it as a `.m2d` file. **One `.m2d` file holds one slot** (24 numbers). These
  files are compatible with the old GP2 magic-data editors.
- Most of these are very situational. The ones most people actually touch are
  the **AI difficulty/pace** values and **fuel burn**; the rest (especially the
  pit-geometry ones) are best left at stock.

Here's what each of the 24 values means:

| # | Name | What it does |
|---|---|---|
| 1 | **Tyre/track grip** | Per-track grip level that also drives tyre wear. Higher = more grip but faster wear; this is the wear term that clearly slows the player too. |
| 2 | **Cornering grip (always)** | Per-track cornering-grip multiplier applied in *both* qualifying and the race. Higher = more cornering speed for every car. |
| 3 | **Cornering grip (qual)** | Per-track cornering-grip multiplier used only in non-race sessions (qualifying/practice). |
| 4 | **Cornering grip (race)** | Per-track cornering-grip multiplier used only in the race. |
| 5 | **Out-lap grip bias** | A small extra grip bias felt most on the out-lap / early laps (the "miss corners at the start" feel). |
| 6 | **Dead data** | The game ignores this — it has no effect. Leave it as-is. (Kept only so saved files match the original layout.) |
| 7 | **Driver pace (qual)** | Per-driver qualifying pace for the AI (the player is always normal). Higher = faster AI driver in qualifying. |
| 8 | **Driver pace (race)** | Per-driver race pace for the AI. Higher = faster AI driver in the race. |
| 9 | **Lap-clock rate (qual)** | Adjusts qualifying lap *times* without changing car speed — it tweaks how fast the lap clock counts. Higher = slower recorded laps. |
| 10 | **Lap-clock rate (race)** | Same idea for the race. Higher = slower recorded laps. |
| 11 | **Difficulty grip (SemiPro)** | AI grip at the SemiPro difficulty for this track (Pro is worked out from it). Higher = faster AI. |
| 12 | **Difficulty grip (Rookie)** | AI grip at the Rookie difficulty (Amateur is worked out from this and SemiPro). Higher = faster AI at the easier levels. |
| 13 | **CC mistake rate** | How often AI cars make a mistake in corners on this track. Higher = more AI mistakes; lower = cleaner AI driving. |
| 14 | **Pit geometry** | Part of the pit-lane entry/exit positioning. Best left alone. |
| 15 | **Pit geometry** | Part of the pit-lane entry/exit positioning. Best left alone. |
| 16 | **Pit geometry** | Part of the pit-lane entry/exit positioning. Best left alone. |
| 17 | **Pit geometry** | Part of the pit-lane entry/exit positioning. Best left alone. |
| 18 | **Pit-approach zone** | Length of the zone before the pit entry where the AI eases off so cars don't pile up at the pit mouth. |
| 19 | **Pit-in distance** | How far before the pit entry cars start leaving the racing line to dive into the pits. |
| 20 | **Pit-out distance** | How far after the pit exit cars use to rejoin the racing line. |
| 21 | **Pit-in speed** | The speed cars are held to in the pit-in zone (the pit-lane speed). Higher = faster pit approach. |
| 22 | **Fuel burn (human)** | Per-track fuel-burn rate for the player (normal = no change). Higher = you use more fuel per lap on this track. |
| 23 | **Fuel burn (CC)** | Per-track fuel-burn rate for the AI cars. Higher = the AI uses more fuel per lap. |
| 24 | **Reference lap time** | A reference lap time (in milliseconds) used by the race-director timing — *not* car performance. Set it to the real track lap time; it does **not** make cars faster or slower. |

---

## Quick recipes — "I just want to…"

- **Make the AI faster / slower overall** → Physics ▸ Mass/Grip ▸ **CC Grip
  (Qualifying)** and **CC Grip (Race)**. For a single track or difficulty, use
  Magic Data values **11–12** (difficulty grip) and **7–8** (driver pace).
- **Let computer cars make more/fewer mistakes** → Magic Data value **13**
  (CC mistake rate).
- **Make tyre choice actually matter** → Physics ▸ Tyres ▸ spread the four
  **Base Grip** values further apart, and make the soft compounds (C, D) wear
  faster with **Wear Sensitivity**.
- **Make low-wing setups cost something** → Physics ▸ Aero ▸ lower **Rear
  Downforce Floor** and raise **Rear Downforce Slope**.
- **Change AI slipstreaming** → Physics ▸ Slipstream ▸ **Tow Strength**. The AI
  already drafts using the same setting you do; there is no separate AI draft to
  switch on. (The old "AI Tow Strength" is now **AI Speed-Scaled Braking** on the
  AI Racecraft tab — it only ever makes the AI brake *harder*.)
- **Make the AI brake harder for traffic** → Physics ▸ AI Racecraft ▸ **AI Max
  Braking / Tick** (more negative), or raise **AI Speed-Scaled Braking** from 0.
- **Make fuel strategy more important** → Physics ▸ Mass/Grip ▸ **Fuel Factor**
  (all tracks), or Magic Data values **22–23** (per-track fuel burn).
- **Raise the rev ceiling** → Physics ▸ Engine ▸ **Rev Limiter** and **Max
  RPM**, then nudge **RPM Light 1–4** up so the dash lights still match.
- **Undo a mess** → use the **↺ stock** button on a single field, or **Reset all
  to stock** to return everything to factory values.

---

*Whenever a description says "experimental", "subtle", or "test in-game", take
it at face value: change it a little, drive, and judge by feel. There's no
substitute for laps.*
