//! Everything that is drawn on screen lives in this crate.

mod panels;
mod update_dialog;

use eframe::egui;
use panels::{chat::Chat, editor::Editor, files::Files, status_bar};
use update_dialog::{UpdateChoice, UpdateOffer};

/// The whole app's state. It lives as long as the window is open.
#[derive(Default)]
pub struct App {
    chat: Chat,
    editor: Editor,
    files: Files,
    /// `Some` while the update dialog is open.
    update_offer: Option<UpdateOffer>,
}

impl eframe::App for App {
    /// Called every frame: egui redraws the whole window from the state above.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Edge panels first, the central panel last: it takes the space that is left.
        egui::Panel::bottom("status_bar")
            .exact_size(26.0)
            .show(ui, |ui| {
                if status_bar::show(ui) {
                    self.update_offer = Some(UpdateOffer::fake());
                }
            });

        egui::Panel::left("chat")
            .resizable(true)
            .default_size(280.0)
            .min_size(180.0)
            .show(ui, |ui| self.chat.show(ui));

        egui::Panel::right("files")
            .resizable(true)
            .default_size(220.0)
            .min_size(140.0)
            .show(ui, |ui| self.files.show(ui));

        egui::CentralPanel::default().show(ui, |ui| {
            self.editor.show(ui, self.files.selected_name());
        });

        if let Some(offer) = &self.update_offer
            && let Some(choice) = update_dialog::show(ui.ctx(), offer)
        {
            // Phase 6 connects the real update here; for now both buttons just close.
            match choice {
                UpdateChoice::Later | UpdateChoice::UpdateAndRestart => self.update_offer = None,
            }
        }
    }
}
