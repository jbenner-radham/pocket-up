use crate::builders::{
    build_about_dialog, build_add_github_access_token_modal, build_header, build_header_bar,
    build_parent,
};
use crate::button_row::build_button_row;
use crate::config::{APP_NAME, POCKET_CORES};
use crate::core_row::build_core_row;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{self, gio};

pub fn on_activate(app: &gtk::Application) {
    // https://developer-old.gnome.org/gtk4/stable/GtkSettings.html
    if let Some(settings) = gtk::Settings::default() {
        // settings.set_gtk_application_prefer_dark_theme(true);

        // Hack to work around the issue with `gtk::Entry` crashing on left or right keypress.
        settings.set_gtk_entry_select_on_focus(false);
    }

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
    let action_add_github_access_token = gio::SimpleAction::new("add-github-access-token", None);
    let action_about = gio::SimpleAction::new("about", None);

    action_add_github_access_token.connect_activate(clone!(@weak window => move |_, _| {
        build_add_github_access_token_modal(&window).present();
    }));

    action_about.connect_activate(|_, _| build_about_dialog().present());

    let header_bar = build_header_bar();

    window.add_action(&action_add_github_access_token);
    window.add_action(&action_about);
    window.set_titlebar(Some(&header_bar));

    for core in POCKET_CORES {
        let core_row = build_core_row(&core);

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
