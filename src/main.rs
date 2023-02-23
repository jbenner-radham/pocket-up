mod about_dialog;
mod activate;
mod button_row;
mod config;
mod core_row;
mod downloader;
mod header;
mod help_window;
mod set_github_access_token_modal;
mod window_child;

use crate::config::APP_ID;
use activate::on_activate;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::path::Path;

// https://docs.gtk.org/gtk4/visual_index.html

fn main() -> glib::ExitCode {
    if !gio::Application::id_is_valid(APP_ID) {
        panic!("The GTK Application ID is not valid!");
    }

    // Create a new application with the builder pattern
    let app = gtk::Application::builder().application_id(APP_ID).build();
    let settings = gio::Settings::new(APP_ID);
    let pocket_base_dir = settings.get::<String>("pocket-base-dir");
    let path = Path::new(&pocket_base_dir);

    // Disable the form if needed.
    if pocket_base_dir.is_empty() || !path.exists() {
        settings
            .set_string("pocket-base-dir", "")
            .expect("Unable to set pocket-base-dir setting.");
        settings
            .set_boolean("is-form-enabled", false)
            .expect("Unable to set is-form-enabled setting.");
    }

    app.connect_activate(on_activate);

    app.run()
}
