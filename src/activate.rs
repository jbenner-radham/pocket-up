use crate::button_row::build_button_row;
use crate::config::{APP_NAME, POCKET_CORES};
use crate::core_row::build_core_row;
use gtk::prelude::*;
use gtk::{self, gio};

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
        .vexpand(true)
        .build()
}

fn build_header() -> gtk::Label {
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

fn build_about_dialog() -> gtk::AboutDialog {
    gtk::AboutDialog::builder()
        .program_name(APP_NAME)
        .title(&format!("About {APP_NAME}"))
        .authors(vec!["James Benner <james.benner@gmail.com>".to_string()])
        .license_type(gtk::License::MitX11)
        .build()
}

fn build_header_bar() -> gtk::HeaderBar {
    let menu = gio::Menu::new();

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

// When the application is launched...
pub fn on_activate(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let parent = build_parent();
    let header = build_header();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .build();
    let scrolled_child = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();
    let button_row = build_button_row(&window);
    let action_about = gio::SimpleAction::new("about", None);

    action_about.connect_activate(|_, _| build_about_dialog().present());

    let header_bar = build_header_bar();

    window.add_action(&action_about);
    window.set_titlebar(Some(&header_bar));

    for core in POCKET_CORES {
        let core_row = build_core_row(&core.description_markup());

        scrolled_child.append(&core_row);
    }

    scrolled_window.set_child(Some(&scrolled_child));
    parent.append(&header);
    parent.append(&scrolled_window);
    parent.append(&button_row);
    window.set_title(Some(APP_NAME));
    window.set_child(Some(&parent));
    window.present();
}
