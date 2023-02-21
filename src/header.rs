use crate::config::APP_NAME;
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
    header.set_markup(r#"<span foreground="blue"><big><b>openFPGA Cores</b></big></span>"#);

    header
}

pub fn build_header_bar() -> gtk::HeaderBar {
    let menu = gio::Menu::new();

    menu.append(
        Some("Add GitHub Access Token"),
        Some("win.add-github-access-token"),
    );
    menu.append(Some("Help"), Some("win.help"));

    // https://developer.gnome.org/hig/patterns/controls/menus.html#standard-primary-menu-items
    menu.append(Some(&format!("_About {APP_NAME}")), Some("win.about"));

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
