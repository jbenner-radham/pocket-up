use crate::config::APP_NAME;
use adw::prelude::*;
use gtk::{self, gio};

pub fn build_header() -> gtk::Label {
    let margin = 8;
    let header = gtk::Label::builder()
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();

    // https://docs.gtk.org/Pango/pango_markup.html
    header.set_markup(r#"<big><b>openFPGA Cores</b></big>"#);
    header.add_css_class("label--header");

    header
}

pub fn build_header_bar() -> adw::HeaderBar {
    let menu = gio::Menu::new();
    let settings_section = gio::Menu::new();
    let info_section = gio::Menu::new();

    settings_section.append(
        Some("Set GitHub Access Token"),
        Some("win.set-github-access-token"),
    );

    info_section.append(Some("Help"), Some("win.help"));

    // https://developer.gnome.org/hig/patterns/controls/menus.html#standard-primary-menu-items
    info_section.append(Some(&format!("_About {APP_NAME}")), Some("win.about"));

    menu.append_section(None, &settings_section);
    menu.append_section(None, &info_section);

    let header_bar = adw::HeaderBar::new();
    let menu_model = gio::MenuModel::from(menu);
    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Main Menu") // https://developer.gnome.org/hig/patterns/controls/menus.html#primary-menus
        .menu_model(&menu_model)
        .has_frame(true) // TODO: This doesn't appear to have the desired effect. Look into this!
        .primary(true) // Adds F10 keybinding: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.MenuButton.html#primary
        .build();

    header_bar.pack_end(&menu_button);

    header_bar
}
