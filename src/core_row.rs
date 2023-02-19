use crate::config::{PocketCore, APP_ID};
use gtk::prelude::*;
use gtk::{self, gio};

pub fn build_core_row(core: &PocketCore) -> gtk::Box {
    let margin = 8;
    let row_horizontal_margin = 28;
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(row_horizontal_margin)
        .margin_end(row_horizontal_margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Start)
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
    let label = gtk::Label::builder()
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
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
    label.set_markup(&core.description_markup());
    row.append(&switch);
    row.append(&label);

    row
}
