//! Right panel: a fixed, fake list of files. Clicking one selects it.

use eframe::egui;

const FAKE_FILES: &[&str] = &[
    "Cargo.toml",
    "README.md",
    "src/main.rs",
    "src/lib.rs",
    "src/app.rs",
    "src/theme.rs",
];

#[derive(Default)]
pub struct Files {
    /// Index into `FAKE_FILES` of the selected file, if any.
    selected: Option<usize>,
}

impl Files {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Files");
        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for (index, name) in FAKE_FILES.iter().enumerate() {
                    let is_selected = self.selected == Some(index);
                    if ui.selectable_label(is_selected, *name).clicked() {
                        self.selected = Some(index);
                    }
                }
            });
    }

    pub fn selected_name(&self) -> Option<&'static str> {
        self.selected.map(|index| FAKE_FILES[index])
    }
}
