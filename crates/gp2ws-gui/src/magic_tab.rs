//! Magic Data tab: slot selector, 28-field grid, .m2d / EXE I/O.
//!
//! The 28 fields map 1:1 to `gp2ws_core::magic::MAGIC_LAYOUT` (v2 layout:
//! strict EXE address order, dead T6 removed, the community "T14-17" replaced
//! by the six real pit-record fields, plus the three tables discovered in the
//! 2026-07 RE). Labels/help are UI presentation only — the values themselves
//! are read/written through `Session::read_magic_slot` / `write_magic_slot`.
//!
//! `.m2d` files: loading accepts both the legacy 24-line format and the
//! 28-line v2 format (detected by line count). Saving always writes v2.
//! Data loaded from a legacy file exports WITHOUT the pit record and the new
//! AI fields — the legacy layout scrambles the pit record across slots, so
//! writing it per slot would corrupt other tracks.

use crate::app::App;
use eframe::egui;
use gp2ws_core::magic::{field_range, to_m2d, MAGIC_COUNT, NOT_IN_LEGACY};

/// (label, help) for each of the 28 magic-data fields, in v2 layout order.
/// Classic tables keep their community "T" numbers so old references still
/// work; the fields new in v2 have no T number.
pub const MAGIC_LABELS: [(&str, &str); MAGIC_COUNT] = [
    (
        "T1 Tyre/track grip",
        "Per-track grip level that also drives tyre wear. Higher = more grip but \
         faster wear; this is the wear term that clearly slows the player too. Old editor: \"Tire Wear\".",
    ),
    (
        "T2 Cornering grip (always)",
        "Per-track cornering-grip multiplier applied in both qualifying and the \
         race. Higher = more cornering speed for every car. Old editor: \"Slot Grip\".",
    ),
    (
        "T3 Cornering grip (qual)",
        "Per-track cornering-grip multiplier used only in non-race sessions \
         (qualifying/practice). Higher = more cornering speed there. Old editor: \"Qual Grip 1\".",
    ),
    (
        "T4 Cornering grip (race)",
        "Per-track cornering-grip multiplier used only in the race. Higher = more \
         cornering speed during the race. Old editor: \"Race Grip 1\".",
    ),
    (
        "AI consistency floor (new)",
        "How good the AI's WORST corners are at this track. Each AI corner is \
         taken somewhere between a floor (set by the difficulty level) and a \
         ceiling (AI bravery, below). This raises or lowers that floor per \
         track: positive = steadier AI with fewer weak corners; negative = \
         scrappier. The game ships this as 0 everywhere - a dormant knob. AI \
         only. Not stored in old-format .m2d files.",
    ),
    (
        "T5 AI bravery (lap 1 pace & mistakes)",
        "How much faster than its own \"safe\" cornering speed the AI dares to \
         go at this track. The effect shows up mostly at the start of a run: a \
         higher value makes the AI noticeably quicker on the first flying lap \
         and bolder into the opening corners - cars brake later behind a rival \
         and stay side by side longer. From lap 2 onwards the game starts \
         letting the AI make mistakes, and a higher value also makes those \
         mistakes bigger (overcooked corners, lifting to recover), so overall \
         race pace barely changes - the AI just gets braver AND scrappier. For \
         a clean all-race AI speed change use the grip/pace tables instead. \
         AI only. Old editor: \"Unknown 5\"; previously mislabelled \"Out-lap \
         grip bias\" here.",
    ),
    (
        "T7 Driver pace (qual)",
        "Per-driver qualifying pace scaler for the AI (the player is always 1.0). \
         Higher = faster AI driver in qualifying. Old editor: \"Qual Grip 2\".",
    ),
    (
        "T8 Driver pace (race)",
        "Per-driver race pace scaler for the AI (the player is always 1.0). \
         Higher = faster AI driver in the race. Old editor: \"Race Grip 2\".",
    ),
    (
        "T9 Lap-clock rate (qual)",
        "Adjusts qualifying lap times without changing car speed - it tweaks how \
         fast the lap clock counts. Higher = slower recorded laps. Old editor: \"Qual Timing\".",
    ),
    (
        "T10 Lap-clock rate (race)",
        "Adjusts race lap times without changing car speed - it tweaks how fast \
         the lap clock counts. Higher = slower recorded laps. Old editor: \"Race Timing\".",
    ),
    (
        "T11 Difficulty grip (SemiPro)",
        "AI grip at the SemiPro difficulty for this track (Pro is interpolated \
         from it). Higher = faster AI; Ace level is about 16384. Old editor: \
         \"Semi-Pro Grip\". Old-format .m2d files stored this scrambled across \
         slots, so it is skipped when loading/exporting legacy files.",
    ),
    (
        "T12 Difficulty grip (Rookie)",
        "AI grip at the Rookie difficulty for this track (Amateur is interpolated \
         from this and SemiPro). Higher = faster AI at easier levels. Old-format \
         .m2d files stored this scrambled across slots, so it is skipped when \
         loading/exporting legacy files.",
    ),
    (
        "T13 CC mistake rate",
        "How often AI cars make a mistake in corners on this track. Higher = more \
         AI mistakes; lower = cleaner AI driving. Old editor: \"CC Mistake Rate\".",
    ),
    (
        "AI mistake severity min (new)",
        "When an AI car makes a mistake at this track (lap 2 onwards), it takes \
         a corner too fast. This is the SMALLEST extra speed a mistake carries; \
         raise it and even minor errors become obvious wobbles. Stock 512. AI \
         only. Not stored in old-format .m2d files.",
    ),
    (
        "AI mistake severity max (new)",
        "The LARGEST extra speed an AI mistake can carry into a corner at this \
         track. Raise it for spectacular offs, lower it for gentle wobbles. \
         Stock 2048. AI only. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: entry angle A",
        "Purely visual: fine-tunes how the track and pit-lane 3D graphics \
         overlap where the pit lane splits from the track. These six \"Pit \
         view\" values replace the old T14-T17, which were a scrambled view of \
         the same data - wrong values show as glitched buildings or road near \
         the pits, but they never affect driving. Signed; stock varies per \
         track. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: entry angle B",
        "The partner of entry angle A: past this angle the game flips the draw \
         order of track vs pit-lane graphics at the pit entry. Signed; stock \
         varies per track. Purely visual. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: entry overlap trim",
        "How many track pieces the pit-entry overlap check covers. Stock is 3-8 \
         depending on the track. CAUTION: this one is unsigned and unchecked by \
         the game - a large value can blank the whole screen near the pits. \
         Keep it small. Purely visual. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: exit angle A",
        "Same as entry angle A, for the pit EXIT end of the pit lane. Signed; \
         purely visual. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: exit angle B",
        "Same as entry angle B, for the pit EXIT end of the pit lane. Signed; \
         purely visual. Not stored in old-format .m2d files.",
    ),
    (
        "Pit view: exit overlap trim",
        "Same as the entry overlap trim, for the pit EXIT. Unsigned and \
         unchecked - keep it small (stock 3-8). Purely visual. Not stored in \
         old-format .m2d files.",
    ),
    (
        "T18 Pit-approach zone",
        "Length (in track segments) of the zone before the pit entry where the AI \
         eases off following and overtaking so cars don't pile up at the pit \
         mouth. Stock is 64 on every track.",
    ),
    (
        "T19 Pit-in distance",
        "How many segments before the pit entry cars start leaving the racing line \
         to dive into the pits. Varies per track (sometimes 0). Old editor: \"Pit-in Distance\".",
    ),
    (
        "T20 Pit-out distance",
        "How many segments after the pit exit cars use to rejoin the racing line. \
         Affects AI cars leaving the pits. Old editor: \"Pit-out Distance\".",
    ),
    (
        "T21 Pit-in speed",
        "The speed cars are held to while in the pit-in zone (the pit-lane speed). \
         Higher = faster pit approach. Old editor: \"Pit-in Speed\".",
    ),
    (
        "T22 Fuel burn (human)",
        "Per-track fuel-burn multiplier for the player (16384 = normal). Higher = \
         you use more fuel per lap on this track. Old editor: \"Human Fuel\".",
    ),
    (
        "T23 Fuel burn (CC)",
        "Per-track fuel-burn multiplier for the AI cars (16384 = normal). Higher = \
         the AI uses more fuel per lap on this track. Old editor: \"CC Fuel\".",
    ),
    (
        "T24 Reference lap time",
        "A reference lap time in milliseconds, used by the race-director event \
         timing - not car performance. Set it to the real track lap time; it does \
         not make cars faster or slower.",
    ),
];

/// Parse the 28 text buffers into values, returning the 1-based grid row and
/// the offending text on the first failure. With `legacy` set, the fields a
/// legacy file doesn't carry are allowed to be blank (parsed as 0 — they are
/// skipped on export anyway).
pub fn parse_buffers(
    buf: &[String; MAGIC_COUNT],
    legacy: bool,
) -> Result<[i32; MAGIC_COUNT], (usize, String)> {
    let mut out = [0i32; MAGIC_COUNT];
    for (i, s) in buf.iter().enumerate() {
        let t = s.trim();
        if legacy && NOT_IN_LEGACY.contains(&i) && t.is_empty() {
            continue;
        }
        let (min, max) = field_range(i);
        match t.parse::<i32>() {
            Ok(v) if v >= min && v <= max => out[i] = v,
            _ => return Err((i + 1, s.clone())),
        }
    }
    Ok(out)
}

pub fn ui(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Magic Data");

    // --- Slot selector ---
    let mut slot_changed = false;
    ui.horizontal(|ui| {
        ui.label("Slot:");
        egui::ComboBox::from_id_salt("magic_slot")
            .selected_text(format!("{}", app.magic_slot + 1))
            .show_ui(ui, |ui| {
                for s in 0..16usize {
                    if ui
                        .selectable_value(&mut app.magic_slot, s, format!("{}", s + 1))
                        .clicked()
                    {
                        slot_changed = true;
                    }
                }
            });
    });
    if slot_changed {
        app.reload_magic_slot();
    }

    if app.magic_legacy {
        ui.colored_label(
            egui::Color32::from_rgb(220, 170, 60),
            "Legacy .m2d loaded: the pit-view, difficulty-grip and new AI \
             fields were not (reliably) in the file. Export leaves them \
             untouched in the EXE; Import (read slot) fills them back in.",
        );
    }

    ui.separator();

    // --- field grid ---
    egui::ScrollArea::vertical()
        .id_salt("magic_grid_scroll")
        .max_height(ui.available_height() - 40.0)
        .show(ui, |ui| {
            egui::Grid::new("magic_grid")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    for (i, &(label, help)) in MAGIC_LABELS.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(label).on_hover_text(help);
                            if ui
                                .small_button("?")
                                .on_hover_text("Show help for this field")
                                .clicked()
                            {
                                app.help_popup =
                                    Some(crate::app::help_popup_entry(label, help));
                            }
                        });
                        let skipped = app.magic_legacy && NOT_IN_LEGACY.contains(&i);
                        let edit = egui::TextEdit::singleline(&mut app.magic_buf[i])
                            .desired_width(140.0);
                        let resp = ui.add(edit);
                        if skipped && app.magic_buf[i].trim().is_empty() {
                            resp.on_hover_text("not in the legacy file - skipped on export");
                        } else {
                            let (min, max) = field_range(i);
                            let ok = app.magic_buf[i]
                                .trim()
                                .parse::<i32>()
                                .is_ok_and(|v| v >= min && v <= max);
                            if !ok {
                                ui.colored_label(
                                    egui::Color32::from_rgb(220, 80, 80),
                                    format!("must be {min}\u{2013}{max}"),
                                );
                            }
                        }
                        ui.end_row();
                    }
                });
        });

    ui.separator();

    // --- Action buttons ---
    let calibrated = app.is_calibrated();
    ui.horizontal(|ui| {
        if ui
            .add_enabled(calibrated, egui::Button::new("Import (read slot)"))
            .on_hover_text("Re-read this slot from the EXE, discarding edits.")
            .clicked()
        {
            app.reload_magic_slot();
            app.info(format!("Read slot {} from EXE", app.magic_slot + 1));
        }

        if ui
            .add_enabled(calibrated, egui::Button::new("Export (write slot)"))
            .on_hover_text(
                "Write this slot into the EXE (makes a .bak backup first). \
                 Data loaded from a legacy .m2d skips the pit-view, \
                 difficulty-grip and new AI fields.",
            )
            .clicked()
        {
            export_slot(app);
        }

        ui.separator();

        if ui
            .button("Load .m2d")
            .on_hover_text("Loads both the new 28-line format and legacy 24-line files.")
            .clicked()
        {
            load_m2d(app);
        }
        if ui
            .button("Save .m2d")
            .on_hover_text("Always saves the new 28-line format.")
            .clicked()
        {
            save_m2d(app);
        }
    });
}

fn export_slot(app: &mut App) {
    let legacy = app.magic_legacy;
    let vals = match parse_buffers(&app.magic_buf, legacy) {
        Ok(v) => v,
        Err((row, text)) => {
            app.error(format!(
                "{}: {text:?} is not a valid value",
                MAGIC_LABELS[row - 1].0
            ));
            return;
        }
    };
    let slot = app.magic_slot;
    let result = (|| {
        let session = app.session.as_mut().ok_or("no session")?;
        session
            .write_magic_slot(slot, &vals, legacy)
            .map_err(|e| e.to_string())?;
        session
            .save_backup_and_write()
            .map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })();
    match result {
        Ok(()) => {
            app.backup_exists = true;
            if legacy {
                app.info(format!(
                    "Wrote slot {} to EXE (pit-view / difficulty-grip / new AI \
                     fields left untouched)",
                    slot + 1
                ));
            } else {
                app.info(format!("Wrote slot {} to EXE", slot + 1));
            }
        }
        Err(e) => app.error(format!("Export failed: {e}")),
    }
}

fn load_m2d(app: &mut App) {
    let picked = rfd::FileDialog::new()
        .add_filter("magic data", &["m2d"])
        .set_title("Load .m2d")
        .pick_file();
    let Some(path) = picked else { return };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            app.error(format!("Read failed: {e}"));
            return;
        }
    };
    match gp2ws_core::magic::parse_m2d(&text) {
        Ok(slot) => {
            for (i, b) in app.magic_buf.iter_mut().enumerate() {
                if slot.legacy && NOT_IN_LEGACY.contains(&i) {
                    b.clear();
                } else {
                    *b = slot.vals[i].to_string();
                }
            }
            app.magic_legacy = slot.legacy;
            if slot.legacy {
                app.info(format!(
                    "Loaded {} (legacy 24-line format - pit-view / \
                     difficulty-grip / new AI fields not carried over)",
                    path.display()
                ));
            } else {
                app.info(format!("Loaded {}", path.display()));
            }
        }
        Err(e) => app.error(format!("Invalid .m2d: {e}")),
    }
}

fn save_m2d(app: &mut App) {
    if app.magic_legacy {
        app.error(
            "This slot was loaded from a legacy file and is missing the pit-view, \
             difficulty-grip and new AI fields. Use Import (read slot) to fill \
             them from the EXE, then save."
                .to_string(),
        );
        return;
    }
    let vals = match parse_buffers(&app.magic_buf, false) {
        Ok(v) => v,
        Err((row, text)) => {
            app.error(format!(
                "{}: {text:?} is not a valid value",
                MAGIC_LABELS[row - 1].0
            ));
            return;
        }
    };
    let default_name = format!("md-slot-{}.m2d", app.magic_slot + 1);
    let picked = rfd::FileDialog::new()
        .add_filter("magic data", &["m2d"])
        .set_file_name(&default_name)
        .set_title("Save .m2d (new 28-line format)")
        .save_file();
    let Some(path) = picked else { return };
    match std::fs::write(&path, to_m2d(&vals)) {
        Ok(()) => app.info(format!("Saved {}", path.display())),
        Err(e) => app.error(format!("Write failed: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gp2ws_core::magic::MAGIC_LAYOUT;

    #[test]
    fn labels_cover_all_fields_and_t6_is_gone() {
        assert_eq!(MAGIC_LABELS.len(), MAGIC_COUNT);
        assert!(MAGIC_LABELS.iter().all(|(l, _)| !l.starts_with("T6")));
        // layout and labels agree on which fields are the new/pit ones
        for &i in &NOT_IN_LEGACY {
            let (l, h) = MAGIC_LABELS[i];
            assert!(
                l.contains("new") || l.starts_with("Pit view") || l.starts_with("T11")
                    || l.starts_with("T12"),
                "label {l:?} at {i}"
            );
            assert!(
                h.contains("Not stored in old-format") || h.contains("skipped when"),
                "help at {i} must mention legacy handling"
            );
        }
    }

    #[test]
    fn labels_signedness_matches_layout() {
        // every signed field's help mentions "Signed" or describes negatives
        for (i, fld) in MAGIC_LAYOUT.iter().enumerate() {
            if fld.signed {
                let (_, h) = MAGIC_LABELS[i];
                assert!(
                    h.contains("Signed") || h.contains("negative"),
                    "field {i} help should mention signedness"
                );
            }
        }
    }

    #[test]
    fn parse_buffers_ok_and_signed() {
        let mut buf: [String; MAGIC_COUNT] = core::array::from_fn(|i| (i * 10).to_string());
        buf[15] = "-2176".to_string(); // pit entry angle A is signed
        let vals = parse_buffers(&buf, false).unwrap();
        assert_eq!(vals[0], 0);
        assert_eq!(vals[15], -2176);
        assert_eq!(vals[27], 270);
    }

    #[test]
    fn parse_buffers_rejects_negative_unsigned() {
        let mut buf: [String; MAGIC_COUNT] = core::array::from_fn(|_| "1".to_string());
        buf[0] = "-1".to_string(); // T1 is unsigned
        let err = parse_buffers(&buf, false).unwrap_err();
        assert_eq!(err.0, 1);
    }

    #[test]
    fn parse_buffers_legacy_allows_blank_skipped_fields() {
        let mut buf: [String; MAGIC_COUNT] = core::array::from_fn(|_| "7".to_string());
        for &i in &NOT_IN_LEGACY {
            buf[i].clear();
        }
        // legacy: blanks in skipped fields are fine
        let vals = parse_buffers(&buf, true).unwrap();
        assert_eq!(vals[0], 7);
        // non-legacy: the first blank is an error
        assert!(parse_buffers(&buf, false).is_err());
    }

    #[test]
    fn parse_buffers_trims_whitespace() {
        let mut buf: [String; MAGIC_COUNT] = core::array::from_fn(|_| "0".to_string());
        buf[0] = "  42  ".to_string();
        assert_eq!(parse_buffers(&buf, false).unwrap()[0], 42);
    }
}
