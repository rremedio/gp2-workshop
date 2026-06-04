//! Physics tab (implemented in Task 14).

use crate::app::App;
use eframe::egui;
use gp2ws_core::field::SubTab;

pub fn ui(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Physics");
    ui.horizontal(|ui| {
        for (st, label) in [
            (SubTab::Engine, "Engine"),
            (SubTab::PowerCurve, "Power Curve"),
            (SubTab::Aero, "Aero"),
            (SubTab::Brakes, "Brakes"),
            (SubTab::MassGrip, "Mass/Grip"),
            (SubTab::Tyres, "Tyres"),
            (SubTab::Slipstream, "Slipstream"),
        ] {
            ui.selectable_value(&mut app.subtab, st, label);
        }
    });
    ui.checkbox(&mut app.show_advanced, "Show advanced");
    if !app.is_calibrated() {
        ui.label("Export disabled until the EXE is calibrated.");
    }
    ui.label("(physics editor)");
}
