use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(ide_brand::DISPLAY_NAME)
            .with_app_id(ide_brand::MACHINE_NAME)
            .with_inner_size([1200.0, 760.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        ide_brand::DISPLAY_NAME,
        options,
        Box::new(|_cc| Ok(Box::new(ide_ui::App::default()))),
    )
}
