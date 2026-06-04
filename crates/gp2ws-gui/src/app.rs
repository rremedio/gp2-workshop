//! The eframe application: holds the open [`Session`] and all transient UI
//! state. All offset/encoding logic lives in `gp2ws-core`; this layer only
//! shuttles strings to/from the core's typed API.

use std::path::PathBuf;

use eframe::egui;
use gp2ws_core::field::SubTab;
use gp2ws_core::physics_io::PhysicsDoc;
use gp2ws_core::Session;

/// The two top-level tabs.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    MagicData,
    Physics,
}

/// A transient status line shown under the top bar (errors / confirmations).
pub struct Status {
    pub text: String,
    pub is_error: bool,
}

pub struct App {
    /// The currently-open EXE session, if any.
    pub session: Option<Session>,

    /// Whether a `.bak` already exists for the open file (drives the indicator).
    pub backup_exists: bool,

    /// Active top tab.
    pub tab: Tab,

    // ---- Magic Data tab state ----
    /// Selected slot, 0-based (UI shows 1..16).
    pub magic_slot: usize,
    /// Editable text buffers for the 24 magic words of the current slot.
    pub magic_buf: [String; 24],

    // ---- Physics tab state ----
    /// Active physics sub-tab.
    pub subtab: SubTab,
    /// Whether the Advanced collapsing section starts open.
    pub show_advanced: bool,
    /// The working physics document (loaded via Import / Load TOML).
    pub physics: Option<PhysicsDoc>,
    /// Editable text buffers for the physics scalar fields, keyed by field id.
    pub physics_buf: std::collections::HashMap<String, String>,
    /// Editable text buffers for the 36 power-curve entries.
    pub curve_buf: Vec<String>,

    // ---- Status ----
    pub status: Option<Status>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            session: None,
            backup_exists: false,
            tab: Tab::MagicData,
            magic_slot: 0,
            magic_buf: core::array::from_fn(|_| String::new()),
            subtab: SubTab::Engine,
            show_advanced: false,
            physics: None,
            physics_buf: std::collections::HashMap::new(),
            curve_buf: Vec::new(),
            status: None,
        }
    }
}

impl App {
    /// Record an informational status line.
    pub fn info(&mut self, text: impl Into<String>) {
        self.status = Some(Status {
            text: text.into(),
            is_error: false,
        });
    }

    /// Record an error status line.
    pub fn error(&mut self, text: impl Into<String>) {
        self.status = Some(Status {
            text: text.into(),
            is_error: true,
        });
    }

    /// Is the open session calibrated? (Export actions are gated on this.)
    pub fn is_calibrated(&self) -> bool {
        self.session.as_ref().and_then(|s| s.delta()).is_some()
    }

    /// Open an EXE via a file dialog and load it into a [`Session`].
    fn open_dialog(&mut self) {
        let picked = rfd::FileDialog::new()
            .add_filter("GP2 executable", &["EXE", "exe"])
            .set_title("Open GP2.EXE")
            .pick_file();
        if let Some(path) = picked {
            self.open_path(path);
        }
    }

    /// Load a specific path (extracted so it is independently testable).
    pub fn open_path(&mut self, path: PathBuf) {
        match Session::open(&path) {
            Ok(session) => {
                self.backup_exists = path.with_extension("bak").exists();
                let calibrated = session.delta().is_some();
                self.session = Some(session);
                // Refresh dependent buffers for whatever data is available.
                if calibrated {
                    self.reload_magic_slot();
                    self.import_physics_into_buffers();
                    self.info(format!("Opened {}", path.display()));
                } else {
                    self.error(format!(
                        "Opened {} but it could not be calibrated — editing disabled",
                        path.display()
                    ));
                }
            }
            Err(e) => self.error(format!("Failed to open: {e}")),
        }
    }

    /// Re-read the current magic slot from the session into the text buffers.
    pub fn reload_magic_slot(&mut self) {
        if let Some(session) = &self.session {
            if let Ok(vals) = session.read_magic_slot(self.magic_slot) {
                for (b, v) in self.magic_buf.iter_mut().zip(vals.iter()) {
                    *b = v.to_string();
                }
            }
        }
    }

    /// Import physics from the EXE into `self.physics` and rebuild the buffers.
    pub fn import_physics_into_buffers(&mut self) {
        if let Some(session) = &self.session {
            if let Ok(doc) = session.import_physics() {
                self.set_physics_doc(doc);
            }
        }
    }

    /// Install a physics doc and rebuild all physics text buffers from it.
    pub fn set_physics_doc(&mut self, doc: PhysicsDoc) {
        use gp2ws_core::display::value_to_edit_string;
        use gp2ws_core::physics_fields::PHYSICS_FIELDS;
        use gp2ws_core::tyre_fields::TYRE_FIELDS;

        self.physics_buf.clear();
        for f in PHYSICS_FIELDS.iter().chain(TYRE_FIELDS.iter()) {
            if let Some(&v) = doc.fields.get(f.id) {
                self.physics_buf
                    .insert(f.id.to_string(), value_to_edit_string(v, f.encoding));
            }
        }
        self.curve_buf = doc.power_curve.iter().map(|v| v.to_string()).collect();
        self.physics = Some(doc);
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Open GP2.EXE").clicked() {
                self.open_dialog();
            }

            ui.separator();

            // Calibration badge.
            match self.session.as_ref().and_then(|s| s.delta()) {
                Some(delta) => {
                    ui.colored_label(
                        egui::Color32::from_rgb(60, 180, 75),
                        format!("\u{2713} calibrated (\u{0394}=0x{:X})", delta),
                    );
                }
                None if self.session.is_some() => {
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 160, 40),
                        "\u{26A0} uncalibrated",
                    );
                }
                None => {
                    ui.label("no file open");
                }
            }

            if self.session.is_some() {
                ui.separator();
                if self.backup_exists {
                    ui.label("backup: .bak present");
                } else {
                    ui.label("backup: none yet");
                }
            }
        });
    }

    fn tab_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.tab, Tab::MagicData, "Magic Data");
            ui.selectable_value(&mut self.tab, Tab::Physics, "Physics");
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            self.top_bar(ui);
            if let Some(status) = &self.status {
                let color = if status.is_error {
                    egui::Color32::from_rgb(220, 80, 80)
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(color, &status.text);
            }
            self.tab_bar(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.session.is_none() {
                ui.heading("GP2 Workshop");
                ui.label("Open a GP2.EXE to begin. A .bak backup is made before any write.");
                return;
            }
            match self.tab {
                Tab::MagicData => crate::magic_tab::ui(self, ui),
                Tab::Physics => crate::physics_tab::ui(self, ui),
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_exe_path() -> PathBuf {
        use gp2ws_core::calibration::ANCHORS;
        use gp2ws_core::exe::ExeImage;
        let mut img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        for a in ANCHORS {
            img.write(a.target.base_offset(), a.width, a.stock);
        }
        let dir = std::env::temp_dir().join(format!(
            "gp2ws-gui-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("GP2.EXE");
        img.save(&path).unwrap();
        path
    }

    #[test]
    fn open_path_populates_buffers_and_calibration() {
        let path = synthetic_exe_path();
        let mut app = App::default();
        app.open_path(path.clone());

        assert!(app.is_calibrated());
        // Magic buffers are filled (24 entries, all parseable).
        assert!(app.magic_buf.iter().all(|b| b.parse::<u16>().is_ok()));
        // Physics doc imported and curve buffer has 36 entries.
        assert!(app.physics.is_some());
        assert_eq!(app.curve_buf.len(), 36);
        // Every known scalar field has a buffer.
        assert!(app.physics_buf.contains_key("brake_force"));

        let _ = std::fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn uncalibrated_disables_editing() {
        use gp2ws_core::exe::ExeImage;
        let img = ExeImage::from_bytes(vec![0u8; 1_400_000]);
        let dir = std::env::temp_dir().join(format!("gp2ws-gui-bad-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("bad.exe");
        img.save(&path).unwrap();

        let mut app = App::default();
        app.open_path(path);
        assert!(!app.is_calibrated());
        assert!(app.session.is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
