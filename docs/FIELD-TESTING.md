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
| `rev_limiter` | Rev Limiter | Engine | Basic | untested |  |
| `max_rpm` | Max RPM | Engine | Basic | untested |  |
| `rpm_light_1` | RPM Light 1 | Engine | Basic | untested |  |
| `rpm_light_2` | RPM Light 2 | Engine | Basic | untested |  |
| `rpm_light_3` | RPM Light 3 | Engine | Basic | untested |  |
| `rpm_light_4` | RPM Light 4 | Engine | Basic | untested |  |
| `diff_lock` | Rear Diff Lock | Drivetrain | Basic | untested |  |
| `final_drive` | Final Drive Divisor | Drivetrain | Basic | untested |  |
| `gearing_base_1` | Gearing Base 1 | Drivetrain | Adv | untested |  |
| `gearing_base_2` | Gearing Base 2 | Drivetrain | Adv | untested |  |
| `shift_cut_player` | Shift Cut Duration | Drivetrain | Basic | untested |  |
| `shift_cut_ai` | Shift Cut Duration (AI) | Drivetrain | Basic | untested |  |
| `downshift_guard` | Downshift Over-Rev Guard | Drivetrain | Adv | untested |  |
| `downshift_margin` | Downshift Table Margin | Drivetrain | Adv | untested |  |
| `min_upshift_speed` | Min Auto-Upshift Speed | Drivetrain | Adv | untested |  |
| `clutch_blend_gain` | Clutch Engagement Gain | Drivetrain | Adv | untested |  |
| `clutch_rpm_lag` | Clutch RPM Lag | Drivetrain | Adv | untested |  |
| `clutch_slip_decay` | Clutch Slip Decay | Drivetrain | Adv | untested |  |
| `engine_spin_down` | Engine-Off Spin-Down | Drivetrain | Adv | untested |  |
| `spin_gain_driven_rl` | Wheelspin Gain (Rear Left) | Drivetrain | Adv | untested |  |
| `spin_gain_driven_rr` | Wheelspin Gain (Rear Right) | Drivetrain | Adv | untested |  |
| `spin_gain_driven_fl` | Wheelspin Gain (Front Left) | Drivetrain | Adv | untested |  |
| `spin_gain_driven_fr` | Wheelspin Gain (Front Right) | Drivetrain | Adv | untested |  |
| `spin_gain_lock_rl` | Lock-Up Gain (Rear Left) | Drivetrain | Adv | untested |  |
| `spin_gain_lock_rr` | Lock-Up Gain (Rear Right) | Drivetrain | Adv | untested |  |
| `spin_gain_lock_fl` | Lock-Up Gain (Front Left) | Drivetrain | Adv | untested |  |
| `spin_gain_lock_fr` | Lock-Up Gain (Front Right) | Drivetrain | Adv | untested |  |
| `slip_decay_rl` | Slip Decay Gain (Rear Left) | Drivetrain | Adv | untested |  |
| `slip_decay_rr` | Slip Decay Gain (Rear Right) | Drivetrain | Adv | untested |  |
| `slip_decay_fl` | Slip Decay Gain (Front Left) | Drivetrain | Adv | untested |  |
| `slip_decay_fr` | Slip Decay Gain (Front Right) | Drivetrain | Adv | untested |  |
| `cog_rear_arm` | CoG -> Rear Axle | Chassis | Basic | untested |  |
| `cog_front_arm` | CoG -> Front Axle | Chassis | Basic | untested |  |
| `track_rear` | Rear Track Width | Chassis | Basic | untested |  |
| `track_front` | Front Track Width | Chassis | Basic | untested |  |
| `cog_height` | CoG Height | Chassis | Basic | untested |  |
| `gyr_yaw` | Yaw Gyration Radius | Chassis | Basic | untested |  |
| `gyr_pitch` | Pitch Gyration Radius | Chassis | Adv | untested |  |
| `gyr_roll` | Roll Gyration Radius | Chassis | Adv | untested |  |
| `unsprung_rear` | Rear Unsprung Mass | Chassis | Adv | untested |  |
| `unsprung_front` | Front Unsprung Mass | Chassis | Adv | untested |  |
| `inertia_fuel` | Inertia Reference Fuel | Chassis | Adv | untested |  |
| `camber_rl` | Camber (Rear Left) | Chassis | Adv | untested |  |
| `camber_rr` | Camber (Rear Right) | Chassis | Adv | untested |  |
| `camber_fl` | Camber (Front Left) | Chassis | Adv | untested |  |
| `camber_fr` | Camber (Front Right) | Chassis | Adv | untested |  |
| `df_scale` | Downforce Scale | Aero | Basic | untested |  |
| `rear_df_slope` | Rear Downforce Slope | Aero | Basic | untested |  |
| `rear_df_floor` | Rear Downforce Floor | Aero | Basic | untested |  |
| `rear_drag_slope` | Rear Drag Slope | Aero | Basic | untested |  |
| `rear_drag_floor` | Rear Drag Floor | Aero | Basic | untested |  |
| `front_wing_slope` | Front Wing Slope | Aero | Adv | untested |  |
| `front_wing_floor` | Front Wing Floor | Aero | Adv | untested |  |
| `front_aero_scale` | Front Aero Scale | Aero | Adv | untested |  |
| `lateral_drag_x` | Lateral Drag X | Aero | Adv | untested |  |
| `lateral_drag_y` | Lateral Drag Y | Aero | Adv | untested |  |
| `rear_wing_drag_factor` | Rear Wing Drag Factor | Aero | Adv | untested |  |
| `brake_force` | Brake Force | Brakes | Basic | untested |  |
| `ai_brake_strength` | AI Brake Strength (shift) | Brakes | Basic | untested |  |
| `brake_fail_mult` | Brake Fail Multiplier | Brakes | Adv | untested |  |
| `abs_threshold` | ABS Threshold | Brakes | Adv | untested |  |
| `std_weight` | Standard Weight | MassGrip | Basic | untested |  |
| `norm_weight` | Normal Weight | MassGrip | Basic | untested |  |
| `gravity` | Gravity | MassGrip | Adv | untested |  |
| `fuel_factor` | Fuel Factor | MassGrip | Adv | untested |  |
| `packer_factor` | Packer Factor | MassGrip | Adv | untested |  |
| `rebound_factor` | Rebound Factor | MassGrip | Adv | untested |  |
| `min_grip_clamp` | Min Grip Clamp | MassGrip | Adv | untested |  |
| `cc_grip_qual` | CC Grip (Qualifying) | MassGrip | Basic | untested |  |
| `cc_grip_race` | CC Grip (Race) | MassGrip | Basic | untested |  |
| `tow_strength` | Tow Strength | Slipstream | Basic | untested |  |
| `tow_reach` | Tow Reach | Slipstream | Basic | untested |  |
| `tow_align_width` | Tow Align Width | Slipstream | Basic | untested |  |
| `ai_tow_strength` | AI Speed-Scaled Braking | AiRacecraft | Basic | untested |  |
| `tow_max_wake` | Tow Max Wake | Slipstream | Adv | untested |  |
| `tow_max_range` | Tow Max Range | Slipstream | Adv | untested |  |
| `tow_min_speed` | Tow Min Speed | Slipstream | Adv | untested |  |
| `ai_follow_base_1` | AI Avoidance Engage | AiRacecraft | Adv | untested |  |
| `ai_follow_base_2` | AI Leader-Decel Match | AiRacecraft | Adv | untested |  |
| `ai_follow_base_3` | AI Close-Follow Select | AiRacecraft | Adv | untested |  |
| `ai_follow_base_4` | AI Close-Follow (Damaged) | AiRacecraft | Adv | untested |  |
| `ai_follow_base_5` | AI Brake Cap: Hold-Back | AiRacecraft | Adv | untested |  |
| `ai_follow_base_6` | AI Brake Cap: Sliding | AiRacecraft | Adv | untested |  |
| `ai_follow_base_7` | AI Brake Cap: Corner Squeeze | AiRacecraft | Adv | untested |  |
| `ai_follow_floor_1` | AI Heavy-Braking Flag | AiRacecraft | Adv | untested |  |
| `ai_follow_floor_2` | AI Max Braking / Tick | AiRacecraft | Adv | untested |  |
| `ai_follow_floor_3` | AI Avoidance Clamp | AiRacecraft | Adv | untested |  |
| `tyre_grip_a` | Tyre Wear Sensitivity A | Tyres | Basic | untested |  |
| `tyre_grip_b` | Tyre Wear Sensitivity B | Tyres | Basic | untested |  |
| `tyre_grip_c` | Tyre Wear Sensitivity C | Tyres | Basic | untested |  |
| `tyre_grip_d` | Tyre Wear Sensitivity D | Tyres | Basic | untested |  |
| `tyre_base_a` | Tyre Base Grip A | Tyres | Basic | untested |  |
| `tyre_base_b` | Tyre Base Grip B | Tyres | Basic | untested |  |
| `tyre_base_c` | Tyre Base Grip C | Tyres | Basic | untested |  |
| `tyre_base_d` | Tyre Base Grip D | Tyres | Basic | untested |  |
| `tyre_worn_floor` | Worn Tyre Floor | Tyres | Basic | untested |  |
**Count:** 96 fields (registry: `PHYSICS_FIELDS` + `TYRE_FIELDS`).

## Magic data (24 per-track tables)

The 24 magic tables are edited per slot in the Magic Data tab and are not part
of the physics registry. T6 is confirmed `dead` (the game ignores it); the rest
are `untested` in the same sense as above.

## Power curve

The 36 power-curve points are edited as a block in the Power Curve tab and are
verified as a unit rather than per point: `untested`.
