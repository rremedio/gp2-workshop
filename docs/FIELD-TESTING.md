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
| `engine_force_scale` | Engine Force Scale | Engine | Basic | untested |  |
| `engine_braking` | Engine Braking | Engine | Basic | untested |  |
| `engine_brake_pitch` | Engine-Brake Pitch Factor | Engine | Adv | untested |  |
| `idle_rpm` | Idle RPM | Engine | Adv | untested |  |
| `misfire_probability` | Misfire Probability | Engine | Adv | untested |  |
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
| `wall_restitution` | Wall Restitution | Walls | Basic | untested |  |
| `wall_friction` | Wall Friction | Walls | Basic | untested |  |
| `wall_yaw_gain` | Wall Yaw-Kick Gain | Walls | Adv | untested |  |
| `wall_yaw_clamp` | Wall Yaw-Kick Clamp | Walls | Adv | untested |  |
| `engine_kill_threshold` | Engine-Kill Impact | Walls | Basic | untested |  |
| `damage_load_floor_rl` | Damage Load Floor (Rear Left) | Walls | Adv | untested |  |
| `damage_load_floor_rr` | Damage Load Floor (Rear Right) | Walls | Adv | untested |  |
| `damage_load_floor_fl` | Damage Load Floor (Front Left) | Walls | Adv | untested |  |
| `damage_load_floor_fr` | Damage Load Floor (Front Right) | Walls | Adv | untested |  |
| `spring_break_rl` | Spring Break Load (Rear Left) | Walls | Adv | untested |  |
| `spring_break_rr` | Spring Break Load (Rear Right) | Walls | Adv | untested |  |
| `spring_break_fl` | Spring Break Load (Front Left) | Walls | Adv | untested |  |
| `spring_break_fr` | Spring Break Load (Front Right) | Walls | Adv | untested |  |
| `dmg_thr_a_rl` | Damage Threshold A (Rear Left) | Walls | Adv | untested |  |
| `dmg_thr_a_rr` | Damage Threshold A (Rear Right) | Walls | Adv | untested |  |
| `dmg_thr_a_fl` | Damage Threshold A (Front Left) | Walls | Adv | untested |  |
| `dmg_thr_a_fr` | Damage Threshold A (Front Right) | Walls | Adv | untested |  |
| `dmg_thr_b_rl` | Damage Threshold B (Rear Left) | Walls | Adv | untested |  |
| `dmg_thr_b_rr` | Damage Threshold B (Rear Right) | Walls | Adv | untested |  |
| `dmg_thr_b_fl` | Damage Threshold B (Front Left) | Walls | Adv | untested |  |
| `dmg_thr_b_fr` | Damage Threshold B (Front Right) | Walls | Adv | untested |  |
| `dmg_thr_c_rl` | Damage Threshold C (Rear Left) | Walls | Adv | untested |  |
| `dmg_thr_c_rr` | Damage Threshold C (Rear Right) | Walls | Adv | untested |  |
| `dmg_thr_c_fl` | Damage Threshold C (Front Left) | Walls | Adv | untested |  |
| `dmg_thr_c_fr` | Damage Threshold C (Front Right) | Walls | Adv | untested |  |
| `dmg_thr_d_rl` | Damage Threshold D (Rear Left) | Walls | Adv | untested |  |
| `dmg_thr_d_rr` | Damage Threshold D (Rear Right) | Walls | Adv | untested |  |
| `dmg_thr_d_fl` | Damage Threshold D (Front Left) | Walls | Adv | untested |  |
| `dmg_thr_d_fr` | Damage Threshold D (Front Right) | Walls | Adv | untested |  |
| `damage_probability` | Damage Probability | Walls | Adv | untested |  |
| `spring_break_drop` | Broken-Spring Ride Drop | Walls | Adv | untested |  |
| `surf_grip_track` | Grip: Track | Surfaces | Adv | untested |  |
| `surf_grip_kerb_low` | Grip: Low Kerb | Surfaces | Adv | untested |  |
| `surf_grip_kerb_high` | Grip: High Kerb | Surfaces | Adv | untested |  |
| `surf_grip_grass` | Grip: Grass | Surfaces | Basic | untested |  |
| `surf_grip_gravel` | Grip: Gravel | Surfaces | Basic | untested |  |
| `surf_traction_track` | Traction: Track | Surfaces | Adv | untested |  |
| `surf_traction_kerb_low` | Traction: Low Kerb | Surfaces | Basic | untested |  |
| `surf_traction_kerb_high` | Traction: High Kerb | Surfaces | Basic | untested |  |
| `surf_traction_grass` | Traction: Grass | Surfaces | Adv | untested |  |
| `surf_traction_gravel` | Traction: Gravel | Surfaces | Adv | untested |  |
| `surf_rough_track` | Roughness: Track | Surfaces | Adv | untested |  |
| `surf_rough_kerb_low` | Roughness: Low Kerb | Surfaces | Adv | untested |  |
| `surf_rough_kerb_high` | Roughness: High Kerb | Surfaces | Adv | untested |  |
| `surf_rough_grass` | Roughness: Grass | Surfaces | Adv | untested |  |
| `surf_rough_gravel` | Roughness: Gravel | Surfaces | Adv | untested |  |
| `bump_track_scale` | Bump Amplitude: Track | Surfaces | Adv | untested |  |
| `bump_grass` | Bump Amplitude: Grass | Surfaces | Adv | untested |  |
| `bump_gravel` | Bump Amplitude: Gravel | Surfaces | Adv | untested |  |
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
| `fuel_burn_base` | Fuel Burn Base | MassGrip | Basic | untested |  |
| `fuel_weight_div` | Fuel Weight Divisor | MassGrip | Adv | untested |  |
| `fuel_weight_mult` | Fuel Weight Multiplier | MassGrip | Adv | untested |  |
| `qual_fuel_laps` | Qualifying Fuel Laps | MassGrip | Adv | untested |  |
| `rear_lateral_blend` | Rear Pure-Lateral Blend | Tyres | Basic | untested |  |
| `slip_prescale` | Slip Sensitivity | Tyres | Adv | untested |  |
| `wear_rate_rl` | Tyre Wear Rate (Rear Left) | Tyres | Basic | untested |  |
| `wear_rate_rr` | Tyre Wear Rate (Rear Right) | Tyres | Basic | untested |  |
| `wear_rate_fl` | Tyre Wear Rate (Front Left) | Tyres | Basic | untested |  |
| `wear_rate_fr` | Tyre Wear Rate (Front Right) | Tyres | Basic | untested |  |
| `segment_grip_boost` | Segment Grip Boost | Tyres | Adv | untested |  |
| `tyre_spring` | Tyre Spring Rate | Suspension | Basic | untested |  |
| `tyre_spring_init_rl` | Tyre Spring Init (Rear Left) | Suspension | Adv | untested |  |
| `tyre_spring_init_rr` | Tyre Spring Init (Rear Right) | Suspension | Adv | untested |  |
| `tyre_spring_init_fl` | Tyre Spring Init (Front Left) | Suspension | Adv | untested |  |
| `tyre_spring_init_fr` | Tyre Spring Init (Front Right) | Suspension | Adv | untested |  |
| `tyre_damping_rl` | Tyre Damping (Rear Left) | Suspension | Adv | untested |  |
| `tyre_damping_rr` | Tyre Damping (Rear Right) | Suspension | Adv | untested |  |
| `tyre_damping_fl` | Tyre Damping (Front Left) | Suspension | Adv | untested |  |
| `tyre_damping_fr` | Tyre Damping (Front Right) | Suspension | Adv | untested |  |
| `spring_factor` | Spring Rate Scale | Suspension | Basic | untested |  |
| `arb_factor` | Anti-Roll Bar Scale | Suspension | Basic | untested |  |
| `free_length_rl` | Suspension Travel (Rear Left) | Suspension | Adv | untested |  |
| `free_length_rr` | Suspension Travel (Rear Right) | Suspension | Adv | untested |  |
| `free_length_fl` | Suspension Travel (Front Left) | Suspension | Adv | untested |  |
| `free_length_fr` | Suspension Travel (Front Right) | Suspension | Adv | untested |  |
| `bumpstop_rate_rl` | Bump-Stop Rate (Rear Left) | Suspension | Adv | untested |  |
| `bumpstop_rate_rr` | Bump-Stop Rate (Rear Right) | Suspension | Adv | untested |  |
| `bumpstop_rate_fl` | Bump-Stop Rate (Front Left) | Suspension | Adv | untested |  |
| `bumpstop_rate_fr` | Bump-Stop Rate (Front Right) | Suspension | Adv | untested |  |
| `packer_cap_rl` | Packer Cap (Rear Left) | Suspension | Adv | untested |  |
| `packer_cap_rr` | Packer Cap (Rear Right) | Suspension | Adv | untested |  |
| `packer_cap_fl` | Packer Cap (Front Left) | Suspension | Adv | untested |  |
| `packer_cap_fr` | Packer Cap (Front Right) | Suspension | Adv | untested |  |
| `droop_stiffness` | Droop Stiffness Add | Suspension | Adv | untested |  |
| `bump_rebound_ratio` | Bump/Rebound Ratio | Suspension | Adv | untested |  |
| `damper_knee_pos` | Damper Knee (+) | Suspension | Adv | untested |  |
| `damper_knee_neg` | Damper Knee (-) | Suspension | Adv | untested |  |
| `bottoming_stiffness` | Bottoming Stiffness | Suspension | Adv | untested |  |
| `plank_wear_rate` | Plank Wear Rate | Suspension | Adv | untested |  |
| `heave_knee` | Soft-Limit Knee (Heave) | Suspension | Adv | untested |  |
| `pitch_knee` | Soft-Limit Knee (Pitch) | Suspension | Adv | untested |  |
| `roll_knee` | Soft-Limit Knee (Roll) | Suspension | Adv | untested |  |
| `heave_gain` | Soft-Limit Gain (Heave) | Suspension | Adv | untested |  |
| `pitch_gain` | Soft-Limit Gain (Pitch) | Suspension | Adv | untested |  |
| `roll_gain` | Soft-Limit Gain (Roll) | Suspension | Adv | untested |  |
| `rake_reference` | Reference Rake | Aero | Basic | untested |  |
| `rake_sens_total` | Rake Sensitivity (Total) | Aero | Adv | untested |  |
| `rake_sens_split` | Rake Sensitivity (Split) | Aero | Adv | untested |  |
| `front_ride_sens` | Front Ride Sensitivity | Aero | Adv | untested |  |
| `front_ride_ref` | Front Ride Reference | Aero | Adv | untested |  |
| `ge_clamp_rear` | GE Ride Clamp Rear | Aero | Adv | untested |  |
| `ge_clamp_front` | GE Ride Clamp Front | Aero | Adv | untested |  |
| `ge_master_scale` | Ground-Effect Master | Aero | Basic | untested |  |
| `tyre_grip_a` | Tyre Wear Sensitivity A | Tyres | Basic | untested |  |
| `tyre_grip_b` | Tyre Wear Sensitivity B | Tyres | Basic | untested |  |
| `tyre_grip_c` | Tyre Wear Sensitivity C | Tyres | Basic | untested |  |
| `tyre_grip_d` | Tyre Wear Sensitivity D | Tyres | Basic | untested |  |
| `tyre_base_a` | Tyre Base Grip A | Tyres | Basic | untested |  |
| `tyre_base_b` | Tyre Base Grip B | Tyres | Basic | untested |  |
| `tyre_base_c` | Tyre Base Grip C | Tyres | Basic | untested |  |
| `tyre_base_d` | Tyre Base Grip D | Tyres | Basic | untested |  |
| `tyre_worn_floor` | Worn Tyre Floor | Tyres | Basic | untested |  |
**Count:** 204 fields (registry: `PHYSICS_FIELDS` + `TYRE_FIELDS`).

## Magic data (24 per-track tables)

The 24 magic tables are edited per slot in the Magic Data tab and are not part
of the physics registry. T6 is confirmed `dead` (the game ignores it); the rest
are `untested` in the same sense as above.

## Power curve

The 36 power-curve points are edited as a block in the Power Curve tab and are
verified as a unit rather than per point: `untested`.
