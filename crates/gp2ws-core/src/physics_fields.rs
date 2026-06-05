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
               so the dashboard lights still make sense. Stock 7146.",
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
               cosmetic; raise it alongside the rev limiter / max RPM. Stock 8146.",
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
               cosmetic; raise it alongside the rev limiter / max RPM. Stock 8496.",
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
               Stock 8696.",
        subtab: SubTab::Engine,
        tier: Tier::Basic,
        target: Target::Data(0xD670C),
        width: 2,
        signed: false,
        encoding: Encoding::Raw,
        stock: 8696,
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
               way. Stock 1441792.",
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
               not brake at all. Experimental - test in-game.",
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
        help: "How strong YOUR slipstream is when you tuck in behind another car. \
               Higher = a bigger speed boost in the draft; 0 turns the player \
               draft off entirely. Player only. Stock 262144.",
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
        help: "How far back behind another car your slipstream still works. \
               Higher = you can catch the draft from further away; lower = you \
               must get very close. Player only. Stock 384.",
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
               tow when not perfectly lined up. Player only. Stock 512.",
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
        label: "AI Tow Strength",
        help: "A speed-scaled boost that lets AI cars slingshot past the car \
               ahead. Stock 0 means OFF - the developers left it disabled. Raise \
               it to enable; it only ADDS tow (can't go below stock) and a little \
               goes a long way, so test in small steps. Stock 0.",
        subtab: SubTab::Slipstream,
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
               Player only. Stock 256.",
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
               Player only. Stock 7.",
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
               draft to higher speeds; lower it to allow it sooner. Player only. \
               Stock 2816.",
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
        label: "AI Follow Base 1",
        help: "One of four internal thresholds (1-4) that decide WHEN and in which \
               mode the AI follows or backs off the car ahead. Very fiddly, no \
               intuitive scale, and the exact direction needs in-game testing. \
               Stock -4096.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 2",
        help: "One of four internal thresholds (1-4) that decide WHEN and in which \
               mode the AI follows or backs off the car ahead. Very fiddly, no \
               intuitive scale, and the exact direction needs in-game testing. \
               Stock -3072.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 3",
        help: "One of four internal thresholds (1-4) that decide WHEN and in which \
               mode the AI follows or backs off the car ahead. Very fiddly, no \
               intuitive scale, and the exact direction needs in-game testing. \
               Stock -24576.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 4",
        help: "One of four internal thresholds (1-4) that decide WHEN and in which \
               mode the AI follows or backs off the car ahead. Very fiddly, no \
               intuitive scale, and the exact direction needs in-game testing. \
               Stock -1024.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 5",
        help: "A clamp (one of 5/6/7) that limits how hard the AI catches up to / \
               slingshots past the car ahead. Lower these to rein in aggressive \
               AI drafting. Fiddly, no intuitive scale - test in-game. Stock -2048.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 6",
        help: "A clamp (one of 5/6/7) that limits how hard the AI catches up to / \
               slingshots past the car ahead. Lower these to rein in aggressive \
               AI drafting. Fiddly, no intuitive scale - test in-game. Stock -4096.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Base 7",
        help: "A clamp (one of 5/6/7) that limits how hard the AI catches up to / \
               slingshots past the car ahead. Lower these to rein in aggressive \
               AI drafting. Fiddly, no intuitive scale - test in-game. Stock -512.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Floor 1",
        help: "Advanced: the minimum value of the AI's follow target (one of \
               three). Sets a lower bound on how the AI tracks the car ahead. \
               No intuitive scale - test by feel. Stock 64512.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Floor 2",
        help: "Advanced: the minimum value of the AI's follow target (one of \
               three). Sets a lower bound on how the AI tracks the car ahead. \
               No intuitive scale - test by feel. Stock 53248.",
        subtab: SubTab::Slipstream,
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
        label: "AI Follow Floor 3",
        help: "Advanced: the minimum value of the AI's follow target (one of \
               three). Sets a lower bound on how the AI tracks the car ahead. \
               No intuitive scale - test by feel. Stock 20480.",
        subtab: SubTab::Slipstream,
        tier: Tier::Advanced,
        target: Target::Data(0xD5FF0),
        width: 4,
        signed: false,
        encoding: Encoding::Raw,
        stock: 0x5000,
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
