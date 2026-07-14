use crate::encoding::Encoding;
use crate::field::{FieldDesc, SubTab, Tier};
use crate::target::Target;

pub static PHYSICS_FIELDS: &[FieldDesc] = &[
    // ---- Engine ----
    FieldDesc {
        id: "rev_limiter",
        label: "Rev Limiter",
        help: "The RPM where the engine starts cutting power (the soft limiter \
               the old editor called \"Rev Limiter\"). Raise it and the engine \
               keeps pulling to higher revs; lower it and power dies sooner. \
               Stock 15200.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD6010),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 15200,
        range: None,
    },
    FieldDesc {
        id: "max_rpm",
        label: "Max RPM",
        help: "The nominal top RPM the game uses for automatic shifting. The \
               engine can still rev past this if a gear is geared too tall. \
               Raise it for a higher shift point. Stock 15000.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD601C),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 15000,
        range: None,
    },
    FieldDesc {
        id: "rpm_light_1",
        label: "RPM Light 1",
        help: "The RPM at which the first cockpit shift light comes on. Purely \
               cosmetic, but bump it up if you raise the rev limiter or max RPM \
               so the dashboard lights still make sense. Old editor: \
               \"RPM Lights 1\". Stock 7146.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD6700),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 7146,
        range: None,
    },
    FieldDesc {
        id: "rpm_light_2",
        label: "RPM Light 2",
        help: "The RPM at which the second cockpit shift light comes on. Purely \
               cosmetic; raise it alongside the rev limiter / max RPM. Old \
               editor: \"RPM Lights 2\". Stock 8146.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD6704),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8146,
        range: None,
    },
    FieldDesc {
        id: "rpm_light_3",
        label: "RPM Light 3",
        help: "The RPM at which the third cockpit shift light comes on. Purely \
               cosmetic; raise it alongside the rev limiter / max RPM. Old \
               editor: \"RPM Lights 3\". Stock 8496.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD6708),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8496,
        range: None,
    },
    FieldDesc {
        id: "rpm_light_4",
        label: "RPM Light 4",
        help: "The RPM at which the fourth (top) cockpit shift light comes on. \
               Purely cosmetic; raise it alongside the rev limiter / max RPM. \
               Old editor: \"RPM Lights 4\". Stock 8696.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD670C),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8696,
        range: None,
    },
    FieldDesc {
        id: "engine_force_scale",
        label: "Engine Force Scale",
        help: "A global multiplier on engine power, applied on top of the power \
               curve AND each car's own horsepower. Higher = every car on the \
               grid gets more punch; lower = the whole field is slower. This is \
               the blunt \"more power\" knob when you don't want to redraw the \
               curve. Affects all cars. Old editor: \"Power Factor\". Stock \
               15728.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD60B0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 15728,
        range: None,
    },
    FieldDesc {
        id: "engine_braking",
        label: "Engine Braking",
        help: "How much the engine slows the car when you lift off the throttle. \
               Higher = lifting scrubs more speed and the car settles onto its \
               nose sooner; lower = it coasts. Affects all cars. Stock 2560.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD5FE4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2560,
        range: None,
    },
    FieldDesc {
        id: "engine_brake_pitch",
        label: "Engine-Brake Pitch Factor",
        help: "How much engine braking pitches the car forward, separately from \
               how much it slows you. It is a quirk: raising it makes lifting off \
               dive the nose more without changing the actual retardation. \
               Affects all cars. Stock 3072.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        target: Target::Data(0x1731C0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3072,
        range: None,
    },
    FieldDesc {
        id: "idle_rpm",
        label: "Idle RPM",
        help: "The RPM the engine idles at when you're stopped and off the \
               throttle (it jitters about 128 either side). Mostly cosmetic - \
               raise it alongside a raised rev range so idle doesn't sound dead. \
               Stock 3712.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        target: Target::Data(0x174040),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3712,
        range: None,
    },
    FieldDesc {
        id: "misfire_probability",
        label: "Misfire Probability",
        help: "How often the engine misfires once it has picked up an engine \
               failure - on a 0-255 scale, so stock 128 is about half the time. \
               Higher = a failing engine splutters more; 0 = it never misfires. \
               Only applies after a failure, not to a healthy engine. Stock 128.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        // Plan flagged this width as uncertain; the listing settles it:
        // `000D56B0 dword_0_D56B0 dd 80h` -> dd, width 4.
        target: Target::Data(0xD56B0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 128,
        range: None,
    },
    // ---- Drivetrain ----
    FieldDesc {
        id: "diff_lock",
        label: "Rear Diff Lock",
        help: "The rear differential's viscous coupling - how strongly the two \
               rear wheels are pulled to the same speed. 0 behaves like a fully \
               open diff (inside wheel spins up easily); higher values act like a \
               locked diff / spool (more traction, more understeer on power). \
               Affects all cars. Stock 24576 (x1.5).",
        subtab: SubTab::Drivetrain,
        tier: Tier::Basic,
        target: Target::Data(0xD53C4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 24576,
        range: None,
    },
    FieldDesc {
        id: "final_drive",
        label: "Final Drive Divisor",
        help: "The divisor in the speed-to-RPM conversion, applied on top of \
               every gear ratio. Higher = longer gearing (fewer RPM for the same \
               speed) in every gear at once; lower = shorter. This is the knob \
               for re-centring the whole gear-ratio range after changing Rev \
               Limiter or Max RPM. Affects all cars. Stock 111522.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Basic,
        target: Target::Data(0xD5FC8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 111522,
        range: None,
    },
    FieldDesc {
        id: "gearing_base_1",
        label: "Gearing Base 1",
        help: "One of three inputs to the speed-to-RPM conversion (with Gearing \
               Base 2 and Final Drive Divisor). Only the RATIO between the three \
               matters, so changing one alone rescales all the gearing. Prefer \
               Final Drive Divisor for normal tuning and leave this alone unless \
               you know why. Old editor: \"Differential Final Ratio - Factor 1\". \
               Stock 304.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FC0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 304,
        range: None,
    },
    FieldDesc {
        id: "gearing_base_2",
        label: "Gearing Base 2",
        help: "The second of the three speed-to-RPM inputs (see Gearing Base 1). \
               Higher = shorter gearing (more RPM for the same speed). Only its \
               ratio to the other two matters. Old editor: \"Differential Final \
               Ratio - Factor 2\". Stock 1728.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FC4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1728,
        range: None,
    },
    FieldDesc {
        id: "shift_cut_player",
        label: "Shift Cut Duration",
        help: "How long engine power is cut on every gear change, for the player. \
               Higher = a longer dead spot on each shift, so shifting costs more \
               time; 0 makes shifts instant. Player only - the AI has its own \
               value. Old editor: \"Upshift Penalty (Humans)\" (which edited a \
               single byte at 1390C1). Stock 2560.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Basic,
        target: Target::Data(0xD5E6C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2560,
        range: None,
    },
    FieldDesc {
        id: "shift_cut_ai",
        label: "Shift Cut Duration (AI)",
        help: "The AI twin of Shift Cut Duration - how long power is cut on each \
               AI gear change. Higher = AI cars lose more time per shift. Stock \
               is higher than the player's 2560, so the AI already shifts slower \
               than you do. AI only. Old editor: \"Upshift Penalty (CCs)\". Stock \
               4096.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Basic,
        target: Target::Data(0xD5E70),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4096,
        range: None,
    },
    FieldDesc {
        id: "downshift_guard",
        label: "Downshift Over-Rev Guard",
        help: "The RPM ceiling the automatic gearbox will not downshift through: \
               a downshift that would send the engine past this is refused. Raise \
               it to allow later, more aggressive downshifts; lower it to protect \
               the engine. Affects cars on auto gears (including the AI). Stock \
               14800.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        // Listing symbol is `w_maxrpm` (dw) - NOT the same as our `max_rpm`
        // field at 0xD601C. Verified dw in the annotated listing.
        target: Target::Data(0xD6020),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 14800,
        range: None,
    },
    FieldDesc {
        id: "downshift_margin",
        label: "Downshift Table Margin",
        help: "The RPM margin left when the game builds its automatic-downshift \
               table at session start. Higher = the auto box downshifts more \
               conservatively (further from the limit). Affects cars on auto \
               gears. Stock 800.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD6024),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 800,
        range: None,
    },
    FieldDesc {
        id: "min_upshift_speed",
        label: "Min Auto-Upshift Speed",
        help: "The speed floor below which the automatic gearbox will not \
               upshift, so it does not short-shift when crawling. Higher = the \
               auto box holds the lower gear longer before it will change up. \
               Affects cars on auto gears. Stock 4608.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD602E),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4608,
        range: None,
    },
    FieldDesc {
        id: "clutch_blend_gain",
        label: "Clutch Engagement Gain",
        help: "How sharply an analog clutch blends drive in as it is released. \
               Higher = the clutch bites harder and sooner; lower = a longer, \
               softer take-up. Only affects players using an analog clutch axis. \
               Stock 262144.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5408),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 262144,
        range: None,
    },
    FieldDesc {
        id: "clutch_rpm_lag",
        label: "Clutch RPM Lag",
        help: "How fast engine revs chase the target while the analog clutch is \
               slipping - the rev-matching rate. Higher = the revs catch up \
               faster. Only affects players using an analog clutch. Stock 32768.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD6036),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 32768,
        range: None,
    },
    FieldDesc {
        id: "clutch_slip_decay",
        label: "Clutch Slip Decay",
        help: "How quickly slipping engine revs settle back to the speed the \
               wheels are asking for. Higher = slip disappears faster (the clutch \
               feels more locked). Only affects players using an analog clutch. \
               Stock 4096.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD6032),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4096,
        range: None,
    },
    FieldDesc {
        id: "engine_spin_down",
        label: "Engine-Off Spin-Down",
        help: "How fast the revs die away once the engine is off (stalled or \
               blown). Higher = the engine stops spinning sooner. Cosmetic in \
               most situations. Stock 4000.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD603A),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4000,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_driven_rl",
        label: "Wheelspin Gain (Rear Left)",
        help: "How fast the rear left wheel spins up when you give it more power \
               than it can put down. Higher = wheelspin builds faster and is \
               easier to trigger; lower = the wheel hooks up more forgivingly. \
               Stock is lower at the rear (98304) than the front (131072). \
               Affects all cars. Stock 98304.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5318 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 98304,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_driven_rr",
        label: "Wheelspin Gain (Rear Right)",
        help: "How fast the rear right wheel spins up when you give it more power \
               than it can put down. Higher = wheelspin builds faster and is \
               easier to trigger; lower = the wheel hooks up more forgivingly. \
               Stock is lower at the rear (98304) than the front (131072). \
               Affects all cars. Stock 98304.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5318 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 98304,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_driven_fl",
        label: "Wheelspin Gain (Front Left)",
        help: "How fast the front left wheel spins up when you give it more power \
               than it can put down. Higher = wheelspin builds faster and is \
               easier to trigger; lower = the wheel hooks up more forgivingly. \
               Stock is lower at the rear (98304) than the front (131072). \
               Affects all cars. Stock 131072.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5318 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 131072,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_driven_fr",
        label: "Wheelspin Gain (Front Right)",
        help: "How fast the front right wheel spins up when you give it more \
               power than it can put down. Higher = wheelspin builds faster and \
               is easier to trigger; lower = the wheel hooks up more forgivingly. \
               Stock is lower at the rear (98304) than the front (131072). \
               Affects all cars. Stock 131072.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5318 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 131072,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_lock_rl",
        label: "Lock-Up Gain (Rear Left)",
        help: "How readily the rear left wheel locks under braking - the \
               mirror-image of Wheelspin Gain. Higher = the wheel locks more \
               easily and flat-spots sooner; lower = it keeps rolling under heavy \
               braking. Affects all cars. Stock 98304.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5328 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 98304,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_lock_rr",
        label: "Lock-Up Gain (Rear Right)",
        help: "How readily the rear right wheel locks under braking - the \
               mirror-image of Wheelspin Gain. Higher = the wheel locks more \
               easily and flat-spots sooner; lower = it keeps rolling under heavy \
               braking. Affects all cars. Stock 98304.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5328 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 98304,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_lock_fl",
        label: "Lock-Up Gain (Front Left)",
        help: "How readily the front left wheel locks under braking - the \
               mirror-image of Wheelspin Gain. Higher = the wheel locks more \
               easily and flat-spots sooner; lower = it keeps rolling under heavy \
               braking. Affects all cars. Stock 131072.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5328 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 131072,
        range: None,
    },
    FieldDesc {
        id: "spin_gain_lock_fr",
        label: "Lock-Up Gain (Front Right)",
        help: "How readily the front right wheel locks under braking - the \
               mirror-image of Wheelspin Gain. Higher = the wheel locks more \
               easily and flat-spots sooner; lower = it keeps rolling under heavy \
               braking. Affects all cars. Stock 131072.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5328 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 131072,
        range: None,
    },
    FieldDesc {
        id: "slip_decay_rl",
        label: "Slip Decay Gain (Rear Left)",
        help: "How fast the rear left wheel recovers once it is spinning or \
               locked - effectively that wheel's inertia. Higher = it returns to \
               rolling speed sooner, so spin and lock-up are shorter-lived; lower \
               = they persist. Pairs with Wheelspin Gain and Lock-Up Gain. \
               Affects all cars. Stock 1310720.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5338 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1310720,
        range: None,
    },
    FieldDesc {
        id: "slip_decay_rr",
        label: "Slip Decay Gain (Rear Right)",
        help: "How fast the rear right wheel recovers once it is spinning or \
               locked - effectively that wheel's inertia. Higher = it returns to \
               rolling speed sooner, so spin and lock-up are shorter-lived; lower \
               = they persist. Pairs with Wheelspin Gain and Lock-Up Gain. \
               Affects all cars. Stock 1310720.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5338 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1310720,
        range: None,
    },
    FieldDesc {
        id: "slip_decay_fl",
        label: "Slip Decay Gain (Front Left)",
        help: "How fast the front left wheel recovers once it is spinning or \
               locked - effectively that wheel's inertia. Higher = it returns to \
               rolling speed sooner, so spin and lock-up are shorter-lived; lower \
               = they persist. Pairs with Wheelspin Gain and Lock-Up Gain. \
               Affects all cars. Stock 2228224.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5338 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2228224,
        range: None,
    },
    FieldDesc {
        id: "slip_decay_fr",
        label: "Slip Decay Gain (Front Right)",
        help: "How fast the front right wheel recovers once it is spinning or \
               locked - effectively that wheel's inertia. Higher = it returns to \
               rolling speed sooner, so spin and lock-up are shorter-lived; lower \
               = they persist. Pairs with Wheelspin Gain and Lock-Up Gain. \
               Affects all cars. Stock 2228224.",
        subtab: SubTab::Drivetrain,
        tier: Tier::Advanced,
        target: Target::Data(0xD5338 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2228224,
        range: None,
    },
    // ---- Chassis & Geometry ----
    // All of these are session-init inputs: the derived-parameter init reads
    // them at track entry, so edits apply from the next session, not live.
    FieldDesc {
        id: "cog_rear_arm",
        label: "CoG -> Rear Axle",
        help: "Distance from the centre of gravity back to the rear axle. This \
               one knob moves THREE things at once: with CoG -> Front Axle it \
               sets the wheelbase, and the ratio between the two sets the \
               front/rear weight distribution (lever rule). Bigger = the CoG sits \
               further forward of the rear axle, shifting weight onto the front. \
               Change it with care. Read once when you enter the track, so \
               changes only take effect on the next session. Stock 65955.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E50),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 65955,
        range: None,
    },
    FieldDesc {
        id: "cog_front_arm",
        label: "CoG -> Front Axle",
        help: "Distance from the centre of gravity forward to the front axle. \
               Pairs with CoG -> Rear Axle to set wheelbase and weight \
               distribution (see that field's warning), and it also feeds the \
               steering-assist bicycle model, so it changes steering feel as well \
               as balance. Read once when you enter the track, so changes only \
               take effect on the next session. Stock 94928.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E54),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 94928,
        range: None,
    },
    FieldDesc {
        id: "track_rear",
        label: "Rear Track Width",
        help: "How far apart the two rear wheels are. Wider = more resistance to \
               roll at the rear and a bit more rear grip in fast direction \
               changes; narrower = the rear lets go sooner. Read once when you \
               enter the track, so changes only take effect on the next session. \
               Stock 86596.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E58),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 86596,
        range: None,
    },
    FieldDesc {
        id: "track_front",
        label: "Front Track Width",
        help: "How far apart the two front wheels are. Wider = more front-end \
               bite and resistance to roll at the front; narrower = less. Raising \
               this relative to the rear track shifts the balance toward \
               oversteer. Read once when you enter the track, so changes only \
               take effect on the next session. Stock 90113.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E5C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 90113,
        range: None,
    },
    FieldDesc {
        id: "cog_height",
        label: "CoG Height",
        help: "How high the centre of gravity sits. This is the classic \"lower \
               the CoG\" knob: it is the baseline torque arm for pitch under \
               braking and roll in corners. Lower = less weight transfer, so less \
               dive, squat and body roll. Affects all cars. Read once when you \
               enter the track, so changes only take effect on the next session. \
               Stock 13438.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E64),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 13438,
        range: None,
    },
    FieldDesc {
        id: "gyr_yaw",
        label: "Yaw Gyration Radius",
        help: "Feeds the car's yaw inertia - how hard it is to rotate the car \
               about a vertical axis. Lower = a pointier, more darty car that \
               changes direction eagerly; higher = lazier turn-in but more \
               stability. Affects all cars. Old editor: \"Polar Moment of \
               Inertia\". Read once when you enter the track, so changes only \
               take effect on the next session. Stock 247.",
        subtab: SubTab::Chassis,
        tier: Tier::Basic,
        target: Target::Data(0xD5E90),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 247,
        range: None,
    },
    FieldDesc {
        id: "gyr_pitch",
        label: "Pitch Gyration Radius",
        help: "Feeds the car's pitch inertia - resistance to nose-up / nose-down \
               rotation under braking and acceleration. Lower = the car pitches \
               more readily; higher = it resists dive and squat. Read once when \
               you enter the track, so changes only take effect on the next \
               session. Stock 220.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5E94),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 220,
        range: None,
    },
    FieldDesc {
        id: "gyr_roll",
        label: "Roll Gyration Radius",
        help: "Feeds the car's roll inertia - resistance to rolling about the \
               long axis in corners. Lower = the car rolls onto its outside tyres \
               more readily; higher = it resists. Read once when you enter the \
               track, so changes only take effect on the next session. Stock 104.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5E98),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 104,
        range: None,
    },
    FieldDesc {
        id: "unsprung_rear",
        label: "Rear Unsprung Mass",
        help: "The mass of one rear wheel/upright assembly - the part NOT carried \
               by the springs. Higher = the rear wheels are slower to follow \
               bumps and kerbs, so the car skips more over rough surfaces. Per \
               wheel. Read once when you enter the track, so changes only take \
               effect on the next session. Stock 55.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5E88),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 55,
        range: None,
    },
    FieldDesc {
        id: "unsprung_front",
        label: "Front Unsprung Mass",
        help: "The mass of one front wheel/upright assembly (see Rear Unsprung \
               Mass). Higher = the front skips more over bumps and kerbs. Per \
               wheel. Read once when you enter the track, so changes only take \
               effect on the next session. Stock 44.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5E84),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 44,
        range: None,
    },
    FieldDesc {
        id: "inertia_fuel",
        label: "Inertia Reference Fuel",
        help: "A nominal fuel load used ONLY when computing the car's inertia - \
               it is not the fuel you actually carry and does not change fuel \
               strategy or weight. Higher = the car behaves as though it has more \
               fuel sloshing in the inertia maths. Read once when you enter the \
               track, so changes only take effect on the next session. Stock \
               60000.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5C3A),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 60000,
        range: None,
    },
    FieldDesc {
        id: "camber_rl",
        label: "Camber (Rear Left)",
        help: "Static camber for the rear left wheel - how far the top of the \
               tyre leans in or out. DORMANT in stock GP2: every wheel is 0, \
               which leaves a whole piece of grip machinery switched off. Setting \
               it non-zero wakes that machinery up, so it is a hidden feature \
               rather than a normal tuning knob - expect surprises and test \
               in-game. Signed: the sign is the lean direction. Read once when \
               you enter the track, so changes only take effect on the next \
               session. Stock 0.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        // Verified structurally, not by value: stock is 0, which would match at
        // any zeroed address. The listing shows `dword_0_D5EC4 dd 0` read as
        // `mov eax, dword_0_D5EC4[ecx*4]` (0x1A9F7/0x1AA28/0x1AA3F, in the tyre
        // coefficient code) - a stride-4 dword array, confirming base + width.
        target: Target::Data(0xD5EC4 + 0),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: 0,
        range: None,
    },
    FieldDesc {
        id: "camber_rr",
        label: "Camber (Rear Right)",
        help: "Static camber for the rear right wheel - how far the top of the \
               tyre leans in or out. DORMANT in stock GP2: every wheel is 0, \
               which leaves a whole piece of grip machinery switched off. Setting \
               it non-zero wakes that machinery up, so it is a hidden feature \
               rather than a normal tuning knob - expect surprises and test \
               in-game. Signed: the sign is the lean direction. Read once when \
               you enter the track, so changes only take effect on the next \
               session. Stock 0.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EC4 + 4),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: 0,
        range: None,
    },
    FieldDesc {
        id: "camber_fl",
        label: "Camber (Front Left)",
        help: "Static camber for the front left wheel - how far the top of the \
               tyre leans in or out. DORMANT in stock GP2: every wheel is 0, \
               which leaves a whole piece of grip machinery switched off. Setting \
               it non-zero wakes that machinery up, so it is a hidden feature \
               rather than a normal tuning knob - expect surprises and test \
               in-game. Signed: the sign is the lean direction. Read once when \
               you enter the track, so changes only take effect on the next \
               session. Stock 0.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EC4 + 8),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: 0,
        range: None,
    },
    FieldDesc {
        id: "camber_fr",
        label: "Camber (Front Right)",
        help: "Static camber for the front right wheel - how far the top of the \
               tyre leans in or out. DORMANT in stock GP2: every wheel is 0, \
               which leaves a whole piece of grip machinery switched off. Setting \
               it non-zero wakes that machinery up, so it is a hidden feature \
               rather than a normal tuning knob - expect surprises and test \
               in-game. Signed: the sign is the lean direction. Read once when \
               you enter the track, so changes only take effect on the next \
               session. Stock 0.",
        subtab: SubTab::Chassis,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EC4 + 12),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: 0,
        range: None,
    },
    // ---- Walls & Damage ----
    FieldDesc {
        id: "wall_restitution",
        label: "Wall Restitution",
        help: "How bouncy the barriers are - how much speed you keep bouncing \
               back off a wall. Higher = you ping off harder; 0 = the car just \
               stops dead against it. Affects all cars. Stock 4096 (0.25).",
        subtab: SubTab::Walls,
        tier: Tier::Basic,
        target: Target::Data(0xC6A2C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4096,
        range: None,
    },
    FieldDesc {
        id: "wall_friction",
        label: "Wall Friction",
        help: "How much speed you keep while scraping ALONG a wall. Higher = you \
               slide along the barrier losing little speed; lower = scraping \
               scrubs you off hard. Affects all cars. Stock 14848 (0.906).",
        subtab: SubTab::Walls,
        tier: Tier::Basic,
        target: Target::Data(0xC6A30),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 14848,
        range: None,
    },
    FieldDesc {
        id: "wall_yaw_gain",
        label: "Wall Yaw-Kick Gain",
        help: "How much clipping a barrier spins the car. Higher = a glancing hit \
               snaps you sideways; lower = the car shrugs walls off. Pairs with \
               Wall Yaw-Kick Clamp, which caps the result. Affects all cars. \
               Stock 131072.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC6A34),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 131072,
        range: None,
    },
    FieldDesc {
        id: "wall_yaw_clamp",
        label: "Wall Yaw-Kick Clamp",
        help: "The ceiling on the spin a wall contact can impart, no matter how \
               bad the hit. Lower = even big clips can only spin you so far; \
               raise it to allow wilder spins. Affects all cars. Stock 6144.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC6A3C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 6144,
        range: None,
    },
    FieldDesc {
        id: "engine_kill_threshold",
        label: "Engine-Kill Impact",
        help: "How hard an impact has to be before it stops the engine. Raise it \
               for forgiving walls (you survive bigger hits still running); lower \
               it and light taps will kill the engine. Affects all cars. Stock \
               7424.",
        subtab: SubTab::Walls,
        tier: Tier::Basic,
        target: Target::Data(0xCBD24),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 7424,
        range: None,
    },
    FieldDesc {
        id: "damage_load_floor_rl",
        label: "Damage Load Floor (Rear Left)",
        help: "The minimum load on the rear left corner before the game even \
               rolls for damage - below this, that corner cannot be damaged at \
               all. Higher = that corner shrugs off bigger hits; lower = it \
               damages more easily. Stock is higher at the rear than the front, \
               so the front is more fragile. Affects all cars. Stock 1048576.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        // PLAN DEVIATION (agreed): the plan modelled this as ONE scalar field
        // `damage_load_floor` @ 0xC7A60 stock 917504. The code reads it per
        // wheel - `cmp edi, osCarDamageTrack[ecx*4]` @ 0x32B09, the same shape
        // as dword_0_C7A70[ecx*4] beside it - and a pristine GP2.EXE holds a
        // rear/front split {0x100000,0x100000,0xE0000,0xE0000}, matching every
        // neighbouring table. The scalar reading came from EXEs where this
        // array was flattened to 0xE0000 x4, which makes indexed and scalar
        // indistinguishable by value.
        target: Target::Data(0xC7A60 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1048576,
        range: None,
    },
    FieldDesc {
        id: "damage_load_floor_rr",
        label: "Damage Load Floor (Rear Right)",
        help: "The minimum load on the rear right corner before the game even \
               rolls for damage - below this, that corner cannot be damaged at \
               all. Higher = that corner shrugs off bigger hits; lower = it \
               damages more easily. Stock is higher at the rear than the front, \
               so the front is more fragile. Affects all cars. Stock 1048576.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A60 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1048576,
        range: None,
    },
    FieldDesc {
        id: "damage_load_floor_fl",
        label: "Damage Load Floor (Front Left)",
        help: "The minimum load on the front left corner before the game even \
               rolls for damage - below this, that corner cannot be damaged at \
               all. Higher = that corner shrugs off bigger hits; lower = it \
               damages more easily. Stock is higher at the rear than the front, \
               so the front is more fragile. Affects all cars. Stock 917504.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A60 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 917504,
        range: None,
    },
    FieldDesc {
        id: "damage_load_floor_fr",
        label: "Damage Load Floor (Front Right)",
        help: "The minimum load on the front right corner before the game even \
               rolls for damage - below this, that corner cannot be damaged at \
               all. Higher = that corner shrugs off bigger hits; lower = it \
               damages more easily. Stock is higher at the rear than the front, \
               so the front is more fragile. Affects all cars. Stock 917504.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A60 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 917504,
        range: None,
    },
    FieldDesc {
        id: "spring_break_rl",
        label: "Spring Break Load (Rear Left)",
        help: "The load at which the rear left spring breaks. Lower = springs \
               snap more readily over kerbs and in heavy landings; raise it for a \
               tougher car. See Broken-Spring Ride Drop for what happens once one \
               goes. Affects all cars. Stock 118095872.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A70 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 118095872,
        range: None,
    },
    FieldDesc {
        id: "spring_break_rr",
        label: "Spring Break Load (Rear Right)",
        help: "The load at which the rear right spring breaks. Lower = springs \
               snap more readily over kerbs and in heavy landings; raise it for a \
               tougher car. See Broken-Spring Ride Drop for what happens once one \
               goes. Affects all cars. Stock 118095872.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A70 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 118095872,
        range: None,
    },
    FieldDesc {
        id: "spring_break_fl",
        label: "Spring Break Load (Front Left)",
        help: "The load at which the front left spring breaks. Lower = springs \
               snap more readily over kerbs and in heavy landings; raise it for a \
               tougher car. See Broken-Spring Ride Drop for what happens once one \
               goes. Affects all cars. Stock 118030336.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A70 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 118030336,
        range: None,
    },
    FieldDesc {
        id: "spring_break_fr",
        label: "Spring Break Load (Front Right)",
        help: "The load at which the front right spring breaks. Lower = springs \
               snap more readily over kerbs and in heavy landings; raise it for a \
               tougher car. See Broken-Spring Ride Drop for what happens once one \
               goes. Affects all cars. Stock 118030336.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A70 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 118030336,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_a_rl",
        label: "Damage Threshold A (Rear Left)",
        help: "One of four load thresholds for the rear left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1835008.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        // Stride 8: these tables are (threshold, flag) dword PAIRS. Fields
        // target the thresholds only (+0/+8/+16/+24); the interleaved flag
        // dwords at +4 are damage bits and must not be edited.
        target: Target::Data(0xC7AA0 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1835008,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_a_rr",
        label: "Damage Threshold A (Rear Right)",
        help: "One of four load thresholds for the rear right corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1835008.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AA0 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1835008,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_a_fl",
        label: "Damage Threshold A (Front Left)",
        help: "One of four load thresholds for the front left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1703936.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AA0 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1703936,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_a_fr",
        label: "Damage Threshold A (Front Right)",
        help: "One of four load thresholds for the front right corner: pass it \
               and the game rolls for a specific piece of damage. Lower = that \
               damage happens on smaller hits (a more fragile car); higher = the \
               corner survives more. A/B/C/D are separate damage effects, not \
               severity steps. Affects all cars. Stock 1703936.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AA0 + 24),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1703936,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_b_rl",
        label: "Damage Threshold B (Rear Left)",
        help: "One of four load thresholds for the rear left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 2097152.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AC0 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2097152,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_b_rr",
        label: "Damage Threshold B (Rear Right)",
        help: "One of four load thresholds for the rear right corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 2097152.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AC0 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2097152,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_b_fl",
        label: "Damage Threshold B (Front Left)",
        help: "One of four load thresholds for the front left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1835008.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AC0 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1835008,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_b_fr",
        label: "Damage Threshold B (Front Right)",
        help: "One of four load thresholds for the front right corner: pass it \
               and the game rolls for a specific piece of damage. Lower = that \
               damage happens on smaller hits (a more fragile car); higher = the \
               corner survives more. A/B/C/D are separate damage effects, not \
               severity steps. Affects all cars. Stock 1835008.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AC0 + 24),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1835008,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_c_rl",
        label: "Damage Threshold C (Rear Left)",
        help: "One of four load thresholds for the rear left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 2359296.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A80 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2359296,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_c_rr",
        label: "Damage Threshold C (Rear Right)",
        help: "One of four load thresholds for the rear right corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 2359296.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A80 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2359296,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_c_fl",
        label: "Damage Threshold C (Front Left)",
        help: "One of four load thresholds for the front left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1048576.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A80 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1048576,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_c_fr",
        label: "Damage Threshold C (Front Right)",
        help: "One of four load thresholds for the front right corner: pass it \
               and the game rolls for a specific piece of damage. Lower = that \
               damage happens on smaller hits (a more fragile car); higher = the \
               corner survives more. A/B/C/D are separate damage effects, not \
               severity steps. Affects all cars. Stock 1048576.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7A80 + 24),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1048576,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_d_rl",
        label: "Damage Threshold D (Rear Left)",
        help: "One of four load thresholds for the rear left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 3145728.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AE0 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3145728,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_d_rr",
        label: "Damage Threshold D (Rear Right)",
        help: "One of four load thresholds for the rear right corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 3145728.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AE0 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3145728,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_d_fl",
        label: "Damage Threshold D (Front Left)",
        help: "One of four load thresholds for the front left corner: pass it and \
               the game rolls for a specific piece of damage. Lower = that damage \
               happens on smaller hits (a more fragile car); higher = the corner \
               survives more. A/B/C/D are separate damage effects, not severity \
               steps. Affects all cars. Stock 1310720.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AE0 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1310720,
        range: None,
    },
    FieldDesc {
        id: "dmg_thr_d_fr",
        label: "Damage Threshold D (Front Right)",
        help: "One of four load thresholds for the front right corner: pass it \
               and the game rolls for a specific piece of damage. Lower = that \
               damage happens on smaller hits (a more fragile car); higher = the \
               corner survives more. A/B/C/D are separate damage effects, not \
               severity steps. Affects all cars. Stock 1310720.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7AE0 + 24),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1310720,
        range: None,
    },
    FieldDesc {
        id: "damage_probability",
        label: "Damage Probability",
        help: "The chance a damage roll actually sticks once a threshold is \
               passed. Stock 256 means ALWAYS - damage is fully deterministic in \
               stock GP2. Lower it and damage becomes a gamble (128 = about half \
               the time). Affects all cars. Stock 256.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xC7B00),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "spring_break_drop",
        label: "Broken-Spring Ride Drop",
        help: "How far the corner sinks once its spring has broken. Higher = a \
               broken spring drops the car further, so it grounds out and handles \
               worse; lower = a broken spring matters less. Affects all cars. \
               Stock 32768.",
        subtab: SubTab::Walls,
        tier: Tier::Advanced,
        target: Target::Data(0xD55E8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 32768,
        range: None,
    },
    // ---- Surfaces ----
    // The listing notes at t_gripmax (0xD5CF4): "there are 3 tables of 2 * 5
    // dwords. The 5 is from 5 surface types. The 2 is from min and max value,
    // using some factor to average between the two."
    //
    // The blend is a Q14 lerp at 0x1FB67: coef = (max*(0x4000-w) + min*w) >> 14,
    // w = dword_0_D5D6C. w is 0 in stock and has NO WRITER anywhere in the
    // binary, so the *min* halves are dead AS WET VALUES and are not exposed.
    //
    // EXCEPT the first entry of each min table. Surface class comes from
    // `D5692 & 0xF`, and class 5 (kerb back-apron) indexes ONE PAST the 5-entry
    // max table (`mov eax, t_gripmax[ecx*4]`), landing on min[0]. So 0xD5D08 /
    // 0xD5D30 / 0xD5D58 ARE live - not as wet values, but as class 5's dry
    // coefficients (0x3800 / 0x2000 / 0x100). A shipped out-of-bounds quirk;
    // see ~/vaults/gp2/docs/physics-surface-sampling.md.
    FieldDesc {
        id: "surf_grip_track",
        label: "Grip: Track",
        help: "Cornering grip on track. Higher = you can lean on that surface \
               harder; lower = it lets go sooner. Stock puts track and both kerbs \
               equal, with grass the most slippery and gravel between. Affects \
               all cars. Old editor: \"Human Grip\" (Misc tab). Note: this entry's wet twin is NOT dead - the game reads it as the \
               kerb back-apron surface via an out-of-bounds read, so it is that \
               surface's live value. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5CF4 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_grip_kerb_low",
        label: "Grip: Low Kerb",
        help: "Cornering grip on low kerb. Higher = you can lean on that surface \
               harder; lower = it lets go sooner. Stock puts track and both kerbs \
               equal, with grass the most slippery and gravel between. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5CF4 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_grip_kerb_high",
        label: "Grip: High Kerb",
        help: "Cornering grip on high kerb. Higher = you can lean on that surface \
               harder; lower = it lets go sooner. Stock puts track and both kerbs \
               equal, with grass the most slippery and gravel between. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5CF4 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_grip_grass",
        label: "Grip: Grass",
        help: "Cornering grip on grass. Higher = you can lean on that surface \
               harder; lower = it lets go sooner. Stock puts track and both kerbs \
               equal, with grass the most slippery and gravel between. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 10240.",
        subtab: SubTab::Surfaces,
        tier: Tier::Basic,
        target: Target::Data(0xD5CF4 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 10240,
        range: None,
    },
    FieldDesc {
        id: "surf_grip_gravel",
        label: "Grip: Gravel",
        help: "Cornering grip on gravel. Higher = you can lean on that surface \
               harder; lower = it lets go sooner. Stock puts track and both kerbs \
               equal, with grass the most slippery and gravel between. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 12288.",
        subtab: SubTab::Surfaces,
        tier: Tier::Basic,
        target: Target::Data(0xD5CF4 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12288,
        range: None,
    },
    FieldDesc {
        id: "surf_traction_track",
        label: "Traction: Track",
        help: "How well power gets down on track - the drive/braking grip, as \
               opposed to cornering grip. Higher = the wheels hook up; lower = \
               they just spin. Note the stock kerb values are TINY (1024 vs 16384 \
               on track), which is why putting power down on a kerb lights the \
               wheels up. Affects all cars. Old editor: \"Asphalt Acceleration \
               1\" (\"Asphalt Acceleration 2\" was its dead wet twin and is not \
               ported). Note: this entry's wet twin is NOT dead - the game reads it as the \
               kerb back-apron surface via an out-of-bounds read, so it is that \
               surface's live value. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D1C + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_traction_kerb_low",
        label: "Traction: Low Kerb",
        help: "How well power gets down on low kerb - the drive/braking grip, as \
               opposed to cornering grip. Higher = the wheels hook up; lower = \
               they just spin. Note the stock kerb values are TINY (1024 vs 16384 \
               on track), which is why putting power down on a kerb lights the \
               wheels up. Affects all cars. Its wet-weather twin in the same \
               table is dead and does nothing. Stock 1024.",
        subtab: SubTab::Surfaces,
        tier: Tier::Basic,
        target: Target::Data(0xD5D1C + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1024,
        range: None,
    },
    FieldDesc {
        id: "surf_traction_kerb_high",
        label: "Traction: High Kerb",
        help: "How well power gets down on high kerb - the drive/braking grip, as \
               opposed to cornering grip. Higher = the wheels hook up; lower = \
               they just spin. Note the stock kerb values are TINY (1024 vs 16384 \
               on track), which is why putting power down on a kerb lights the \
               wheels up. Affects all cars. Its wet-weather twin in the same \
               table is dead and does nothing. Stock 1024.",
        subtab: SubTab::Surfaces,
        tier: Tier::Basic,
        target: Target::Data(0xD5D1C + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1024,
        range: None,
    },
    FieldDesc {
        id: "surf_traction_grass",
        label: "Traction: Grass",
        help: "How well power gets down on grass - the drive/braking grip, as \
               opposed to cornering grip. Higher = the wheels hook up; lower = \
               they just spin. Note the stock kerb values are TINY (1024 vs 16384 \
               on track), which is why putting power down on a kerb lights the \
               wheels up. Affects all cars. Its wet-weather twin in the same \
               table is dead and does nothing. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D1C + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_traction_gravel",
        label: "Traction: Gravel",
        help: "How well power gets down on gravel - the drive/braking grip, as \
               opposed to cornering grip. Higher = the wheels hook up; lower = \
               they just spin. Note the stock kerb values are TINY (1024 vs 16384 \
               on track), which is why putting power down on a kerb lights the \
               wheels up. Affects all cars. Its wet-weather twin in the same \
               table is dead and does nothing. Stock 16384.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D1C + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "surf_rough_track",
        label: "Roughness: Track",
        help: "How rough track is to drive over - how much it shakes the car and \
               upsets the suspension. Higher = rougher. Stock makes gravel far \
               rougher than grass, and the track and kerbs smooth. Affects all \
               cars. Note: this entry's wet twin is NOT dead - the game reads it as the \
               kerb back-apron surface via an out-of-bounds read, so it is that \
               surface's live value. Stock 256.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D44 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "surf_rough_kerb_low",
        label: "Roughness: Low Kerb",
        help: "How rough low kerb is to drive over - how much it shakes the car \
               and upsets the suspension. Higher = rougher. Stock makes gravel \
               far rougher than grass, and the track and kerbs smooth. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 256.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D44 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "surf_rough_kerb_high",
        label: "Roughness: High Kerb",
        help: "How rough high kerb is to drive over - how much it shakes the car \
               and upsets the suspension. Higher = rougher. Stock makes gravel \
               far rougher than grass, and the track and kerbs smooth. Affects \
               all cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 256.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D44 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "surf_rough_grass",
        label: "Roughness: Grass",
        help: "How rough grass is to drive over - how much it shakes the car and \
               upsets the suspension. Higher = rougher. Stock makes gravel far \
               rougher than grass, and the track and kerbs smooth. Affects all \
               cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 2048.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D44 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2048,
        range: None,
    },
    FieldDesc {
        id: "surf_rough_gravel",
        label: "Roughness: Gravel",
        help: "How rough gravel is to drive over - how much it shakes the car and \
               upsets the suspension. Higher = rougher. Stock makes gravel far \
               rougher than grass, and the track and kerbs smooth. Affects all \
               cars. Its wet-weather twin in the same table is dead and does \
               nothing. Stock 12288.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D44 + 16),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12288,
        range: None,
    },
    FieldDesc {
        id: "bump_track_scale",
        label: "Bump Amplitude: Track",
        help: "How big the track's built-in bumps feel. Higher = a rougher ride \
               on the racing surface itself; lower = a smoother track. Separate \
               from Roughness: this scales the bump shapes, roughness scales how \
               much they upset the car. Affects all cars. Stock 65536.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD7E24),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 65536,
        range: None,
    },
    FieldDesc {
        id: "bump_grass",
        label: "Bump Amplitude: Grass",
        help: "How big the bumps are once you put a wheel on the grass. Higher = \
               running wide over grass throws the car around more. Affects all \
               cars. Stock 786432.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD7E1C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 786432,
        range: None,
    },
    FieldDesc {
        id: "bump_gravel",
        label: "Bump Amplitude: Gravel",
        help: "How big the bumps are in a gravel trap. Higher = a wilder ride \
               once you are in the gravel. Stock is the roughest of the three \
               surfaces. Affects all cars. Stock 1310720.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Data(0xD7E20),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1310720,
        range: None,
    },
    // ---- Aero ----
    FieldDesc {
        id: "df_scale",
        label: "Downforce Scale",
        help: "Overall downforce from the wings (the old editor's \"Downforce\"). \
               Higher = more cornering grip across the board; lower = less grip \
               but slightly less drag. Affects all cars.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Data(0xD5EA8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 44369,
        range: None,
    },
    FieldDesc {
        id: "rear_df_slope",
        label: "Rear Downforce Slope",
        help: "How much cornering grip each step of the rear-wing slider (1-20) \
               adds. Raise it so higher wing settings give more grip - the main \
               lever for fixing \"wing 1 is always fastest\". Stock 430.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Code(0x1682D),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 430,
        range: None,
    },
    FieldDesc {
        id: "rear_df_floor",
        label: "Rear Downforce Floor",
        help: "The cornering grip you already get at the lowest rear-wing setting \
               (wing 1). Stock is high (2064), which is why low wing still corners \
               well; lower it to make running little wing actually cost grip. \
               Stock 2064.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Code(0x1683B),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2064,
        range: None,
    },
    FieldDesc {
        id: "rear_drag_slope",
        label: "Rear Drag Slope",
        help: "How much top-speed-killing drag each step of the rear-wing slider \
               (1-20) adds. Raise it to make high wing cost more top speed; lower \
               it to make wing cheaper on the straights. Stock 1259.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Code(0x168EC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1259,
        range: None,
    },
    FieldDesc {
        id: "rear_drag_floor",
        label: "Rear Drag Floor",
        help: "The baseline drag present even at the lowest rear-wing setting \
               (wing 1). Raise it to slow every car's top speed regardless of \
               wing; lower it for a higher minimum top speed. Stock 1792.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Code(0x168FA),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1792,
        range: None,
    },
    FieldDesc {
        id: "front_wing_slope",
        label: "Front Wing Slope",
        help: "How much front downforce each step of the front-wing slider (1-20) \
               adds. Raise it so higher front wing gives more front grip / turn-in. \
               Stock 430.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        // Plan table listed IDA 0x1685F, but that address points directly at the
        // imm32 operand (opcode 0xBA `mov edx,imm32` sits at 0x1685E). Point Target
        // at the opcode so the +1 operand adjustment lands correctly. Verified
        // against real EXE: file off 0x8EAB3 reads 430 (= 0x1AE).
        target: Target::Code(0x1685E),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x1AE,
        range: None,
    },
    FieldDesc {
        id: "front_wing_floor",
        label: "Front Wing Floor",
        help: "The front downforce already present at the lowest front-wing \
               setting (wing 1). Lower it to make minimum front wing actually \
               reduce front grip. Stock 2064.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        // Same correction as front_wing_slope: plan listed 0x1686D (the operand),
        // opcode 0x05 (`add eax,imm32`) is at 0x1686C. Verified against real EXE:
        // file off 0x8EAC1 reads 2064 (= 0x810).
        target: Target::Code(0x1686C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x810,
        range: None,
    },
    FieldDesc {
        id: "front_aero_scale",
        label: "Front Aero Scale",
        help: "Advanced front-aero tuning multiplier. The exact effect is subtle \
               and not fully confirmed - we expect higher = a bit more front aero, \
               but it's worth testing in-game before relying on it.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EA4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x18F5,
        range: None,
    },
    FieldDesc {
        id: "lateral_drag_x",
        label: "Lateral Drag X",
        help: "Advanced: sideways air resistance along one axis (felt mostly when \
               the car is sliding). The effect is subtle and not well confirmed - \
               test in-game before changing much. Stock 6656.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FD4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x1A00,
        range: None,
    },
    FieldDesc {
        id: "lateral_drag_y",
        label: "Lateral Drag Y",
        help: "Advanced: sideways air resistance along the other axis (felt mostly \
               when the car is sliding). The effect is subtle and not well \
               confirmed - test in-game before changing much. Stock 22528.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FD8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x5800,
        range: None,
    },
    FieldDesc {
        id: "rear_wing_drag_factor",
        label: "Rear Wing Drag Factor",
        help: "Advanced extra multiplier on rear-wing drag, used together with the \
               slipstream/wake maths. Effect is subtle and uncertain - prefer the \
               Rear Drag Slope/Floor fields for normal tuning, and test this one \
               in-game. Stock 4096.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD53E0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x1000,
        range: None,
    },
    // ---- Brakes ----
    FieldDesc {
        id: "brake_force",
        label: "Brake Force",
        help: "Overall braking power. Higher = shorter stopping distances. It \
               affects the human player strongly (you brake by feel), but only a \
               little for the AI, which brakes to hit a target corner speed either \
               way. Old editor: \"Breaking Force\". Stock 1441792.",
        subtab: SubTab::Brakes,
        tier: Tier::Basic,
        target: Target::Data(0xD53FC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x160000,
        range: None,
    },
    FieldDesc {
        id: "ai_brake_strength",
        label: "AI Brake Strength (shift)",
        help: "AI-only brake strength (the player is not affected). The AI brakes \
               from a table the game rebuilds each session; this is the right-shift \
               used when building it. It works BACKWARDS and in big steps: each +1 \
               HALVES AI braking (brakes earlier and softer), each -1 DOUBLES it. \
               Stock 8. Safe range about 7-10. Below 7 the values can overflow and \
               make AI braking erratic; above 10 they can hit zero and the AI may \
               not brake at all. This is how hard the AI brakes for the CORNER; \
               how it brakes for TRAFFIC is on the AI Racecraft tab. \
               Experimental - test in-game.",
        subtab: SubTab::Brakes,
        tier: Tier::Basic,
        // 1-byte code patch: the shift count of `sar eax, 8` at IDA 0x2EF90 inside
        // the AI decel-table builder sub_2EE0F. Direct file offset (opcode+2, so
        // Target::Code's opcode+1 rule doesn't fit). Verified byte = 0x08.
        target: Target::Direct(0xA71E6),
        width: 1,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8,
        range: Some((4, 14)),
    },
    FieldDesc {
        id: "brake_fail_mult",
        label: "Brake Fail Multiplier",
        help: "Advanced: how much braking is left after a brake failure (it's \
               applied twice, so stock leaves roughly a sixteenth). Rarely needed; \
               higher = a failed car can still brake more. Stock 4096.",
        subtab: SubTab::Brakes,
        tier: Tier::Advanced,
        target: Target::Data(0xD56A8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x1000,
        range: None,
    },
    FieldDesc {
        id: "abs_threshold",
        label: "ABS Threshold",
        help: "Advanced: the point at which the special anti-lock braking path \
               kicks in. Rarely needed and the exact in-game effect isn't \
               confirmed - test before relying on it. Stock 256000.",
        subtab: SubTab::Brakes,
        tier: Tier::Advanced,
        target: Target::Data(0xD5400),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x3E800,
        range: None,
    },
    // (Removed "AI Brake Lookahead 1/2" = off_C9970/off_C9974. Tracing the EXE
    // showed these are NOT corner-braking lookahead: both gate pit-lane approach
    // behaviour (compared against car+0x29C distance-to-pit, guarding pit line
    // departure and the pit speed limiter), so they have no effect on racing.)
    // ---- Mass / Grip ----
    FieldDesc {
        id: "std_weight",
        label: "Standard Weight",
        help: "Advanced: the car's reference (dry) weight. Heavier cars \
               accelerate and brake more slowly and shift load more in corners. \
               Best left near stock unless you know what you're after. Stock 1313.",
        subtab: SubTab::MassGrip,
        tier: Tier::Basic,
        target: Target::Data(0xD5E74),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1313,
        range: None,
    },
    FieldDesc {
        id: "norm_weight",
        label: "Normal Weight",
        help: "Advanced: a second reference weight used alongside Standard Weight \
               for the car's mass/acceleration feel. Keep it close to Standard \
               Weight; changing it is experimental. Stock 1313.",
        subtab: SubTab::MassGrip,
        tier: Tier::Basic,
        target: Target::Data(0xD5E78),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1313,
        range: None,
    },
    FieldDesc {
        id: "gravity",
        label: "Gravity",
        help: "Experimental: the global gravity the game uses for weight transfer \
               (stored as a negative number, stock -524288 = 32.0 down). Stronger \
               gravity shifts more load under braking/cornering. Changing it is \
               experimental - test carefully.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EA0),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: -0x80000,
        range: None,
    },
    FieldDesc {
        id: "fuel_factor",
        label: "Fuel Factor",
        help: "How much weight each unit of fuel adds (more fuel = heavier, \
               slower car). Raise it to make a full tank hurt more and fuel \
               strategy matter more; lower it to soften the penalty. Stock 1627167.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD57D8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1627167,
        range: None,
    },
    FieldDesc {
        id: "packer_factor",
        label: "Packer Factor",
        help: "Advanced: scales the suspension bump rubbers (packers) and ride \
               heights. The effect is subtle and only applies with advanced setup \
               on; test in-game. Stock 14091072.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD54B4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0xD70340,
        range: None,
    },
    FieldDesc {
        id: "rebound_factor",
        label: "Rebound Factor",
        help: "Advanced: scales the suspension dampers (how the springs settle \
               after bumps). The effect is subtle; test in-game before relying on \
               it. Stock 5529600.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD54C8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x546000,
        range: None,
    },
    FieldDesc {
        id: "min_grip_clamp",
        label: "Min Grip Clamp",
        help: "Advanced: a floor on grip so it never drops below a set minimum. \
               Raising it guarantees more baseline grip; rarely needs touching. \
               Stock 11264.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD5E3E),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x2C00,
        range: None,
    },
    FieldDesc {
        id: "cc_grip_qual",
        label: "CC Grip (Qualifying)",
        help: "Overall AI grip level in qualifying, across every track (the old \
               editor's \"Overall CC Grip Level\" for quali). Higher = faster AI \
               in qualifying; lower = slower. Stock 16588.",
        subtab: SubTab::MassGrip,
        tier: Tier::Basic,
        target: Target::Data(0xD5974),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x40CC,
        range: None,
    },
    FieldDesc {
        id: "cc_grip_race",
        label: "CC Grip (Race)",
        help: "Overall AI grip level in the race, across every track (the old \
               editor's \"Overall CC Grip Level\" for the race). Higher = faster \
               AI in the race; lower = slower. Stock 16588.",
        subtab: SubTab::MassGrip,
        tier: Tier::Basic,
        target: Target::Data(0xD5978),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x40CC,
        range: None,
    },
    // ---- Slipstream / Tow ----
    FieldDesc {
        id: "tow_strength",
        label: "Tow Strength",
        help: "How strong the slipstream is when tucked in behind another car. \
               Higher = a bigger speed boost in the draft; 0 turns the draft \
               off entirely. Affects BOTH the player and AI cars (the AI wake \
               uses the same constant). Stock 262144.",
        subtab: SubTab::Slipstream,
        tier: Tier::Basic,
        target: Target::Data(0xD53DC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x40000,
        range: None,
    },
    FieldDesc {
        id: "tow_reach",
        label: "Tow Reach",
        help: "How far back behind another car the slipstream still works. \
               Higher = the draft can be caught from further away; lower = you \
               must get very close. Affects both player and AI. Stock 384.",
        subtab: SubTab::Slipstream,
        tier: Tier::Basic,
        target: Target::Data(0xC9750),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x180,
        range: None,
    },
    FieldDesc {
        id: "tow_align_width",
        label: "Tow Align Width",
        help: "How directly behind the other car you must be to get the draft - \
               the width of the slipstream cone. Wider = easier to stay in the \
               tow when not perfectly lined up. Affects both player and AI. \
               Stock 512.",
        subtab: SubTab::Slipstream,
        tier: Tier::Basic,
        target: Target::Data(0xC9748),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x200,
        range: None,
    },
    FieldDesc {
        id: "ai_tow_strength",
        label: "AI Speed-Scaled Braking",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Despite the old name it can never add speed: every \
               place it is read scales the AI\'s traffic-BRAKING caps with speed, \
               so raising it above stock 0 makes AI cars brake harder for the car \
               ahead at high speed. The real AI slipstream is Tow Strength, which \
               AI cars share with the player. Old editor: \"AI Tow Strength\". \
               Stock 0 (dormant).",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Basic,
        target: Target::Data(0xD5FF4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0,
        range: None,
    },
    FieldDesc {
        id: "tow_max_wake",
        label: "Tow Max Wake",
        help: "A cap on how much slipstream boost you can get, no matter how \
               close you get. Raise it to allow a bigger maximum draft effect. \
               Affects both player and AI. Stock 256.",
        subtab: SubTab::Slipstream,
        tier: Tier::Advanced,
        target: Target::Data(0xC9752),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x100,
        range: None,
    },
    FieldDesc {
        id: "tow_max_range",
        label: "Tow Max Range",
        help: "The maximum distance (in track segments) over which the draft can \
               apply. Higher = the slipstream stretches further down the track. \
               Used by the player's car scan; the AI wake path picks its target \
               car differently. Stock 7.",
        subtab: SubTab::Slipstream,
        tier: Tier::Advanced,
        target: Target::Data(0xC9766),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 7,
        range: None,
    },
    FieldDesc {
        id: "tow_min_speed",
        label: "Tow Min Speed",
        help: "The minimum speed you must be going for the slipstream to work at \
               all (so it only helps on fast sections). Raise it to restrict the \
               draft to higher speeds; lower it to allow it sooner. Gates the \
               player's scan. Stock 2816.",
        subtab: SubTab::Slipstream,
        tier: Tier::Advanced,
        target: Target::Data(0xD5D96),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0xB00,
        range: None,
    },
    // ai_follow_base_1..7 (7 signed thresholds, Hex)
    FieldDesc {
        id: "ai_follow_base_1",
        label: "AI Avoidance Engage",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). It can only slow AI cars, never boost them. Avoidance \
               engage threshold - when the avoidance metric is at/above this, the \
               AI ignores the car ahead (neutral). Signed; scaled by frame time \
               at session start. Old editor: \"AI Follow Base 1\". Stock -4096.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96CA),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -4096, // 0xF000 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_2",
        label: "AI Leader-Decel Match",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Leader-deceleration matching threshold - when the car \
               ahead is slowing harder than this, the follower copies its \
               deceleration instead of predicting its own braking point. Signed; \
               scaled by frame time at session start. Old editor: \"AI Follow \
               Base 2\". Stock -3072.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96CC),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -3072, // 0xF400 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_3",
        label: "AI Close-Follow Select",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Close-follow selector - decides whether the AI uses \
               its energy model or the tight gap servo when running right behind \
               another car. Signed; scaled by frame time at session start. Old \
               editor: \"AI Follow Base 3\". Stock -24576.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96D2),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -24576, // 0xA000 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_4",
        label: "AI Close-Follow (Damaged)",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Same close-follow selection as Close-Follow Select, \
               but for the branch used when the AI car is damaged/out of shape. \
               Signed; scaled by frame time at session start. Old editor: \"AI \
               Follow Base 4\". Stock -1024.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96D4),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -1024, // 0xFC00 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_5",
        label: "AI Brake Cap: Hold-Back",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Per-tick BRAKE ceiling forced on an AI car in its \
               hold-back / yellow-flag state (slowing past accident sites, \
               yielding). More negative = it brakes harder in that state. Keep it \
               NEGATIVE: pushed toward 0 or positive it turns into an \
               acceleration limit whenever the state fires, which reads as a \
               mysterious AI top-speed change. Scaled by frame time at session \
               start. Old editor: \"AI Follow Base 5\". Stock -2048.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96D8),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -2048, // 0xF800 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_6",
        label: "AI Brake Cap: Sliding",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Per-tick BRAKE ceiling forced on an AI car while it \
               is SLIDING (the slide-test flag). More negative = harder braking \
               when sliding. Keep it NEGATIVE - toward 0 or positive it becomes \
               an acceleration limit at the grip limit, which shows up as an AI \
               top-speed change. Scaled by frame time at session start. Old \
               editor: \"AI Follow Base 6\". Stock -4096.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96DA),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -4096, // 0xF000 as signed i16
        range: None,
    },
    FieldDesc {
        id: "ai_follow_base_7",
        label: "AI Brake Cap: Corner Squeeze",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Per-tick BRAKE ceiling in the AI\'s corner-squeeze \
               state (\"can\'t steer around the car ahead at this grip, so brake \
               instead\"). The hardest of the three state ceilings. Keep it \
               NEGATIVE (see the Hold-Back and Sliding caps). Scaled by frame \
               time at session start. Old editor: \"AI Follow Base 7\". Stock \
               -512.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xC96DC),
        width: 2,
        signed: true,
        encoding: Encoding::Raw,
        stock: -512, // 0xFE00 as signed i16
        range: None,
    },
    // ai_follow_floor_1..3 (Hex, u32)
    FieldDesc {
        id: "ai_follow_floor_1",
        label: "AI Heavy-Braking Flag",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Threshold below which an AI car\'s per-tick speed \
               delta counts as \"heavy braking\" (sets a status flag; the value \
               is a negative s16, stored here as 0xFC00 = -1024). Scaled by frame \
               time at session start. Old editor: \"AI Follow Floor 1\". Stock \
               64512.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FE8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0xFC00,
        range: None,
    },
    FieldDesc {
        id: "ai_follow_floor_2",
        label: "AI Max Braking / Tick",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). The AI\'s maximum braking per tick for ALL \
               traffic-follow caps - every braking request from the follow \
               controller is floored here (negative s16, stored as 0xD000 = \
               -12288). More negative = the AI may brake harder for other cars. \
               Scaled by frame time at session start. Old editor: \"AI Follow \
               Floor 2\". Stock 53248.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FEC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0xD000,
        range: None,
    },
    FieldDesc {
        id: "ai_follow_floor_3",
        label: "AI Avoidance Clamp",
        help: "Part of the AI's traffic-braking controller (System C - not the \
               slipstream). Clamp on the AI\'s avoidance metric - the largest \
               slow-down request the gap/closing-speed servo may generate when \
               closing on the car ahead (positive s16, 0x5000). Scaled by frame \
               time at session start. Old editor: \"AI Follow Floor 3\". Stock \
               20480.",
        subtab: SubTab::AiRacecraft,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FF0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x5000,
        range: None,
    },
    FieldDesc {
        id: "fuel_burn_base",
        label: "Fuel Burn Base",
        help: "The baseline rate the cars burn fuel. Higher = everyone uses more \
               fuel per lap, so races need bigger loads or more stops. This \
               chains with the per-track fuel multipliers in Magic Data (T22 \
               human / T23 CC), which scale it per circuit. Affects all cars. \
               Stock 2048.",
        subtab: SubTab::MassGrip,
        tier: Tier::Basic,
        target: Target::Data(0xD57DC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2048,
        range: None,
    },
    FieldDesc {
        id: "fuel_weight_div",
        label: "Fuel Weight Divisor",
        help: "One half of the pair that converts fuel into weight (with Fuel \
               Weight Multiplier). Together they set the roughly \
               776-units-to-pounds slope, about 4.7 lbs per lap of fuel. Only \
               their ratio matters, so change one and you rescale how heavy fuel \
               feels. Affects all cars. Stock 563.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD57CC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 563,
        range: None,
    },
    FieldDesc {
        id: "fuel_weight_mult",
        label: "Fuel Weight Multiplier",
        help: "The other half of the fuel-to-weight conversion (see Fuel Weight \
               Divisor). Raising this relative to the divisor makes each lap of \
               fuel weigh more, so heavy cars feel heavier and fuel strategy \
               matters more. Affects all cars. Stock 437318.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD57D4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 437318,
        range: None,
    },
    FieldDesc {
        id: "qual_fuel_laps",
        label: "Qualifying Fuel Laps",
        help: "How many laps of fuel the AI carries in qualifying (the code adds \
               1, so stock 4 means 5 laps' worth). Lower = lighter, faster AI \
               qualifying runs; higher = heavier and slower. AI only. Stock 4.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        target: Target::Data(0xD3550),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4,
        range: None,
    },
    // ---- Tyres (additions) ----
    FieldDesc {
        id: "rear_lateral_blend",
        label: "Rear Pure-Lateral Blend",
        help: "How much of the rear tyre's cornering force comes from the \
               pure-lateral model versus the combined (cornering + driving) \
               model. Stock 6144 mixes 0.375 pure with 0.625 combined. Higher = \
               the rear behaves more like a tyre that isn't being asked to do two \
               jobs at once, so it breaks away more predictably; lower = more of \
               the snappy combined-slip character. This is the rear breakaway \
               feel knob. Affects all cars. Stock 6144.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5354),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 6144,
        range: None,
    },
    FieldDesc {
        id: "slip_prescale",
        label: "Slip Sensitivity",
        help: "Scales slip before it goes into the tyre model - effectively how \
               twitchy the tyres are about sliding. Higher = the tyre reacts to \
               smaller slip, so the grip peak arrives sooner and the car feels \
               sharper and edgier; lower = lazier, more forgiving. Affects all \
               cars. Stock 682.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::Data(0xD5F5C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 682,
        range: None,
    },
    FieldDesc {
        id: "wear_rate_rl",
        label: "Tyre Wear Rate (Rear Left)",
        help: "How fast the rear left tyre wears. Higher = it wears out sooner, \
               so stints get shorter. Stock wears the FRONTS faster than the \
               rears (1024 vs 640). This is the per-wheel rate; the per-compound \
               multipliers are the Tyre Wear Sensitivity fields. Affects all \
               cars. Stock 640.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5524 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 640,
        range: None,
    },
    FieldDesc {
        id: "wear_rate_rr",
        label: "Tyre Wear Rate (Rear Right)",
        help: "How fast the rear right tyre wears. Higher = it wears out sooner, \
               so stints get shorter. Stock wears the FRONTS faster than the \
               rears (1024 vs 640). This is the per-wheel rate; the per-compound \
               multipliers are the Tyre Wear Sensitivity fields. Affects all \
               cars. Stock 640.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5524 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 640,
        range: None,
    },
    FieldDesc {
        id: "wear_rate_fl",
        label: "Tyre Wear Rate (Front Left)",
        help: "How fast the front left tyre wears. Higher = it wears out sooner, \
               so stints get shorter. Stock wears the FRONTS faster than the \
               rears (1024 vs 640). This is the per-wheel rate; the per-compound \
               multipliers are the Tyre Wear Sensitivity fields. Affects all \
               cars. Stock 1024.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5524 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1024,
        range: None,
    },
    FieldDesc {
        id: "wear_rate_fr",
        label: "Tyre Wear Rate (Front Right)",
        help: "How fast the front right tyre wears. Higher = it wears out sooner, \
               so stints get shorter. Stock wears the FRONTS faster than the \
               rears (1024 vs 640). This is the per-wheel rate; the per-compound \
               multipliers are the Tyre Wear Sensitivity fields. Affects all \
               cars. Stock 1024.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::Data(0xD5524 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1024,
        range: None,
    },
    FieldDesc {
        id: "segment_grip_boost",
        label: "Segment Grip Boost",
        help: "A per-track-segment grip multiplier the game can apply on top of \
               everything else. Higher = more grip wherever it applies. Stock \
               17408 is just above 16384 (1.0), so it is a slight boost. Affects \
               all cars. Stock 17408.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::Data(0xD5704),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 17408,
        range: None,
    },
    // ---- Suspension ----
    FieldDesc {
        id: "tyre_spring",
        label: "Tyre Spring Rate",
        help: "How stiff the tyre carcass itself is as a spring - the last bit of \
               give between the wheel and the road. Higher = a harsher ride and \
               more kerb kick; lower = the tyre soaks up bumps. MUST move \
               together with the Tyre Spring (Init) values: the init value x120 \
               has to equal this rate, or the sim's setup pass and its \
               per-substep pass disagree. Affects all cars. Stock 24000.",
        subtab: SubTab::Suspension,
        tier: Tier::Basic,
        // Paired with tyre_spring_init_* (0xD54E8+4i): init * 120 == this rate
        // (200 * 120 == 24000 in stock). Change one without the other and the
        // session-init pass and the substep pass use different spring rates.
        target: Target::Data(0xD5508),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 24000,
        range: None,
    },
    FieldDesc {
        id: "tyre_spring_init_rl",
        label: "Tyre Spring Init (Rear Left)",
        help: "The rear left tyre's carcass spring rate as used by the \
               session-init pass, stored 120x smaller than the real rate. Stock \
               200 x 120 = 24000, which is exactly Tyre Spring Rate - keep that \
               relationship or the two passes disagree and the car behaves \
               inconsistently. Change Tyre Spring Rate and you must change this \
               too. Stock 200.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD54E8 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 200,
        range: None,
    },
    FieldDesc {
        id: "tyre_spring_init_rr",
        label: "Tyre Spring Init (Rear Right)",
        help: "The rear right tyre's carcass spring rate as used by the \
               session-init pass, stored 120x smaller than the real rate. Stock \
               200 x 120 = 24000, which is exactly Tyre Spring Rate - keep that \
               relationship or the two passes disagree and the car behaves \
               inconsistently. Change Tyre Spring Rate and you must change this \
               too. Stock 200.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD54E8 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 200,
        range: None,
    },
    FieldDesc {
        id: "tyre_spring_init_fl",
        label: "Tyre Spring Init (Front Left)",
        help: "The front left tyre's carcass spring rate as used by the \
               session-init pass, stored 120x smaller than the real rate. Stock \
               200 x 120 = 24000, which is exactly Tyre Spring Rate - keep that \
               relationship or the two passes disagree and the car behaves \
               inconsistently. Change Tyre Spring Rate and you must change this \
               too. Stock 200.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD54E8 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 200,
        range: None,
    },
    FieldDesc {
        id: "tyre_spring_init_fr",
        label: "Tyre Spring Init (Front Right)",
        help: "The front right tyre's carcass spring rate as used by the \
               session-init pass, stored 120x smaller than the real rate. Stock \
               200 x 120 = 24000, which is exactly Tyre Spring Rate - keep that \
               relationship or the two passes disagree and the car behaves \
               inconsistently. Change Tyre Spring Rate and you must change this \
               too. Stock 200.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD54E8 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 200,
        range: None,
    },
    FieldDesc {
        id: "tyre_damping_rl",
        label: "Tyre Damping (Rear Left)",
        help: "How much the rear left tyre carcass damps its own bouncing \
               (separate from the damper). Higher = the tyre stops ringing sooner \
               after a bump; lower = it keeps oscillating. Affects all cars. \
               Stock 256.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD550C + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "tyre_damping_rr",
        label: "Tyre Damping (Rear Right)",
        help: "How much the rear right tyre carcass damps its own bouncing \
               (separate from the damper). Higher = the tyre stops ringing sooner \
               after a bump; lower = it keeps oscillating. Affects all cars. \
               Stock 256.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD550C + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "tyre_damping_fl",
        label: "Tyre Damping (Front Left)",
        help: "How much the front left tyre carcass damps its own bouncing \
               (separate from the damper). Higher = the tyre stops ringing sooner \
               after a bump; lower = it keeps oscillating. Affects all cars. \
               Stock 256.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD550C + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "tyre_damping_fr",
        label: "Tyre Damping (Front Right)",
        help: "How much the front right tyre carcass damps its own bouncing \
               (separate from the damper). Higher = the tyre stops ringing sooner \
               after a bump; lower = it keeps oscillating. Affects all cars. \
               Stock 256.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD550C + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 256,
        range: None,
    },
    FieldDesc {
        id: "spring_factor",
        label: "Spring Rate Scale",
        help: "A global multiplier on every spring setting in the garage. Higher \
               = each click of spring in your setup does more, so the whole range \
               gets stiffer; lower = softer. Use it to re-centre the setup range \
               rather than changing individual setups. Affects all cars. Stock \
               1966080.",
        subtab: SubTab::Suspension,
        tier: Tier::Basic,
        target: Target::Data(0xD54C4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1966080,
        range: None,
    },
    FieldDesc {
        id: "arb_factor",
        label: "Anti-Roll Bar Scale",
        help: "A global multiplier on both anti-roll bar settings. Higher = every \
               ARB click resists roll more, so the bars bite harder across the \
               range; lower = they matter less. The knob for making ARB choice \
               meaningful. Affects all cars. Stock 196608.",
        subtab: SubTab::Suspension,
        tier: Tier::Basic,
        target: Target::Data(0xD54CC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 196608,
        range: None,
    },
    FieldDesc {
        id: "free_length_rl",
        label: "Suspension Travel (Rear Left)",
        help: "The rear left suspension's free length - how much travel it has \
               before it runs out. Higher = more room to move before hitting the \
               bump stops; lower = it runs out of travel sooner and rides on the \
               stops. Stock gives the rear slightly more travel than the front. \
               Stock 283984.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5544 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 283984,
        range: None,
    },
    FieldDesc {
        id: "free_length_rr",
        label: "Suspension Travel (Rear Right)",
        help: "The rear right suspension's free length - how much travel it has \
               before it runs out. Higher = more room to move before hitting the \
               bump stops; lower = it runs out of travel sooner and rides on the \
               stops. Stock gives the rear slightly more travel than the front. \
               Stock 283984.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5544 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 283984,
        range: None,
    },
    FieldDesc {
        id: "free_length_fl",
        label: "Suspension Travel (Front Left)",
        help: "The front left suspension's free length - how much travel it has \
               before it runs out. Higher = more room to move before hitting the \
               bump stops; lower = it runs out of travel sooner and rides on the \
               stops. Stock gives the rear slightly more travel than the front. \
               Stock 278528.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5544 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 278528,
        range: None,
    },
    FieldDesc {
        id: "free_length_fr",
        label: "Suspension Travel (Front Right)",
        help: "The front right suspension's free length - how much travel it has \
               before it runs out. Higher = more room to move before hitting the \
               bump stops; lower = it runs out of travel sooner and rides on the \
               stops. Stock gives the rear slightly more travel than the front. \
               Stock 278528.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5544 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 278528,
        range: None,
    },
    FieldDesc {
        id: "bumpstop_rate_rl",
        label: "Bump-Stop Rate (Rear Left)",
        help: "How stiff the rear left bump stop is once the suspension runs out \
               of travel and hits it. Higher = a harsh, sudden stop; lower = it \
               squashes progressively. Affects all cars. Stock 1000.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5570 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1000,
        range: None,
    },
    FieldDesc {
        id: "bumpstop_rate_rr",
        label: "Bump-Stop Rate (Rear Right)",
        help: "How stiff the rear right bump stop is once the suspension runs out \
               of travel and hits it. Higher = a harsh, sudden stop; lower = it \
               squashes progressively. Affects all cars. Stock 1000.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5570 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1000,
        range: None,
    },
    FieldDesc {
        id: "bumpstop_rate_fl",
        label: "Bump-Stop Rate (Front Left)",
        help: "How stiff the front left bump stop is once the suspension runs out \
               of travel and hits it. Higher = a harsh, sudden stop; lower = it \
               squashes progressively. Affects all cars. Stock 1000.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5570 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1000,
        range: None,
    },
    FieldDesc {
        id: "bumpstop_rate_fr",
        label: "Bump-Stop Rate (Front Right)",
        help: "How stiff the front right bump stop is once the suspension runs \
               out of travel and hits it. Higher = a harsh, sudden stop; lower = \
               it squashes progressively. Affects all cars. Stock 1000.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5570 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1000,
        range: None,
    },
    FieldDesc {
        id: "packer_cap_rl",
        label: "Packer Cap (Rear Left)",
        help: "A cap on how far the rear left corner can compress - the packer. \
               Lower = the suspension is blocked from compressing as far, so it \
               bottoms out earlier. Stock allows the rear twice the travel of the \
               front. Stock 68800.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5590 + 0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 68800,
        range: None,
    },
    FieldDesc {
        id: "packer_cap_rr",
        label: "Packer Cap (Rear Right)",
        help: "A cap on how far the rear right corner can compress - the packer. \
               Lower = the suspension is blocked from compressing as far, so it \
               bottoms out earlier. Stock allows the rear twice the travel of the \
               front. Stock 68800.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5590 + 4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 68800,
        range: None,
    },
    FieldDesc {
        id: "packer_cap_fl",
        label: "Packer Cap (Front Left)",
        help: "A cap on how far the front left corner can compress - the packer. \
               Lower = the suspension is blocked from compressing as far, so it \
               bottoms out earlier. Stock allows the rear twice the travel of the \
               front. Stock 34400.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5590 + 8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 34400,
        range: None,
    },
    FieldDesc {
        id: "packer_cap_fr",
        label: "Packer Cap (Front Right)",
        help: "A cap on how far the front right corner can compress - the packer. \
               Lower = the suspension is blocked from compressing as far, so it \
               bottoms out earlier. Stock allows the rear twice the travel of the \
               front. Stock 34400.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5590 + 12),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 34400,
        range: None,
    },
    FieldDesc {
        id: "droop_stiffness",
        label: "Droop Stiffness Add",
        help: "Extra stiffness added as the suspension extends past its normal \
               range (droop), e.g. over a crest. Higher = the suspension resists \
               topping out more. Affects all cars. Stock 9600.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5540),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 9600,
        range: None,
    },
    FieldDesc {
        id: "bump_rebound_ratio",
        label: "Bump/Rebound Ratio",
        help: "How damping splits between compression (bump) and extension \
               (rebound). Stock 8192 is the balance point; raise it for \
               relatively more bump damping, lower it for more rebound. Affects \
               all cars. Stock 8192.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD55BC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8192,
        range: None,
    },
    FieldDesc {
        id: "damper_knee_pos",
        label: "Damper Knee (+)",
        help: "The damper velocity at which the damping curve changes slope, on \
               the compression side - the knee between low-speed and high-speed \
               damping. Higher = the damper stays in its low-speed behaviour to \
               higher velocities. Keep it mirrored with Damper Knee (-), which \
               should be the same magnitude negated. Affects all cars. Stock \
               43690.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD55AC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 43690,
        range: None,
    },
    FieldDesc {
        id: "damper_knee_neg",
        label: "Damper Knee (-)",
        help: "The same damper-curve knee as Damper Knee (+), but on the \
               extension side. SIGNED and NEGATIVE: keep it the mirror of the \
               positive knee (stock is exactly -43690 against +43690). Making the \
               pair asymmetric gives the damper a different knee compressing than \
               extending, which is possible but is not what stock does. Affects \
               all cars. Stock -43690.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD55B0),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: -43690,
        range: None,
    },
    FieldDesc {
        id: "bottoming_stiffness",
        label: "Bottoming Stiffness",
        help: "How hard the car resists once the floor itself is on the ground. \
               Higher = bottoming out feels like hitting a wall; lower = the car \
               grounds out softly. Affects all cars. Stock 240000.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5584),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 240000,
        range: None,
    },
    FieldDesc {
        id: "plank_wear_rate",
        label: "Plank Wear Rate",
        help: "How fast the wooden plank under the car wears away when it drags \
               on the track. Higher = it wears through sooner. Affects all cars. \
               Stock 4096.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5588),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4096,
        range: None,
    },
    FieldDesc {
        id: "heave_knee",
        label: "Soft-Limit Knee (Heave)",
        help: "Where the soft limit on heave movement starts to bite. Higher = \
               the car can heave further before the limit does anything. Pairs \
               with Soft-Limit Gain (Heave), which sets how hard it pushes back \
               once past this point. Stock 65536.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5624),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 65536,
        range: None,
    },
    FieldDesc {
        id: "pitch_knee",
        label: "Soft-Limit Knee (Pitch)",
        help: "Where the soft limit on pitch movement starts to bite. Higher = \
               the car can pitch further before the limit does anything. Pairs \
               with Soft-Limit Gain (Pitch), which sets how hard it pushes back \
               once past this point. Stock 134217728.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5620),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 134217728,
        range: None,
    },
    FieldDesc {
        id: "roll_knee",
        label: "Soft-Limit Knee (Roll)",
        help: "Where the soft limit on roll movement starts to bite. Higher = the \
               car can roll further before the limit does anything. Pairs with \
               Soft-Limit Gain (Roll), which sets how hard it pushes back once \
               past this point. Stock 134217728.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD561C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 134217728,
        range: None,
    },
    FieldDesc {
        id: "heave_gain",
        label: "Soft-Limit Gain (Heave)",
        help: "How hard the soft limit pushes back once heave passes its knee. \
               Higher = a firmer stop; 0 disables the limit. Pairs with \
               Soft-Limit Knee (Heave), which sets where it starts. Stock 65536.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5630),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 65536,
        range: None,
    },
    FieldDesc {
        id: "pitch_gain",
        label: "Soft-Limit Gain (Pitch)",
        help: "How hard the soft limit pushes back once pitch passes its knee. \
               Higher = a firmer stop; 0 disables the limit. Pairs with \
               Soft-Limit Knee (Pitch), which sets where it starts. Stock 16384.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD562C),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    FieldDesc {
        id: "roll_gain",
        label: "Soft-Limit Gain (Roll)",
        help: "How hard the soft limit pushes back once roll passes its knee. \
               Higher = a firmer stop; 0 disables the limit. Pairs with \
               Soft-Limit Knee (Roll), which sets where it starts. Stock 16384.",
        subtab: SubTab::Suspension,
        tier: Tier::Advanced,
        target: Target::Data(0xD5628),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    // ---- Aero: ground effect / ride height ----
    FieldDesc {
        id: "rake_reference",
        label: "Reference Rake",
        help: "The rake (nose-down angle) the aero is tuned around. The further \
               your setup's actual rake is from this, the more downforce changes. \
               Raise it and the car wants to run more nose-down to hit its aero \
               sweet spot. Affects all cars. Stock 13760.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Data(0xD5750),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 13760,
        range: None,
    },
    FieldDesc {
        id: "rake_sens_total",
        label: "Rake Sensitivity (Total)",
        help: "How much total downforce changes as rake moves away from Reference \
               Rake. Higher = ride height and rake matter more for overall grip; \
               0 = total downforce ignores rake entirely. Affects all cars. Stock \
               3121.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EC0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3121,
        range: None,
    },
    FieldDesc {
        id: "rake_sens_split",
        label: "Rake Sensitivity (Split)",
        help: "How much rake shifts downforce BETWEEN the axles, as opposed to \
               changing the total (that's Rake Sensitivity (Total)). Higher = \
               rake changes the aero balance more, so ride height becomes a \
               handling-balance tool. Affects all cars. Stock 3121.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EB8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 3121,
        range: None,
    },
    FieldDesc {
        id: "front_ride_sens",
        label: "Front Ride Sensitivity",
        help: "How much front downforce responds to front ride height \
               specifically, on top of the rake effects. Higher = the front end \
               is fussier about how low it runs. Affects all cars. Stock 1561.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EBC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1561,
        range: None,
    },
    FieldDesc {
        id: "front_ride_ref",
        label: "Front Ride Reference",
        help: "The front ride height that Front Ride Sensitivity measures against \
               - the front's aero sweet spot. Affects all cars. Stock 12040.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5748),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12040,
        range: None,
    },
    FieldDesc {
        id: "ge_clamp_rear",
        label: "GE Ride Clamp Rear",
        help: "A ceiling on how much the rear ride height is allowed to alter \
               downforce, so extreme heights can't produce absurd aero. Lower = \
               the rear ride-height effect saturates sooner. Affects all cars. \
               Stock 86000.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD55D4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 86000,
        range: None,
    },
    FieldDesc {
        id: "ge_clamp_front",
        label: "GE Ride Clamp Front",
        help: "The front twin of GE Ride Clamp Rear. Stock clamps the front \
               considerably tighter than the rear (51600 vs 86000). Affects all \
               cars. Stock 51600.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD55D0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 51600,
        range: None,
    },
    FieldDesc {
        id: "ge_master_scale",
        label: "Ground-Effect Master",
        help: "A single multiplier over the WHOLE ride-height/rake downforce \
               correction. Stock 16384 is exactly x1.0, so it is dormant - it \
               sits there doing nothing until you change it. Raise it to amplify \
               every ground-effect behaviour at once, drop it toward 0 to switch \
               ride-height aero off entirely. A hidden global switch rather than \
               a fine-tuning knob. Affects all cars. Stock 16384.",
        subtab: SubTab::Aero,
        tier: Tier::Basic,
        target: Target::Data(0xD5EE8),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 16384,
        range: None,
    },
    // ---- Steering ----
    FieldDesc {
        id: "steer_master_clamp",
        label: "Max Steering Lock",
        help: "The hard ceiling on how far the front wheels can be steered, \
               whatever the input. Higher = more lock available for tight corners \
               and catching slides; lower = the car cannot turn as sharply at \
               all. Affects all cars. Stock 6372.",
        subtab: SubTab::Steering,
        tier: Tier::Basic,
        // Plan marked this width `2?`. The listing settles it as `dd` -> width 4:
        // `000D61C4 dword_0_D61C4 dd 18E4h`. The block is genuinely MIXED
        // (D61C0 dw / D61C4 dd / D61C8 dw), and width 2 would ALSO read 6372
        // here because the high word is zero - so the stock test cannot tell
        // the widths apart. Only the directive does.
        target: Target::Data(0xD61C4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 6372,
        range: None,
    },
    FieldDesc {
        id: "steer_base_lock",
        label: "Manual Base Lock",
        help: "The baseline steering lock used when you are steering manually, \
               before the speed-sensitive assist scales it. Higher = more \
               steering per unit of input at the low-speed end. Sits under Max \
               Steering Lock, which caps the result. Stock 2048.",
        subtab: SubTab::Steering,
        tier: Tier::Advanced,
        target: Target::Data(0x1731E4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 2048,
        range: None,
    },
    FieldDesc {
        id: "tc_ramp_rate",
        label: "Traction-Control Ramp",
        help: "How fast the traction-control aid winds power back when it detects \
               wheelspin. Higher = it cuts in more abruptly; lower = it eases in. \
               Only affects players driving with the traction-control aid \
               switched on. Stock 4096.",
        subtab: SubTab::Steering,
        tier: Tier::Advanced,
        // Listing confirms `dw` -> width 2: `000C9722 word_0_C9722 dw 1000h`.
        target: Target::Data(0xC9722),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4096,
        range: None,
    },
    // ---- Tyres: the tyre-model coefficient block ----
    // Data constants living INSIDE the code segment (0x1A93F-0x1A9B6), hence
    // Target::CodeData (file = IDA + CODE_BASE, with NO +1 opcode skip - these
    // are not instruction operands). Each pair is rear @addr, front @addr+4.
    // All 12 verified byte-for-byte against a pristine GP2.EXE.
    FieldDesc {
        id: "tyre_k3_rear",
        label: "Base Lateral Grip (Rear)",
        help: "The rear tyre's baseline cornering grip - the single biggest lever \
               on front/rear balance. Raise it for more rear grip (understeer); \
               lower it for a looser, more oversteery car. Read this together \
               with Base Lateral Grip (Front): what matters is the RATIO between \
               them, and stock already gives the rear noticeably more (327616 vs \
               251904). Affects all cars. Stock 327616.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        // The oversteer/understeer knob: k3 rear vs front IS the grip balance.
        target: Target::CodeData(0x1A95F),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 327616,
        range: None,
    },
    FieldDesc {
        id: "tyre_k3_front",
        label: "Base Lateral Grip (Front)",
        help: "The front tyre's baseline cornering grip. Raise it for more front \
               bite (oversteer); lower it for understeer. Only its ratio to Base \
               Lateral Grip (Rear) really matters - move both together and you \
               just change overall grip. Affects all cars. Stock 251904.",
        subtab: SubTab::Tyres,
        tier: Tier::Basic,
        target: Target::CodeData(0x1A963),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 251904,
        range: None,
    },
    FieldDesc {
        id: "tyre_k1_rear",
        label: "Load Sensitivity Slope (Rear)",
        help: "How fast the rear tyre LOSES grip per unit of load - real tyres \
               get less grippy the harder you push them, and this is that \
               falloff. SIGNED and negative: more negative = a sharper penalty \
               for loading the tyre up, so weight transfer costs more grip. Works \
               with Load Sensitivity Base (Rear). Affects all cars. Stock -87327.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A94F),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: -87327,
        range: None,
    },
    FieldDesc {
        id: "tyre_k1_front",
        label: "Load Sensitivity Slope (Front)",
        help: "The front twin of Load Sensitivity Slope (Rear). SIGNED and \
               negative; stock makes the FRONT more load-sensitive than the rear \
               (-117651 vs -87327), so the front loses grip faster as it is \
               loaded. Affects all cars. Stock -117651.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A953),
        width: 4,
        signed: true,
        encoding: Encoding::Raw,
        stock: -117651,
        range: None,
    },
    FieldDesc {
        id: "tyre_k2_rear",
        label: "Load Sensitivity Base (Rear)",
        help: "The constant term of the rear tyre's load-sensitivity curve - its \
               grip at the reference load, before the slope eats into it. Pairs \
               with Load Sensitivity Slope (Rear). Affects all cars. Stock 33161.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A957),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 33161,
        range: None,
    },
    FieldDesc {
        id: "tyre_k2_front",
        label: "Load Sensitivity Base (Front)",
        help: "The front twin of Load Sensitivity Base (Rear). Affects all cars. \
               Stock 35307.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A95B),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 35307,
        range: None,
    },
    FieldDesc {
        id: "tyre_k5_rear",
        label: "Optimal Load (Rear)",
        help: "The load at which the rear tyre is happiest - where it gives its \
               best grip. Move it and you change which corners and which fuel \
               loads suit the car. Affects all cars. Stock 1064.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A967),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 1064,
        range: None,
    },
    FieldDesc {
        id: "tyre_k5_front",
        label: "Optimal Load (Front)",
        help: "The front twin of Optimal Load (Rear); stock sets the front's \
               optimum a little lower (972 vs 1064). Affects all cars. Stock 972.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A96B),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 972,
        range: None,
    },
    FieldDesc {
        id: "tyre_k6_rear",
        label: "Curve Shape Slope (Rear)",
        help: "Shapes how peaky the rear tyre's grip curve is - how sharply grip \
               falls away once you pass the limit. Higher = a more knife-edge \
               tyre that snaps; lower = a progressive one that slides gently. \
               Pairs with Curve Shape Base (Rear). Affects all cars. Stock 18292.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A977),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 18292,
        range: None,
    },
    FieldDesc {
        id: "tyre_k6_front",
        label: "Curve Shape Slope (Front)",
        help: "The front twin of Curve Shape Slope (Rear). Stock makes the front \
               curve far peakier than the rear (70249 vs 18292). Affects all \
               cars. Stock 70249.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A97B),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 70249,
        range: None,
    },
    FieldDesc {
        id: "tyre_k7_rear",
        label: "Curve Shape Base (Rear)",
        help: "The constant term of the rear tyre's curve-shape function (see \
               Curve Shape Slope (Rear)). Affects all cars. Stock 4000.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A97F),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 4000,
        range: None,
    },
    FieldDesc {
        id: "tyre_k7_front",
        label: "Curve Shape Base (Front)",
        help: "The front twin of Curve Shape Base (Rear). Affects all cars. Stock \
               6404.",
        subtab: SubTab::Tyres,
        tier: Tier::Advanced,
        target: Target::CodeData(0x1A983),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 6404,
        range: None,
    },
    // ---- Wing damage (CalcBothWings @0x16AA5) ----
    // The plan listed these as code immediates (imm 0x3000 / 0x5000). They are
    // NOT: CalcBothWings contains no immediates, it does `imul dword_0_D5EAC` /
    // `D5EB0` / `D5EB4` (0x16ABE / 0x16ADE / 0x16B01). Plain data dwords, so
    // Target::Data - no operand-offset guesswork. There are THREE, not two.
    FieldDesc {
        id: "wing_dmg_loss",
        label: "Wing Damage: Rear Loss",
        help: "How much rear downforce survives once the rear wing is damaged. \
               Stock 12288 is x0.75, i.e. a damaged rear wing keeps three \
               quarters of its downforce. Lower = a broken rear wing hurts more. \
               Affects all cars. Stock 12288.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EAC),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12288,
        range: None,
    },
    FieldDesc {
        id: "wing_dmg_front_boost",
        label: "Wing Damage: Front Boost",
        help: "Applied to FRONT downforce when the rear wing is damaged but the \
               front is still intact. Stock 20480 is x1.25 - a boost, not a loss, \
               which shifts the balance forward and makes a rear-wing-damaged car \
               pointy rather than merely slow. Affects all cars. Stock 20480.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        target: Target::Data(0xD5EB0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 20480,
        range: None,
    },
    FieldDesc {
        id: "wing_dmg_front_loss",
        label: "Wing Damage: Front Loss",
        help: "How much front downforce survives once the FRONT wing is damaged \
               (and the rear is not). Stock 12288 is x0.75, mirroring the rear. \
               Lower = a broken front wing costs more turn-in. Affects all cars. \
               Stock 12288.",
        subtab: SubTab::Aero,
        tier: Tier::Advanced,
        // Not in the plan's table (it listed only two wing fields), but it is the
        // front-damage twin of wing_dmg_loss - `imul dword_0_D5EB4` @0x16B01 on
        // the [esi+0BBh]&0x80 branch. Shipping without it would leave front-wing
        // damage uneditable while rear-wing damage is exposed.
        target: Target::Data(0xD5EB4),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12288,
        range: None,
    },
    // ---- Code immediates (verified operand offsets) ----
    // Target::Code assumes the operand sits at +1 (immediately after a 1-byte
    // opcode). That is FALSE for `cmp [mem], imm`, which encodes a disp32 first
    // and puts the operand at +6/+7. Every offset below was confirmed by reading
    // the real bytes out of a pristine GP2.EXE, hence Target::Direct with the
    // instruction and IDA address recorded.
    FieldDesc {
        id: "practice_fuel_laps",
        label: "Practice Fuel Laps",
        help: "How many laps of fuel the car is given in free practice. Higher = \
               a heavier, slower practice car; lower = lighter. Note the \
               qualifying equivalent (Qualifying Fuel Laps) is a data field and \
               gets +1 in code, whereas this one is used as-is. Stock 12.",
        subtab: SubTab::MassGrip,
        tier: Tier::Advanced,
        // `mov eax, 12` @ IDA 0x2C543 (B8 0C 00 00 00); operand +1 -> file 0xA4798.
        target: Target::Direct(0xA4798),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 12,
        range: None,
    },
    FieldDesc {
        id: "upshift_block_rpm",
        label: "Upshift Block Wheel-RPM",
        help: "The wheel-RPM below which an upshift is refused, so the box will \
               not change up while the wheels are turning too slowly. Higher = \
               upshifts are blocked over a wider range. Cross-check with Rev \
               Limiter and Max RPM if you are building a low-rev engine. Stock \
               9000.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        // `cmp word_0_D4024, 2328h` @ IDA 0x19E4A
        // (66 81 3D 24 40 01 00 | 28 23); operand at +7, width 2 -> file 0x920A5.
        target: Target::Direct(0x920A5),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 9000,
        range: None,
    },
    FieldDesc {
        id: "reengage_rpm_hi",
        label: "Shift Re-Engage RPM (1st)",
        help: "The wheel-RPM the car must reach before drive is re-engaged after \
               a shift, when in FIRST gear. Higher = a longer pause before the \
               engine reconnects. Pairs with Shift Re-Engage RPM (2nd+), which \
               stock sets lower (8000). Relevant when building a low-rev engine - \
               see Rev Limiter. Stock 9000.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        // `cmp dword ptr word_0_D4024, 9000` @ IDA 0x1A1FE (1st-gear branch)
        // (81 3D 24 40 01 00 | 28 23 00 00); operand at +6 -> file 0x92458.
        target: Target::Direct(0x92458),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 9000,
        range: None,
    },
    FieldDesc {
        id: "reengage_rpm_lo",
        label: "Shift Re-Engage RPM (2nd+)",
        help: "The same re-engage threshold as Shift Re-Engage RPM (1st), but for \
               second gear and above, where stock uses a lower figure (8000 vs \
               9000). Higher = a longer pause before drive resumes after each \
               shift. Stock 8000.",
        subtab: SubTab::Engine,
        tier: Tier::Advanced,
        // `cmp dword ptr word_0_D4024, 1F40h` @ IDA 0x1A1E7 (gear >= 2 branch)
        // (81 3D 24 40 01 00 | 40 1F 00 00); operand at +6 -> file 0x92441.
        target: Target::Direct(0x92441),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8000,
        range: None,
    },
    FieldDesc {
        id: "kerb_a_x1",
        label: "Kerb A Profile: First Width Point",
        help: "The first width point of the default kerb profile A. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 110.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        // These are the DEFAULTS written into dSetByCmdCA/CBarg* at track load
        // (`mov [mem], imm32` = C7 05 <disp32> <imm32>, operand at +6). A track
        // carrying its own 0xCA / 0xCB command overrides them, so editing these
        // only changes tracks that DON'T specify a kerb profile.
        target: Target::Direct(0xEB7B1),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 110,
        range: None,
    },
    FieldDesc {
        id: "kerb_a_x2",
        label: "Kerb A Profile: Second Width Point",
        help: "The second width point of the default kerb profile A. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 300.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7BB),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 300,
        range: None,
    },
    FieldDesc {
        id: "kerb_a_h1",
        label: "Kerb A Profile: First Height Point",
        help: "The first height point of the default kerb profile A. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 18.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7C5),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 18,
        range: None,
    },
    FieldDesc {
        id: "kerb_a_h2",
        label: "Kerb A Profile: Second Height Point",
        help: "The second height point of the default kerb profile A. Only \
               applies to tracks that do not carry their own kerb-profile command \
               - a track that does overrides this. Affects all cars. Stock 26.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7CF),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 26,
        range: None,
    },
    FieldDesc {
        id: "kerb_b_x1",
        label: "Kerb B Profile: First Width Point",
        help: "The first width point of the default kerb profile B. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 178.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7D9),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 178,
        range: None,
    },
    FieldDesc {
        id: "kerb_b_x2",
        label: "Kerb B Profile: Second Width Point",
        help: "The second width point of the default kerb profile B. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 356.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7E3),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 356,
        range: None,
    },
    FieldDesc {
        id: "kerb_b_h1",
        label: "Kerb B Profile: First Height Point",
        help: "The first height point of the default kerb profile B. Only applies \
               to tracks that do not carry their own kerb-profile command - a \
               track that does overrides this. Affects all cars. Stock 14.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7ED),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 14,
        range: None,
    },
    FieldDesc {
        id: "kerb_b_h2",
        label: "Kerb B Profile: Second Height Point",
        help: "The second height point of the default kerb profile B. Only \
               applies to tracks that do not carry their own kerb-profile command \
               - a track that does overrides this. Affects all cars. Stock 24.",
        subtab: SubTab::Surfaces,
        tier: Tier::Advanced,
        target: Target::Direct(0xEB7F7),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 24,
        range: None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_ids() {
        let mut ids: Vec<&str> = PHYSICS_FIELDS.iter().map(|f| f.id).collect();
        ids.sort();
        let len = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), len, "duplicate field id found");
    }

    #[test]
    fn stock_values_match_real_exe() {
        let Ok(p) = std::env::var("GP2WS_TEST_EXE") else {
            return;
        }; // skip if unset
        let img = crate::exe::ExeImage::load(std::path::Path::new(&p)).unwrap();
        let crate::calibration::Calibration::Calibrated { delta } =
            crate::calibration::calibrate(&img)
        else {
            panic!("not calibrated")
        };
        for f in PHYSICS_FIELDS {
            // `stock` is already the decoded human value (same value the
            // reset-to-stock button writes), and `read` decodes internally —
            // so compare directly. Decoding the expected side here would
            // double-decode any Bias/Fixed field.
            assert_eq!(f.read(&img, delta), f.stock, "field {} mismatch", f.id);
        }
    }
}
