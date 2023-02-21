use crate::config::{APP_ID, APP_NAME};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{self, gio};

pub fn build_about_dialog() -> gtk::AboutDialog {
    gtk::AboutDialog::builder()
        .program_name(APP_NAME)
        .title(&format!("About {APP_NAME}"))
        .authors(vec!["James Benner https://www.jamesbenner.com/".to_string()])
        .license_type(gtk::License::MitX11)
        .build()
}

pub fn build_add_github_access_token_modal(window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("Add GitHub Access Token")
        .width_request(375)
        .transient_for(window)
        .modal(true)
        .build();
    let margin = 8;
    let child = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let settings = gio::Settings::new(APP_ID);
    let github_access_token = settings.get::<String>("github-access-token");
    let entry = gtk::Entry::builder().text(&github_access_token).build();
    let button_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .spacing(margin)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let cancel_button = gtk::Button::with_mnemonic("_Cancel");
    let ok_button = gtk::Button::with_mnemonic("_OK");

    cancel_button.connect_clicked(clone!(@weak modal => move |_| modal.close()));

    ok_button.connect_clicked(clone!(@weak entry, @weak modal => move |_| {
        let settings = gio::Settings::new(APP_ID);

        settings
            .set_string("github-access-token", &entry.text().trim())
            .expect("Should have been able to set github-access-token setting.");
        modal.close();
    }));

    button_row.append(&cancel_button);
    button_row.append(&ok_button);

    child.append(&entry);
    child.append(&button_row);

    modal.set_child(Some(&child));

    modal
}

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

pub fn build_parent() -> gtk::Box {
    let margin = 12;

    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .build()
}
