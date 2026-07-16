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
| **Engine Force Scale** | Overall engine power, in one number — every car, every gear, on top of the Power Curve shape. Raise it for more grunt everywhere, lower it for a gentler grid. | 15728 |
| **Engine Braking** | How much the engine slows the car when you lift off the throttle in gear. Higher = the car "sits down" more on a lift; lower = it coasts more freely. | 2560 |

*Advanced here:* the Engine-Brake Pitch Factor (how much lifting off pitches the
nose), Idle RPM, Misfire Probability (only matters once an engine failure has
started), and the three gearbox RPM gates — **Upshift Block Wheel-RPM** and the
two **Shift Re-Engage RPM** values. Those three exist for one job: building
low-revving engines. Stock GP2 refuses upshifts below 9000 wheel-RPM and won't
re-engage the clutch after a shift below 8000–9000, so an engine limited to,
say, 6500 RPM can never shift — lower these gates together with the Rev
Limiter (roughly proportionally) and low-rev engines become driveable.

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
| **Reference Rake** | Basic | The nose-down "rake" angle the car's ground effect is happiest at. Downforce falls off as the car's actual attitude drifts away from this. Pairs with your ride-height setups: change one, re-tune the other. | 13760 |
| **Ground-Effect Master** | Basic | One knob for the *whole* ride-height/rake downforce effect. Stock 16384 = exactly 1.0 — the developers left it neutral, so this is a hidden lever: raise it and running the car low is rewarded much more; 0 switches ground effect off entirely. | 16384 |

*Advanced here:* the individual ground-effect terms (rake sensitivities, front
ride sensitivity and reference, and the rear/front ride clamps where the effect
maxes out), plus the three **Wing Damage** values — how much downforce a broken
rear or front wing costs (stock −25%) and how much the front end gains when
only the rear wing is gone (+25%).

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
| **Fuel Burn Base** | Basic | How fast every car burns fuel, everywhere. Higher = more consumption per lap (heavier strategy pressure); lower = economy runs. For a single track, use the Magic Data fuel-burn values (T22/T23) instead. | 2048 |

*Advanced here:* the two **Fuel Weight** values (together they set how *heavy*
each lap of fuel is — how much a full tank actually slows the car), and the
starting fuel loads for **Qualifying** (stock 4 laps + 1) and **Practice**
(stock 12 laps).

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

**Tyre behaviour (applies to every compound):**

| Setting | Everyday meaning | Stock |
|---|---|---|
| **Base Lateral Grip (Rear)** | How much sideways grip the rear tyres make. The front/rear pair is the game's built-in balance: raise the rear (or lower the front) for a more planted, understeery car; the other way round for a livelier, oversteery one. | 327616 |
| **Base Lateral Grip (Front)** | The front half of that balance. Note stock is rear-biased — that's a big part of why GP2 feels stable. | 251904 |
| **Tyre Wear Rate (Rear L/R)** | How fast the rear tyres wear, per wheel. | 640 each |
| **Tyre Wear Rate (Front L/R)** | Same for the fronts. Stock fronts wear ~1.6× faster than rears, which is why the front end goes off first — even the values out (or flip them) to change which end gives up first. | 1024 each |
| **Rear Pure-Lateral Blend** | How the rear behaves when it's sliding sideways *and* driving at the same time. Higher leans on the pure-sideways grip more (a steadier rear under power); lower makes power cost more sideways grip (snappier). Subtle — test in-game. | 6144 |

*Advanced here:* Slip Sensitivity (how sharply grip responds to slip angle),
Segment Grip Boost (the small extra grip some track segments grant), and the
rear/front tyre-curve pairs — Load Sensitivity Slope/Base (how grip falls away
as load rises), Optimal Load (the load where each axle's grip peaks) and Curve
Shape Slope/Base (how peaky or forgiving the slide is). These reshape the heart
of the tyre model; change one pair at a time and keep notes.

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

These are **AI-only**, and they are the AI's traffic-braking controller: every
one of them can only ever *slow an AI car down*, never speed it up. If you came
here looking for AI drafting, you want **Tow Strength** above — the AI already
drafts using the same setting you do.

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

### Drivetrain

How power gets from the engine to the road.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Rear Diff Lock** | Basic | The rear differential's viscous coupling. 0 behaves like an open diff (the inside wheel spins up easily); higher acts like a spool — more traction, more understeer on power. | 24576 |
| **Final Drive Divisor** | Basic | Longer or shorter gearing in *every* gear at once. Higher = fewer RPM for the same speed. The knob for re-centring gearing after changing the rev range. | 111522 |
| **Shift Cut Duration** | Basic | How long power is cut on each of *your* gear changes. Higher = shifting costs more time; 0 = instant shifts. | 2560 |
| **Shift Cut Duration (AI)** | Basic | The AI's version. Stock is *higher* than yours (4096 vs 2560), so the AI already shifts slower than you do. | 4096 |

*Advanced here:* the two Gearing Base constants (only their ratio to the Final
Drive Divisor matters), the auto-gearbox downshift guard/margin and upshift speed
floor, the analog-clutch trio, engine spin-down, and the per-wheel wheelspin /
lock-up / slip-decay gains.

### Chassis

The car's basic dimensions and how its mass is distributed. **All of these are
read when you enter the track**, so changes apply from the next session, not
live.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **CoG → Rear Axle** | Basic | ⚠️ Moves **three** things at once: with the front arm it sets the wheelbase, and the ratio between them sets weight distribution. Change with care. | 65955 |
| **CoG → Front Axle** | Basic | The other half of the pair. Also feeds the steering-assist model, so it changes steering feel too. | 94928 |
| **Rear Track Width** | Basic | How far apart the rear wheels are. Wider = more resistance to roll at the rear. | 86596 |
| **Front Track Width** | Basic | Same at the front. Raising it relative to the rear shifts balance toward oversteer. | 90113 |
| **CoG Height** | Basic | The classic "lower the centre of gravity" knob. Lower = less dive, squat and roll. | 13438 |
| **Yaw Gyration Radius** | Basic | How hard the car is to rotate. Lower = pointier and dartier; higher = lazier but more stable. | 247 |

*Advanced here:* pitch/roll gyration radii, front/rear unsprung mass, the inertia
reference fuel, and **per-wheel camber — dormant in stock GP2** (every wheel is
0, which leaves real grip machinery switched off; setting it non-zero wakes that
up, so expect surprises).

### Suspension

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Tyre Spring Rate** | Basic | How stiff the tyre carcass is — the last bit of give before the road. ⚠️ Must move together with the four Tyre Spring Init values (init × 120 has to equal this). | 24000 |
| **Spring Rate Scale** | Basic | A global multiplier on every spring setting in the garage. Use it to re-centre the whole setup range rather than editing setups. | 1966080 |
| **Anti-Roll Bar Scale** | Basic | The same idea for both anti-roll bars — the knob for making ARB choice actually matter. | 196608 |

*Advanced here:* the four **Tyre Spring Init** values (keep each equal to Tyre
Spring Rate ÷ 120), per-wheel tyre damping, suspension travel, bump-stop rates
and packer caps, droop stiffness, bump/rebound ratio, the damper knee pair
(keep it mirrored: one positive, one the same value negative), bottoming
stiffness, plank wear, and the heave/pitch/roll soft-limit knee+gain pairs.

### Surfaces

What each surface does to the car. Five classes: track, low kerb, high kerb,
grass, gravel.

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Grip: Grass** | Basic | Cornering grip on grass — the most slippery surface in stock. | 10240 |
| **Grip: Gravel** | Basic | Cornering grip in the gravel trap. | 12288 |
| **Traction: Low Kerb** | Basic | How well power goes down on a low kerb. Stock is *tiny* (1024 vs 16384 on track) — this is why kerbs light the wheels up. | 1024 |
| **Traction: High Kerb** | Basic | The same for high kerbs. | 1024 |

*Advanced here:* grip/traction/roughness for the remaining classes, the three
bump amplitudes, and the default **kerb profile shapes** — two width points and
two height points for each of the two kerb types (A = high, B = low), which set
how wide and how tall kerbs are on tracks that don't define their own.

> **Note on the "wet" tables.** GP2 ships a full second set of surface values for
> wet weather, but the blend factor that would mix them in is 0 and nothing in
> the game ever writes it — so they do nothing. They are deliberately not exposed.
> One exception: the *first* entry of each wet table is read as the kerb
> back-apron surface through an out-of-bounds read, so those three values are
> live even on a dry track.

### Walls & Damage

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Wall Restitution** | Basic | How bouncy the barriers are. Higher = you ping off harder; 0 = the car stops dead. | 4096 |
| **Wall Friction** | Basic | How much speed you keep scraping *along* a wall. Higher = you slide along losing little. | 14848 |
| **Engine-Kill Impact** | Basic | How hard a hit must be to stop the engine. Raise it for forgiving walls. | 7424 |

*Advanced here:* the wall yaw-kick gain and clamp, per-wheel damage load floors
and spring-break loads, the four per-wheel damage-threshold tables, damage
probability (stock 256 = damage is **always** deterministic; lower makes it a
gamble), and the broken-spring ride drop.

### Steering

| Setting | Tier | Everyday meaning | Stock |
|---|---|---|---|
| **Max Steering Lock** | Basic | The hard ceiling on how far the front wheels can steer. Higher = more lock for tight corners and catching slides. | 6372 |

*Advanced here:* the manual base lock, and the traction-control ramp rate (only
affects players running the traction-control aid).

---

## Magic Data (per-track and per-driver tuning)

The **Magic Data** tab is a more advanced area. Where the Physics tab changes
the cars themselves, Magic Data holds **28 separate tuning values that the game
keeps for each of 16 entries** (the app calls them **slots 1–16**, roughly
corresponding to the championship's tracks/cars).

How to think about it:

- Pick a **slot (1–16)** at the top, then edit the **28 values** for that slot
  in the grid below.
- You can **Import/Export** a slot straight to/from the game, or **Load/Save**
  it as a `.m2d` file. **One `.m2d` file holds one slot** — 28 numbers, one
  per line, in the same order as the grid.
- **Old `.m2d` files still load.** Files from the classic editors (24 lines)
  open fine; the values they don't carry (the six *Pit view* fields and the
  three *new* AI fields) simply stay untouched in your game when you Export.
  Saving always writes the new 28-line format — old editors can't read those
  files, so don't hand them to someone still on a classic tool.
- **Why the change?** The classic format's four "pit geometry" values (old
  T14–T17) turned out to be a scrambled view of six real values, mixed
  *across* tracks — editing them for one track silently changed two others.
  The new format fixes that (and drops old T6, which the game never reads).
- Most of these are very situational. The ones most people actually touch are
  the **AI difficulty/pace** values and **fuel burn**; the rest (especially the
  pit-view ones) are best left at stock.

Here's what each of the 28 values means (in file/grid order — classic values
keep their old "T" numbers):

| # | Name | What it does |
|---|---|---|
| 1 | **T1 Tyre/track grip** | Per-track grip level that also drives tyre wear. Higher = more grip but faster wear; this is the wear term that clearly slows the player too. |
| 2 | **T2 Cornering grip (always)** | Per-track cornering-grip multiplier applied in *both* qualifying and the race. Higher = more cornering speed for every car. |
| 3 | **T3 Cornering grip (qual)** | Per-track cornering-grip multiplier used only in non-race sessions (qualifying/practice). |
| 4 | **T4 Cornering grip (race)** | Per-track cornering-grip multiplier used only in the race. |
| 5 | **AI consistency floor** *(new)* | How good the AI's *worst* corners are at this track. Positive = steadier AI with fewer weak corners; negative = scrappier. The game ships this as 0 everywhere — a dormant knob. AI only. |
| 6 | **T5 AI bravery (lap 1 pace & mistakes)** | How much faster than its own "safe" cornering speed the AI dares to go at this track. Shows up mostly early: higher = a quicker first flying lap and bolder opening corners (AI brakes later behind rivals and runs side by side longer). From lap 2 the AI is allowed to make mistakes, and a higher value also makes those mistakes bigger — so overall race pace barely changes, the AI just gets braver *and* scrappier. For a clean all-race speed change use the grip/pace tables instead. AI only. |
| 7 | **T7 Driver pace (qual)** | Per-driver qualifying pace for the AI (the player is always normal). Higher = faster AI driver in qualifying. |
| 8 | **T8 Driver pace (race)** | Per-driver race pace for the AI. Higher = faster AI driver in the race. |
| 9 | **T9 Lap-clock rate (qual)** | Adjusts qualifying lap *times* without changing car speed — it tweaks how fast the lap clock counts. Higher = slower recorded laps. |
| 10 | **T10 Lap-clock rate (race)** | Same idea for the race. Higher = slower recorded laps. |
| 11 | **T11 Difficulty grip (SemiPro)** | AI grip at the SemiPro difficulty for this track (Pro is worked out from it). Higher = faster AI. |
| 12 | **T12 Difficulty grip (Rookie)** | AI grip at the Rookie difficulty (Amateur is worked out from this and SemiPro). Higher = faster AI at the easier levels. |
| 13 | **T13 CC mistake rate** | How often AI cars make a mistake in corners on this track. Higher = more AI mistakes; lower = cleaner AI driving. |
| 14 | **AI mistake severity min** *(new)* | The *smallest* extra corner speed an AI mistake carries at this track. Higher = even minor errors become obvious wobbles. Stock 512. AI only. |
| 15 | **AI mistake severity max** *(new)* | The *largest* extra corner speed an AI mistake can carry. Raise it for spectacular offs, lower it for gentle wobbles. Stock 2048. AI only. |
| 16 | **Pit view: entry angle A** | Purely visual: fine-tunes how track and pit-lane 3D graphics overlap at the pit entry. These six values replace the old T14–T17. Wrong values = glitched buildings near the pits; never affects driving. |
| 17 | **Pit view: entry angle B** | Partner of entry angle A — past this angle the draw order of track vs pit lane flips. Purely visual. |
| 18 | **Pit view: entry overlap trim** | How many track pieces the pit-entry overlap check covers (stock 3–8). ⚠️ Unchecked by the game — a large value can blank the screen near the pits. Keep it small. |
| 19 | **Pit view: exit angle A** | Same as entry angle A, for the pit *exit*. Purely visual. |
| 20 | **Pit view: exit angle B** | Same as entry angle B, for the pit *exit*. Purely visual. |
| 21 | **Pit view: exit overlap trim** | Same as the entry trim, for the pit *exit* (stock 3–8, keep small). |
| 22 | **T18 Pit-approach zone** | Length of the zone before the pit entry where the AI eases off so cars don't pile up at the pit mouth. |
| 23 | **T19 Pit-in distance** | How far before the pit entry cars start leaving the racing line to dive into the pits. |
| 24 | **T20 Pit-out distance** | How far after the pit exit cars use to rejoin the racing line. |
| 25 | **T21 Pit-in speed** | The speed cars are held to in the pit-in zone (the pit-lane speed). Higher = faster pit approach. |
| 26 | **T22 Fuel burn (human)** | Per-track fuel-burn rate for the player (normal = no change). Higher = you use more fuel per lap on this track. |
| 27 | **T23 Fuel burn (CC)** | Per-track fuel-burn rate for the AI cars. Higher = the AI uses more fuel per lap. |
| 28 | **T24 Reference lap time** | A reference lap time (in milliseconds) used by the race-director timing — *not* car performance. Set it to the real track lap time; it does **not** make cars faster or slower. |

---

## Quick recipes — "I just want to…"

- **Make the AI faster / slower overall** → Physics ▸ Mass/Grip ▸ **CC Grip
  (Qualifying)** and **CC Grip (Race)**. For a single track or difficulty, use
  the Magic Data difficulty-grip (T11/T12) and driver-pace (T7/T8) values.
- **Let computer cars make more/fewer mistakes** → Magic Data **T13
  CC mistake rate** (and the two *AI mistake severity* values for how big
  the mistakes are).
- **Make tyre choice actually matter** → Physics ▸ Tyres ▸ spread the four
  **Base Grip** values further apart, and make the soft compounds (C, D) wear
  faster with **Wear Sensitivity**.
- **Make low-wing setups cost something** → Physics ▸ Aero ▸ lower **Rear
  Downforce Floor** and raise **Rear Downforce Slope**.
- **Change AI slipstreaming** → Physics ▸ Slipstream ▸ **Tow Strength**. The AI
  already drafts using the same setting you do; there is no separate AI draft to
  switch on. (**AI Speed-Scaled Braking**, on the AI Racecraft tab, is *not* a
  draft — it only ever makes the AI brake *harder*.)
- **Make the AI brake harder for traffic** → Physics ▸ AI Racecraft ▸ **AI Max
  Braking / Tick** (more negative), or raise **AI Speed-Scaled Braking** from 0.
- **Make fuel strategy more important** → Physics ▸ Mass/Grip ▸ **Fuel Burn
  Base** (how much fuel is used) and **Fuel Factor** (how heavy it is), or the Magic
  Data fuel-burn values (T22/T23) for one track.
- **Give every car more (or less) power** → Physics ▸ Engine ▸ **Engine Force
  Scale** — one number, no need to redraw the Power Curve.
- **Dial in understeer / oversteer for everyone** → Physics ▸ Tyres ▸ nudge
  **Base Lateral Grip (Rear)** vs **(Front)** toward each other or apart.
- **Raise the rev ceiling** → Physics ▸ Engine ▸ **Rev Limiter** and **Max
  RPM**, then nudge **RPM Light 1–4** up so the dash lights still match.
- **Build a low-revving engine** (e.g. a 6500 RPM classic) → lower **Rev
  Limiter** / **Max RPM**, reshape the **Power Curve**, then lower the three
  shift RPM gates in Engine ▸ Advanced and re-gear with Drivetrain ▸ **Final
  Drive Divisor** — without the gates the car can't shift; without the gearing
  it runs out of speed.
- **Make lap 1 wilder (or calmer) at one track** → Magic Data **T5
  AI bravery**.
- **Undo a mess** → use the **↺ stock** button on a single field, or **Reset all
  to stock** to return everything to factory values.

---

*Whenever a description says "experimental", "subtle", or "test in-game", take
it at face value: change it a little, drive, and judge by feel. There's no
substitute for laps.*
