//! Everything that is drawn on screen lives in this crate.

use eframe::egui;

/// The whole app's state. It lives as long as the window is open.
#[derive(Default)]
pub struct App {}

impl eframe::App for App {
    /// Called every frame: egui redraws the whole window from the state above.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(ide_brand::DISPLAY_NAME);
            ui.label(format!("Version {}", ide_brand::VERSION));
        });
    }
}
