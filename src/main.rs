mod activate;
mod button_row;
mod config;
mod core_row;
mod downloader;

use crate::config::APP_ID;
use activate::on_activate;
use gtk::gio;
use gtk::prelude::*;
use std::path::Path;

fn main() {
    if !gio::Application::id_is_valid(APP_ID) {
        panic!("The GTK Application ID is not valid!");
    }

    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        // .flags(gtk::gio::ApplicationFlags::empty())
        // .flags(Default::default())
        // .settings(&settings)
        .build();

    app.connect_activate(on_activate);
    app.run();

    let settings = gio::Settings::new(APP_ID);
    let pocket_base_dir = settings.get::<String>("pocket-base-dir");
    let path = Path::new(&pocket_base_dir);

    // Disable the form if needed.
    if pocket_base_dir.is_empty() || !path.exists() {
        settings
            .set_string("pocket-base-dir", "")
            .expect("Unable to set pocket-base-dir setting.");
        settings
            .set_boolean("are-switches-enabled", false)
            .expect("Unable to set are-switches-enabled setting.");
    }

    // settings.set_boolean("gtk-application-prefer-dark-theme", true).expect("Could not set setting.");
}
