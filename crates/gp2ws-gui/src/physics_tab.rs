//! Physics tab: per-subtab field grids (Basic + Advanced), the editable power
//! curve with a line plot, and EXE / TOML import-export.
//!
//! All decoding/encoding lives in `gp2ws-core`; here we only parse the text
//! buffers into decoded `i64` values and hand a `PhysicsDoc` to the session.

use crate::app::App;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use gp2ws_core::display::{fixed_float, format_hint, parse_edit_string};
use gp2ws_core::field::{FieldDesc, SubTab, Tier};
use gp2ws_core::physics_fields::PHYSICS_FIELDS;
use gp2ws_core::physics_io::PhysicsDoc;
use gp2ws_core::tyre_fields::TYRE_FIELDS;

const SUBTABS: [(SubTab, &str); 7] = [
    (SubTab::Engine, "Engine"),
    (SubTab::PowerCurve, "Power Curve"),
    (SubTab::Aero, "Aero"),
    (SubTab::Brakes, "Brakes"),
    (SubTab::MassGrip, "Mass/Grip"),
    (SubTab::Tyres, "Tyres"),
    (SubTab::Slipstream, "Slipstream"),
];

/// All registry fields (physics + tyre) belonging to `subtab`.
pub fn fields_for(subtab: SubTab) -> Vec<&'static FieldDesc> {
    PHYSICS_FIELDS
        .iter()
        .chain(TYRE_FIELDS.iter())
        .filter(|f| f.subtab == subtab)
        .collect()
}

/// Collect the current text buffers + curve buffer into a fresh `PhysicsDoc`,
/// based on `base` (which supplies meta and any fields the UI does not edit).
/// Returns the doc plus a list of parse warnings (field id, bad text).
pub fn doc_from_buffers(
    base: &PhysicsDoc,
    field_buf: &std::collections::HashMap<String, String>,
    curve_buf: &[String],
) -> (PhysicsDoc, Vec<String>) {
    let mut doc = base.clone();
    let mut warnings = Vec::new();

    for f in PHYSICS_FIELDS.iter().chain(TYRE_FIELDS.iter()) {
        if let Some(text) = field_buf.get(f.id) {
            match parse_edit_string(text, f.encoding, f.width, f.signed) {
                Some(v) if f.validate(v) => {
                    doc.fields.insert(f.id.to_string(), v);
                }
                Some(v) => {
                    let (min, max) = f.bounds();
                    warnings.push(format!(
                        "{}: value {} out of range ({}..={})",
                        f.id, v, min, max
                    ));
                }
                None => warnings.push(format!("{}: invalid value {:?}", f.id, text)),
            }
        }
    }

    let mut curve = Vec::with_capacity(curve_buf.len());
    for (i, text) in curve_buf.iter().enumerate() {
        match text.trim().parse::<i64>() {
            Ok(v) => curve.push(v),
            Err(_) => warnings.push(format!("power curve [{i}]: invalid value {text:?}")),
        }
    }
    if curve.len() == curve_buf.len() {
        doc.power_curve = curve;
    }

    (doc, warnings)
}

pub fn ui(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Physics");

    if app.physics.is_none() {
        ui.label("Import from the EXE (or load a TOML) to begin editing.");
        physics_buttons(app, ui);
        return;
    }

    // --- Sub-tab selector ---
    ui.horizontal(|ui| {
        for (st, label) in SUBTABS {
            ui.selectable_value(&mut app.subtab, st, label);
        }
    });
    ui.separator();

    let avail = ui.available_height();
    egui::ScrollArea::vertical()
        .id_salt("physics_scroll")
        .max_height(avail - 40.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if app.subtab == SubTab::PowerCurve {
                power_curve_ui(app, ui);
            } else {
                subtab_fields_ui(app, ui);
            }
        });

    ui.separator();
    physics_buttons(app, ui);
}

fn subtab_fields_ui(app: &mut App, ui: &mut egui::Ui) {
    let fields = fields_for(app.subtab);
    let basic: Vec<_> = fields.iter().filter(|f| f.tier == Tier::Basic).collect();
    let advanced: Vec<_> = fields.iter().filter(|f| f.tier == Tier::Advanced).collect();

    if app.subtab == SubTab::Tyres {
        ui.label(
            "Wear curve and track-abrasion tables are not editable in this version (read-only).",
        );
    }

    egui::Grid::new("physics_basic_grid")
        .num_columns(3)
        .spacing([12.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            for f in &basic {
                field_row(app, ui, f);
            }
        });

    if !advanced.is_empty() {
        egui::CollapsingHeader::new("Advanced")
            .default_open(app.show_advanced)
            .show(ui, |ui| {
                egui::Grid::new("physics_advanced_grid")
                    .num_columns(3)
                    .spacing([12.0, 6.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for f in &advanced {
                            field_row(app, ui, f);
                        }
                    });
            });
    }
}

fn field_row(app: &mut App, ui: &mut egui::Ui, f: &FieldDesc) {
    use gp2ws_core::display::value_to_edit_string;

    ui.label(f.label).on_hover_text(f.help);

    let buf = app
        .physics_buf
        .entry(f.id.to_string())
        .or_insert_with(String::new);
    // add_sized forces the width: inside the ScrollArea this box is a middle
    // grid column, where desired_width alone gets squeezed.
    let resp = ui.add_sized(
        [130.0, ui.spacing().interact_size.y],
        egui::TextEdit::singleline(buf),
    );

    // Secondary column: float (for Fixed), validity, or hex hint tooltip.
    match parse_edit_string(&app.physics_buf[f.id], f.encoding, f.width, f.signed) {
        Some(v) => {
            let hint = format_hint(v, f.encoding, f.width);
            if let Some(fl) = fixed_float(v, f.encoding) {
                ui.label(format!("= {fl:.3}\u{00d7}")).on_hover_text(hint);
            } else {
                resp.on_hover_text(hint);
                // The "↺ stock" button lives in the third column.
                ui.label("");
            }
        }
        None => {
            ui.colored_label(egui::Color32::from_rgb(220, 80, 80), "invalid");
        }
    }

    if ui
        .small_button("\u{21BA} stock")
        .on_hover_text(format!("Reset to stock ({})", f.stock))
        .clicked()
    {
        app.physics_buf
            .insert(f.id.to_string(), value_to_edit_string(f.stock, f.encoding, f.width));
    }
    ui.end_row();
}

fn power_curve_ui(app: &mut App, ui: &mut egui::Ui) {
    ui.label("Torque curve (36 entries). Decoded values; the EXE bias is applied for you.");

    // Plot of decoded values currently in the buffers.
    let points: PlotPoints = app
        .curve_buf
        .iter()
        .enumerate()
        .map(|(i, s)| [i as f64, s.trim().parse::<f64>().unwrap_or(0.0)])
        .collect();
    Plot::new("power_curve_plot")
        .height(200.0)
        .allow_zoom(true)
        .allow_drag(true)
        .show(ui, |plot_ui| {
            plot_ui.line(Line::new(points).name("torque"));
        });

    ui.separator();

    egui::Grid::new("power_curve_grid")
        .num_columns(4)
        .spacing([16.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for i in 0..app.curve_buf.len() {
                ui.label(format!("[{i}]"));
                ui.add_sized(
                    [80.0, ui.spacing().interact_size.y],
                    egui::TextEdit::singleline(&mut app.curve_buf[i]),
                );
                if app.curve_buf[i].trim().parse::<i64>().is_err() {
                    ui.colored_label(egui::Color32::from_rgb(220, 80, 80), "invalid");
                } else {
                    ui.label("");
                }
                // Two entries per visual row.
                if i % 2 == 1 {
                    ui.end_row();
                }
            }
        });
}

fn physics_buttons(app: &mut App, ui: &mut egui::Ui) {
    let calibrated = app.is_calibrated();
    ui.horizontal(|ui| {
        if ui
            .add_enabled(calibrated, egui::Button::new("Import EXE"))
            .on_hover_text("Re-read all physics values from the EXE, discarding edits.")
            .clicked()
        {
            app.import_physics_into_buffers();
            app.info("Imported physics from EXE");
        }

        if ui
            .add_enabled(calibrated, egui::Button::new("Export EXE"))
            .on_hover_text("Write all physics values into the EXE (makes a .bak backup first).")
            .clicked()
        {
            export_exe(app);
        }

        ui.separator();

        if ui.button("Load TOML").clicked() {
            load_toml(app);
        }
        if ui
            .add_enabled(app.physics.is_some(), egui::Button::new("Save TOML"))
            .clicked()
        {
            save_toml(app);
        }
    });
}

/// Build a `PhysicsDoc` from the current buffers, reporting parse errors.
fn collect_doc(app: &mut App) -> Option<PhysicsDoc> {
    let base = app.physics.clone()?;
    let (doc, warnings) = doc_from_buffers(&base, &app.physics_buf, &app.curve_buf);
    if !warnings.is_empty() {
        app.error(format!("Fix invalid fields: {}", warnings.join("; ")));
        return None;
    }
    Some(doc)
}

fn export_exe(app: &mut App) {
    let Some(doc) = collect_doc(app) else { return };
    let result = (|| {
        let session = app.session.as_mut().ok_or("no session".to_string())?;
        let warnings = session.export_physics(&doc).map_err(|e| e.to_string())?;
        session
            .save_backup_and_write()
            .map_err(|e| e.to_string())?;
        Ok::<Vec<String>, String>(warnings)
    })();
    match result {
        Ok(warnings) => {
            app.backup_exists = true;
            app.physics = Some(doc);
            if warnings.is_empty() {
                app.info("Wrote physics to EXE");
            } else {
                app.info(format!("Wrote physics to EXE ({} warnings)", warnings.len()));
            }
        }
        Err(e) => app.error(format!("Export failed: {e}")),
    }
}

fn load_toml(app: &mut App) {
    let picked = rfd::FileDialog::new()
        .add_filter("physics TOML", &["toml"])
        .set_title("Load physics TOML")
        .pick_file();
    let Some(path) = picked else { return };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            app.error(format!("Read failed: {e}"));
            return;
        }
    };
    match gp2ws_core::physics_io::from_toml(&text) {
        Ok(doc) => {
            app.set_physics_doc(doc);
            app.info(format!("Loaded {}", path.display()));
        }
        Err(e) => app.error(format!("Invalid TOML: {e}")),
    }
}

fn save_toml(app: &mut App) {
    let Some(doc) = collect_doc(app) else { return };
    let picked = rfd::FileDialog::new()
        .add_filter("physics TOML", &["toml"])
        .set_file_name("gp2-physics.toml")
        .set_title("Save physics TOML")
        .save_file();
    let Some(path) = picked else { return };
    let text = gp2ws_core::physics_io::to_toml(&doc);
    match std::fs::write(&path, text) {
        Ok(()) => {
            app.physics = Some(doc);
            app.info(format!("Saved {}", path.display()));
        }
        Err(e) => app.error(format!("Write failed: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gp2ws_core::physics_io::Meta;

    fn base_doc() -> PhysicsDoc {
        let mut fields = std::collections::BTreeMap::new();
        fields.insert("brake_force".to_string(), 0x160000);
        fields.insert("tow_strength".to_string(), 0x40000);
        PhysicsDoc {
            meta: Meta {
                format: "gp2ws-physics".to_string(),
                version: 1,
            },
            fields,
            power_curve: vec![0i64; 36],
        }
    }

    #[test]
    fn fields_for_partitions_by_subtab() {
        let aero = fields_for(SubTab::Aero);
        assert!(aero.iter().all(|f| f.subtab == SubTab::Aero));
        assert!(aero.iter().any(|f| f.id == "df_scale"));
        // Tyres come from the tyre registry.
        let tyres = fields_for(SubTab::Tyres);
        assert!(!tyres.is_empty());
        assert!(tyres.iter().all(|f| f.subtab == SubTab::Tyres));
    }

    #[test]
    fn doc_from_buffers_applies_edits() {
        let base = base_doc();
        let mut buf = std::collections::HashMap::new();
        buf.insert("tow_strength".to_string(), "999".to_string());
        let curve: Vec<String> = (0..36).map(|i| i.to_string()).collect();
        let (doc, warnings) = doc_from_buffers(&base, &buf, &curve);
        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(doc.fields.get("tow_strength"), Some(&999));
        // Untouched field preserved from base.
        assert_eq!(doc.fields.get("brake_force"), Some(&0x160000));
        assert_eq!(doc.power_curve[5], 5);
    }

    #[test]
    fn doc_from_buffers_reports_bad_value() {
        let base = base_doc();
        let mut buf = std::collections::HashMap::new();
        buf.insert("tow_strength".to_string(), "notanumber".to_string());
        let curve: Vec<String> = (0..36).map(|i| i.to_string()).collect();
        let (_doc, warnings) = doc_from_buffers(&base, &buf, &curve);
        assert!(warnings.iter().any(|w| w.contains("tow_strength")));
    }

    #[test]
    fn doc_from_buffers_rejects_out_of_range_value() {
        // tow_strength is a width-4 unsigned Raw field: 0..=4294967295.
        let base = base_doc();
        let mut buf = std::collections::HashMap::new();
        buf.insert("tow_strength".to_string(), "4294967296".to_string());
        let curve: Vec<String> = vec!["0".to_string(); 36];
        let (doc, warnings) = doc_from_buffers(&base, &buf, &curve);
        assert!(
            warnings.iter().any(|w| w.contains("tow_strength") && w.contains("out of range")),
            "{warnings:?}"
        );
        // The out-of-range edit must not overwrite the base value.
        assert_eq!(doc.fields.get("tow_strength"), Some(&0x40000));
    }

    #[test]
    fn doc_from_buffers_hex_field_parses_hex() {
        // min_grip_clamp is now a plain-decimal field, but Raw still accepts a
        // convenience 0x.. prefix; ensure that still parses.
        let base = base_doc();
        let mut buf = std::collections::HashMap::new();
        buf.insert("min_grip_clamp".to_string(), "0x2C00".to_string());
        let curve: Vec<String> = vec!["0".to_string(); 36];
        let (doc, warnings) = doc_from_buffers(&base, &buf, &curve);
        assert!(warnings.is_empty(), "{warnings:?}");
        assert_eq!(doc.fields.get("min_grip_clamp"), Some(&0x2C00));
    }

    #[test]
    fn doc_from_buffers_bad_curve_entry_warns() {
        let base = base_doc();
        let buf = std::collections::HashMap::new();
        let mut curve: Vec<String> = vec!["0".to_string(); 36];
        curve[10] = "oops".to_string();
        let (_doc, warnings) = doc_from_buffers(&base, &buf, &curve);
        assert!(warnings.iter().any(|w| w.contains("power curve [10]")));
    }
}
