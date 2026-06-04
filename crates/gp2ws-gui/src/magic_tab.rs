//! Magic Data tab (implemented in Task 13).

use crate::app::App;
use eframe::egui;

pub fn ui(_app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Magic Data");
    ui.label("(magic data editor)");
}
