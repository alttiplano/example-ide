//! Middle panel: egui's built-in text box in code style. Our own editor widget comes later.

use eframe::egui;

const SAMPLE_CODE: &str = r#"fn main() {
    let name = "Example IDE";
    println!("Hello from {name}!");
}
"#;

pub struct Editor {
    code: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            code: SAMPLE_CODE.to_owned(),
        }
    }
}

impl Editor {
    pub fn show(&mut self, ui: &mut egui::Ui, file_name: Option<&str>) {
        ui.label(egui::RichText::new(file_name.unwrap_or("untitled")).strong());
        ui.separator();

        egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.code).code_editor(),
            );
        });
    }
}
