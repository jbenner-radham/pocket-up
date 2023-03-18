use crate::config::{PocketCore, APP_ID};
use adw::prelude::*;
use gtk::{self, gio};

pub fn build_core_row(core: &PocketCore) -> adw::ActionRow {
    let margin = 8;
    let row_horizontal_margin = 28;
    let row = adw::ActionRow::builder()
        .activatable(true)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(row_horizontal_margin)
        .margin_end(row_horizontal_margin)
        .width_request(450)
        .title(core.name)
        .subtitle(core.subtitle_markup())
        .build();
    let switch = gtk::Switch::builder()
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .sensitive(false) // Disable initially.
        .state(false)
        .build();
    let settings = gio::Settings::new(APP_ID);

    settings
        .bind("is-form-enabled", &switch, "sensitive")
        .flags(gio::SettingsBindFlags::DEFAULT)
        .build();
    settings
        .bind(&core.settings_name(), &switch, "state")
        .flags(gio::SettingsBindFlags::DEFAULT)
        .build();

    row.add_prefix(&switch);
    row.set_activatable_widget(Some(&switch));

    row
}
