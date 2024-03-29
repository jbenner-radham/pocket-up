use crate::about_dialog::build_about_dialog;
use crate::button_row::build_button_row;
use crate::config::{APP_NAME, POCKET_CORES};
use crate::core_row::build_core_row;
use crate::header::{build_header, build_header_bar};
use crate::help_window::build_help_window;
use crate::set_github_access_token_modal::build_set_github_access_token_modal;
use crate::window_child::build_window_child;
use adw::prelude::*;
use gtk::gdk;
use gtk::glib::{self, clone};
use gtk::{self, gio};

fn load_css(settings: &gtk::Settings) {
    let display = gdk::Display::default().expect("Could not get default display.");
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;
    let provider = gtk::CssProvider::new();
    let theme_name = settings
        .gtk_theme_name()
        .expect("Could not get theme name.");

    // Check if the current them is a dark variant or if prefer dark theme is set in the GTK settings
    // and then set the appropriate CSS theme. On the current Ubuntu LTS (22.04) changing the appearance
    // in the settings app only changes the theme it doesn't trigger the prefer dark theme GTK setting.
    // Not sure if that behaviour will change in the future.
    if theme_name.to_lowercase().contains("dark") || settings.is_gtk_application_prefer_dark_theme()
    {
        provider.load_from_data(include_str!("../resources/styles/dark.css"));
    } else {
        provider.load_from_data(include_str!("../resources/styles/light.css"));
    }

    gtk::StyleContext::add_provider_for_display(&display, &provider, priority);
}

pub fn on_activate(app: &adw::Application) {
    // https://developer-old.gnome.org/gtk4/stable/GtkSettings.html
    if let Some(settings) = gtk::Settings::default() {
        // Use this for testing dark mode.
        // settings.set_gtk_application_prefer_dark_theme(true);

        // Hack to work around the issue with `gtk::Entry` crashing on left or right keypress.
        settings.set_gtk_entry_select_on_focus(false);

        // Listen for dark or light mode events and reload the CSS.
        settings.connect_gtk_application_prefer_dark_theme_notify(load_css);
        settings.connect_gtk_theme_name_notify(load_css);

        // Initial load of CSS theme.
        load_css(&settings);
    }

    let window = adw::ApplicationWindow::new(app);
    let window_child = build_window_child();
    let header = build_header();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .build();
    let scrolled_child = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let action_set_github_access_token = gio::SimpleAction::new("set-github-access-token", None);
    let action_help = gio::SimpleAction::new("help", None);
    let action_about = gio::SimpleAction::new("about", None);
    let header_bar = build_header_bar();
    let button_row = build_button_row(&window);
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);

    action_set_github_access_token.connect_activate(clone!(@weak window => move |_, _| {
        build_set_github_access_token_modal(&window).present();
    }));

    action_help.connect_activate(|_, _| build_help_window().present());

    action_about.connect_activate(|_, _| build_about_dialog().present());

    window.add_action(&action_set_github_access_token);
    window.add_action(&action_help);
    window.add_action(&action_about);

    let list_box = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    for core in POCKET_CORES {
        let core_row = build_core_row(&core);

        list_box.append(&core_row);
    }

    scrolled_child.append(&list_box);

    scrolled_window.set_child(Some(&scrolled_child));

    window_child.append(&header);
    window_child.append(&scrolled_window);
    window_child.append(&button_row);

    content.append(&header_bar);
    content.append(&window_child);

    window.set_title(Some(APP_NAME));
    window.set_content(Some(&content));
    window.present();
}
