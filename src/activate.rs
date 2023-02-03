use crate::button_row::build_button_row;
use crate::core_row::build_core_row;
use gtk::{gio, self};
use gtk::prelude::*;

const APP_NAME: &str = "PocketUp";

fn build_parent() -> gtk::Box {
    let margin = 12;

    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build()
}

fn build_header_bar() -> gtk::HeaderBar {
    let menu = gio::Menu::new();

    menu.append(Some(&format!("_About {APP_NAME}")), Some("win.about"));

    let header_bar = gtk::HeaderBar::new();
    let menu_model = gio::MenuModel::from(menu);
    let menu_button = gtk::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu_model)
        .build();

    header_bar.pack_end(&menu_button);

    header_bar
}

// When the application is launched...
pub fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let cores = vec![
        "Core One by Person A",
        "Core Two by Person B",
        "Core Three by Person C",
    ];
    let parent = build_parent();
    let button_row = build_button_row(&window);
    let action_about = gio::SimpleAction::new("about", None);

    action_about.connect_activate(|_, _| {
        let about_dialog = gtk::AboutDialog::builder()
            .program_name(APP_NAME)
            .title(&format!("About {APP_NAME}"))
            .authors(vec!["James Benner <james.benner@gmail.com>".to_string()])
            .license_type(gtk::License::MitX11)
            .build();

        about_dialog.present();
    });

    let header_bar = build_header_bar();

    window.add_action(&action_about);
    window.set_titlebar(Some(&header_bar));

    for core in cores {
        let core_row = build_core_row(core);

        parent.append(&core_row);
    }

    parent.append(&button_row);
    window.set_title(Some(APP_NAME));
    window.set_child(Some(&parent));
    window.present();
}
