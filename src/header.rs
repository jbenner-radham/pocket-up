use crate::config::APP_NAME;
use gtk::{self, gio, traits::WidgetExt};

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

pub fn build_header_bar() -> gtk::HeaderBar {
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

    let header_bar = gtk::HeaderBar::new();
    let menu_model = gio::MenuModel::from(menu);
    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .tooltip_text("Main Menu") // https://developer.gnome.org/hig/patterns/controls/menus.html#primary-menus
        .menu_model(&menu_model)
        .build();

    header_bar.pack_end(&menu_button);

    header_bar
}
