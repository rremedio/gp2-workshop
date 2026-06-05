#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod magic_tab;
mod physics_tab;

use app::App;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("GP2 Workshop"),
        ..Default::default()
    };
    eframe::run_native(
        "GP2 Workshop",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
