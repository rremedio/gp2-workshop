//! Magic Data tab: slot selector, 24-field grid, .m2d / EXE I/O.
//!
//! The 24 tables map 1:1 to `gp2ws_core::magic::MAGIC_LAYOUT` (table order
//! 1..24). Labels/help are UI presentation only — the values themselves are
//! read/written through `Session::read_magic_slot` / `write_magic_slot`.

use crate::app::App;
use eframe::egui;
use gp2ws_core::magic::{to_m2d, MAGIC_DEAD_TABLE};

/// (label, help) for each of the 24 magic-data tables, 0-based index = table-1.
pub const MAGIC_LABELS: [(&str, &str); 24] = [
    (
        "T1 Tyre/track grip",
        "Per-track grip level that also drives tyre wear. Higher = more grip but \
         faster wear; this is the wear term that clearly slows the player too.",
    ),
    (
        "T2 Cornering grip (always)",
        "Per-track cornering-grip multiplier applied in both qualifying and the \
         race. Higher = more cornering speed for every car.",
    ),
    (
        "T3 Cornering grip (qual)",
        "Per-track cornering-grip multiplier used only in non-race sessions \
         (qualifying/practice). Higher = more cornering speed there.",
    ),
    (
        "T4 Cornering grip (race)",
        "Per-track cornering-grip multiplier used only in the race. Higher = more \
         cornering speed during the race.",
    ),
    (
        "T5 Out-lap grip bias",
        "A small extra grip bias felt most on the out-lap / early laps (the \
         \"miss corners at the start\" feel). Higher = more early-lap grip.",
    ),
    (
        "T6 Dead data",
        "The game ignores this - it has no effect. Kept only so saved files match \
         the original layout.",
    ),
    (
        "T7 Driver pace (qual)",
        "Per-driver qualifying pace scaler for the AI (the player is always 1.0). \
         Higher = faster AI driver in qualifying.",
    ),
    (
        "T8 Driver pace (race)",
        "Per-driver race pace scaler for the AI (the player is always 1.0). \
         Higher = faster AI driver in the race.",
    ),
    (
        "T9 Lap-clock rate (qual)",
        "Adjusts qualifying lap times without changing car speed - it tweaks how \
         fast the lap clock counts. Higher = slower recorded laps.",
    ),
    (
        "T10 Lap-clock rate (race)",
        "Adjusts race lap times without changing car speed - it tweaks how fast \
         the lap clock counts. Higher = slower recorded laps.",
    ),
    (
        "T11 Difficulty grip (SemiPro)",
        "AI grip at the SemiPro difficulty for this track (Pro is interpolated \
         from it). Higher = faster AI; Ace level is about 16384.",
    ),
    (
        "T12 Difficulty grip (Rookie)",
        "AI grip at the Rookie difficulty for this track (Amateur is interpolated \
         from this and SemiPro). Higher = faster AI at easier levels.",
    ),
    (
        "T13 CC mistake rate",
        "How often AI cars make a mistake in corners on this track. Higher = more \
         AI mistakes; lower = cleaner AI driving.",
    ),
    (
        "T14 Pit geometry",
        "Part of the pit-lane entry/exit positioning for this track. Best left \
         alone unless you really know the pit geometry.",
    ),
    (
        "T15 Pit geometry",
        "Part of the pit-lane entry/exit positioning for this track. Best left \
         alone unless you really know the pit geometry.",
    ),
    (
        "T16 Pit geometry",
        "Part of the pit-lane entry/exit positioning for this track. Best left \
         alone unless you really know the pit geometry.",
    ),
    (
        "T17 Pit geometry",
        "Part of the pit-lane entry/exit positioning for this track. Best left \
         alone unless you really know the pit geometry.",
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
         to dive into the pits. Varies per track (sometimes 0).",
    ),
    (
        "T20 Pit-out distance",
        "How many segments after the pit exit cars use to rejoin the racing line. \
         Affects AI cars leaving the pits.",
    ),
    (
        "T21 Pit-in speed",
        "The speed cars are held to while in the pit-in zone (the pit-lane speed). \
         Higher = faster pit approach.",
    ),
    (
        "T22 Fuel burn (human)",
        "Per-track fuel-burn multiplier for the player (16384 = normal). Higher = \
         you use more fuel per lap on this track.",
    ),
    (
        "T23 Fuel burn (CC)",
        "Per-track fuel-burn multiplier for the AI cars (16384 = normal). Higher = \
         the AI uses more fuel per lap on this track.",
    ),
    (
        "T24 Reference lap time",
        "A reference lap time in milliseconds, used by the race-director event \
         timing - not car performance. Set it to the real track lap time; it does \
         not make cars faster or slower.",
    ),
];

/// Is the given 0-based table index the dead table (`T6`)?
pub fn is_dead_table(idx0: usize) -> bool {
    idx0 + 1 == MAGIC_DEAD_TABLE
}

/// Parse the 24 text buffers into a `[u16; 24]`, returning the 1-based table
/// number and the offending text on the first failure.
pub fn parse_buffers(buf: &[String; 24]) -> Result<[u16; 24], (usize, String)> {
    let mut out = [0u16; 24];
    for (i, s) in buf.iter().enumerate() {
        match s.trim().parse::<u16>() {
            Ok(v) => out[i] = v,
            Err(_) => return Err((i + 1, s.clone())),
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

    ui.separator();

    // --- 24-field grid ---
    egui::ScrollArea::vertical()
        .id_salt("magic_grid_scroll")
        .max_height(ui.available_height() - 40.0)
        .show(ui, |ui| {
            egui::Grid::new("magic_grid")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    for i in 0..24usize {
                        let (label, help) = MAGIC_LABELS[i];
                        let dead = is_dead_table(i);
                        ui.horizontal(|ui| {
                            ui.label(label).on_hover_text(help);
                            if ui
                                .small_button("?")
                                .on_hover_text("Show help for this table")
                                .clicked()
                            {
                                app.help_popup =
                                    Some(crate::app::help_popup_entry(label, help));
                            }
                        });
                        let edit = egui::TextEdit::singleline(&mut app.magic_buf[i])
                            .desired_width(140.0);
                        let resp = ui.add_enabled(!dead, edit);
                        if dead {
                            resp.on_hover_text("(ignored by game)");
                        } else if app.magic_buf[i].trim().parse::<u16>().is_err() {
                            ui.colored_label(
                                egui::Color32::from_rgb(220, 80, 80),
                                "invalid u16",
                            );
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
            .on_hover_text("Write this slot into the EXE (makes a .bak backup first).")
            .clicked()
        {
            export_slot(app);
        }

        ui.separator();

        if ui.button("Load .m2d").clicked() {
            load_m2d(app);
        }
        if ui.button("Save .m2d").clicked() {
            save_m2d(app);
        }
    });
}

fn export_slot(app: &mut App) {
    let vals = match parse_buffers(&app.magic_buf) {
        Ok(v) => v,
        Err((line, text)) => {
            app.error(format!("T{line}: {text:?} is not a valid u16"));
            return;
        }
    };
    let slot = app.magic_slot;
    let result = (|| {
        let session = app.session.as_mut().ok_or("no session")?;
        session
            .write_magic_slot(slot, &vals)
            .map_err(|e| e.to_string())?;
        session
            .save_backup_and_write()
            .map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    })();
    match result {
        Ok(()) => {
            app.backup_exists = true;
            app.info(format!("Wrote slot {} to EXE", slot + 1));
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
        Ok(vals) => {
            for (b, v) in app.magic_buf.iter_mut().zip(vals.iter()) {
                *b = v.to_string();
            }
            app.info(format!("Loaded {}", path.display()));
        }
        Err(e) => app.error(format!("Invalid .m2d: {e}")),
    }
}

fn save_m2d(app: &mut App) {
    let vals = match parse_buffers(&app.magic_buf) {
        Ok(v) => v,
        Err((line, text)) => {
            app.error(format!("T{line}: {text:?} is not a valid u16"));
            return;
        }
    };
    let default_name = format!("md-slot-{}.m2d", app.magic_slot + 1);
    let picked = rfd::FileDialog::new()
        .add_filter("magic data", &["m2d"])
        .set_file_name(&default_name)
        .set_title("Save .m2d")
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

    #[test]
    fn t6_is_dead() {
        assert!(is_dead_table(5)); // 0-based 5 == T6
        assert!(!is_dead_table(0));
        assert!(!is_dead_table(23));
    }

    #[test]
    fn labels_cover_all_24() {
        assert_eq!(MAGIC_LABELS.len(), 24);
        assert!(MAGIC_LABELS[5].0.starts_with("T6"));
    }

    #[test]
    fn parse_buffers_ok() {
        let buf: [String; 24] = core::array::from_fn(|i| (i * 10).to_string());
        let vals = parse_buffers(&buf).unwrap();
        assert_eq!(vals[3], 30);
    }

    #[test]
    fn parse_buffers_reports_bad_line() {
        let mut buf: [String; 24] = core::array::from_fn(|_| "1".to_string());
        buf[7] = "nope".to_string();
        let err = parse_buffers(&buf).unwrap_err();
        assert_eq!(err.0, 8); // 1-based
        assert_eq!(err.1, "nope");
    }

    #[test]
    fn parse_buffers_trims_whitespace() {
        let mut buf: [String; 24] = core::array::from_fn(|_| "0".to_string());
        buf[0] = "  42  ".to_string();
        assert_eq!(parse_buffers(&buf).unwrap()[0], 42);
    }
}
