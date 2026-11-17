use eframe::egui;

/// Window icon, rendered by `scripts/app-icon.sh`. Shown on X11 and Windows; on Wayland the
/// desktop takes the icon from the `.desktop` file that matches the app id instead.
const ICON_PNG: &[u8] = include_bytes!("../../../assets/app-icon/app-icon.png");

fn main() -> eframe::Result {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title(ide_brand::DISPLAY_NAME)
        // Must match the .desktop file name (packaging/linux/<app id>.desktop), or the dock shows
        // this raw id and a generic icon.
        .with_app_id(ide_brand::MACHINE_NAME)
        .with_inner_size([1200.0, 760.0])
        .with_min_inner_size([640.0, 400.0]);
    if let Ok(icon) = eframe::icon_data::from_png_bytes(ICON_PNG) {
        viewport = viewport.with_icon(icon);
    }

    eframe::run_native(
        ide_brand::DISPLAY_NAME,
        eframe::NativeOptions {
            viewport,
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(ide_ui::App::default()))),
    )
}
