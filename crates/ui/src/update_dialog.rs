//! The update dialog. It only draws what it is given; it knows nothing about
//! GitHub or downloads, so Phase 6 can plug in real data without redrawing it.

use eframe::egui;

/// What the dialog shows.
pub struct UpdateOffer {
    pub app_name: String,
    pub current_version: String,
    pub new_version: String,
    /// "What's new" link: the release page on GitHub.
    pub release_url: String,
}

impl UpdateOffer {
    /// Placeholder data until the real update check exists.
    pub fn fake() -> Self {
        Self {
            app_name: ide_brand::DISPLAY_NAME.to_owned(),
            current_version: ide_brand::VERSION.to_owned(),
            new_version: "0.0.2".to_owned(),
            release_url: format!("https://github.com/{}/releases", ide_brand::GITHUB_REPO),
        }
    }
}

/// What the person chose.
pub enum UpdateChoice {
    Later,
    UpdateAndRestart,
}

/// Draws the dialog. Returns `Some` on the frame a choice is made.
/// Escape or a click outside the dialog counts as "Later".
pub fn show(ctx: &egui::Context, offer: &UpdateOffer) -> Option<UpdateChoice> {
    let mut choice = None;

    let modal = egui::Modal::new(egui::Id::unique("update_dialog")).show(ctx, |ui| {
        ui.set_width(320.0);
        ui.heading(format!("Update {}", offer.app_name));
        ui.add_space(8.0);
        ui.label(egui::RichText::new(format!("{} → {}", offer.current_version, offer.new_version)).size(18.0));
        ui.hyperlink_to("What's new", &offer.release_url);
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            if ui.button("Update and restart").clicked() {
                choice = Some(UpdateChoice::UpdateAndRestart);
            }
            if ui.button("Later").clicked() {
                choice = Some(UpdateChoice::Later);
            }
        });
    });

    if choice.is_none() && modal.should_close() {
        choice = Some(UpdateChoice::Later);
    }
    choice
}
