# Field Testing Checklist

Tracks in-game verification of every editable field. A field is `verified` only
after it has been changed in the EXE, run in the game, and observed to do what
its help text claims. `untested` is the honest default — it does not mean the
field is wrong, only that nobody has driven it yet.

Status values: `untested` | `verified` | `suspect` (behaves unlike its help) |
`dead` (confirmed no in-game effect).

Every new field batch appends its rows here as `untested`.

## Physics fields

| id | label | sub-tab | tier | status | notes |
|---|---|---|---|---|---|
| `rev_limiter` | Rev Limiter | Engine | Basic | untested | |
| `max_rpm` | Max RPM | Engine | Basic | untested | |
| `rpm_light_1` | RPM Light 1 | Engine | Basic | untested | |
| `rpm_light_2` | RPM Light 2 | Engine | Basic | untested | |
| `rpm_light_3` | RPM Light 3 | Engine | Basic | untested | |
| `rpm_light_4` | RPM Light 4 | Engine | Basic | untested | |
| `df_scale` | Downforce Scale | Aero | Basic | untested | |
| `rear_df_slope` | Rear Downforce Slope | Aero | Basic | untested | |
| `rear_df_floor` | Rear Downforce Floor | Aero | Basic | untested | |
| `rear_drag_slope` | Rear Drag Slope | Aero | Basic | untested | |
| `rear_drag_floor` | Rear Drag Floor | Aero | Basic | untested | |
| `front_wing_slope` | Front Wing Slope | Aero | Adv | untested | |
| `front_wing_floor` | Front Wing Floor | Aero | Adv | untested | |
| `front_aero_scale` | Front Aero Scale | Aero | Adv | untested | |
| `lateral_drag_x` | Lateral Drag X | Aero | Adv | untested | |
| `lateral_drag_y` | Lateral Drag Y | Aero | Adv | untested | |
| `rear_wing_drag_factor` | Rear Wing Drag Factor | Aero | Adv | untested | |
| `brake_force` | Brake Force | Brakes | Basic | untested | |
| `ai_brake_strength` | AI Brake Strength (shift) | Brakes | Basic | untested | |
| `brake_fail_mult` | Brake Fail Multiplier | Brakes | Adv | untested | |
| `abs_threshold` | ABS Threshold | Brakes | Adv | untested | |
| `std_weight` | Standard Weight | MassGrip | Basic | untested | |
| `norm_weight` | Normal Weight | MassGrip | Basic | untested | |
| `gravity` | Gravity | MassGrip | Adv | untested | |
| `fuel_factor` | Fuel Factor | MassGrip | Adv | untested | |
| `packer_factor` | Packer Factor | MassGrip | Adv | untested | |
| `rebound_factor` | Rebound Factor | MassGrip | Adv | untested | |
| `min_grip_clamp` | Min Grip Clamp | MassGrip | Adv | untested | |
| `cc_grip_qual` | CC Grip (Qualifying) | MassGrip | Basic | untested | |
| `cc_grip_race` | CC Grip (Race) | MassGrip | Basic | untested | |
| `tow_strength` | Tow Strength | Slipstream | Basic | untested | |
| `tow_reach` | Tow Reach | Slipstream | Basic | untested | |
| `tow_align_width` | Tow Align Width | Slipstream | Basic | untested | |
| `ai_tow_strength` | AI Tow Strength | Slipstream | Basic | untested | |
| `tow_max_wake` | Tow Max Wake | Slipstream | Adv | untested | |
| `tow_max_range` | Tow Max Range | Slipstream | Adv | untested | |
| `tow_min_speed` | Tow Min Speed | Slipstream | Adv | untested | |
| `ai_follow_base_1` | AI Follow Base 1 | Slipstream | Adv | untested | |
| `ai_follow_base_2` | AI Follow Base 2 | Slipstream | Adv | untested | |
| `ai_follow_base_3` | AI Follow Base 3 | Slipstream | Adv | untested | |
| `ai_follow_base_4` | AI Follow Base 4 | Slipstream | Adv | untested | |
| `ai_follow_base_5` | AI Follow Base 5 | Slipstream | Adv | untested | |
| `ai_follow_base_6` | AI Follow Base 6 | Slipstream | Adv | untested | |
| `ai_follow_base_7` | AI Follow Base 7 | Slipstream | Adv | untested | |
| `ai_follow_floor_1` | AI Follow Floor 1 | Slipstream | Adv | untested | |
| `ai_follow_floor_2` | AI Follow Floor 2 | Slipstream | Adv | untested | |
| `ai_follow_floor_3` | AI Follow Floor 3 | Slipstream | Adv | untested | |
| `tyre_grip_a` | Tyre Wear Sensitivity A | Tyres | Basic | untested | |
| `tyre_grip_b` | Tyre Wear Sensitivity B | Tyres | Basic | untested | |
| `tyre_grip_c` | Tyre Wear Sensitivity C | Tyres | Basic | untested | |
| `tyre_grip_d` | Tyre Wear Sensitivity D | Tyres | Basic | untested | |
| `tyre_base_a` | Tyre Base Grip A | Tyres | Basic | untested | |
| `tyre_base_b` | Tyre Base Grip B | Tyres | Basic | untested | |
| `tyre_base_c` | Tyre Base Grip C | Tyres | Basic | untested | |
| `tyre_base_d` | Tyre Base Grip D | Tyres | Basic | untested | |
| `tyre_worn_floor` | Worn Tyre Floor | Tyres | Basic | untested | |

**Count:** 56 fields (registry: `PHYSICS_FIELDS` + `TYRE_FIELDS`).

## Magic data (24 per-track tables)

The 24 magic tables are edited per slot in the Magic Data tab and are not part
of the physics registry. T6 is confirmed `dead` (the game ignores it); the rest
are `untested` in the same sense as above.

## Power curve

The 36 power-curve points are edited as a block in the Power Curve tab and are
verified as a unit rather than per point: `untested`.
