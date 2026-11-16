//! Left panel: a placeholder chat. No AI yet; Enter just adds the text to the list.

use eframe::egui;

pub struct Chat {
    messages: Vec<String>,
    input: String,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            messages: vec!["Welcome! The AI chat comes later.".to_owned()],
            input: String::new(),
        }
    }
}

impl Chat {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Chat");
        ui.separator();

        // The input sits at the bottom of the panel; the messages fill the rest.
        egui::Panel::bottom("chat_input")
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                ui.add_space(6.0);
                let input = ui.add(
                    egui::TextEdit::singleline(&mut self.input)
                        .hint_text("Type a message and press Enter")
                        .desired_width(f32::INFINITY),
                );
                if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.send();
                    input.request_focus();
                }
                ui.add_space(6.0);
            });

        egui::CentralPanel::no_frame().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in &self.messages {
                        ui.label(message);
                        ui.add_space(4.0);
                    }
                });
        });
    }

    fn send(&mut self) {
        let text = self.input.trim();
        if !text.is_empty() {
            self.messages.push(text.to_owned());
        }
        self.input.clear();
    }
}
