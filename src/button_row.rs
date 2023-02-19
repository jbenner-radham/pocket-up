use crate::config::{PocketCore, APP_ID, POCKET_CORES};
use crate::downloader::{fetch_download, fetch_github_release};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{self, gio};
use std::path::Path;

fn build_file_chooser(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
    let title = Some("Select a Folder");
    let parent = Some(window);
    let action = gtk::FileChooserAction::SelectFolder;
    let buttons = &[
        ("_Cancel", gtk::ResponseType::Cancel),
        ("_Select", gtk::ResponseType::Accept),
    ];
    let settings = gio::Settings::new(APP_ID);
    let file_chooser = gtk::FileChooserDialog::new(title, parent, action, buttons);
    let pocket_base_dir = settings.get::<String>("pocket-base-dir");
    let path = Path::new(&pocket_base_dir);

    if !pocket_base_dir.is_empty() && path.exists() {
        let file = gio::File::for_path(path);
        file_chooser
            .set_file(&file)
            .expect("Should have been able to set file.");
    }

    file_chooser
}

pub fn build_button_row(window: &gtk::ApplicationWindow) -> gtk::Box {
    let margin = 8;
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    let directory_button = gtk::Button::builder()
        // Icon names are here: https://developer-old.gnome.org/gtk3/stable/gtk3-Stock-Items.html
        // and supposedly here: https://developer.gnome.org/gtk/stable/gtk-Stock-Items.html
        // if the later ever comes back online.
        .icon_name("folder")
        .tooltip_text("Select Folder")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();
    let update_button = gtk::Button::builder()
        .label("Update")
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .sensitive(false) // Disable initially.
        .build();
    let settings = gio::Settings::new(APP_ID);

    settings
        .bind("is-form-enabled", &update_button, "sensitive")
        .flags(gio::SettingsBindFlags::DEFAULT)
        .build();

    directory_button.connect_clicked(clone!(@weak window => move |_| {
        let file_chooser = build_file_chooser(&window);

        file_chooser.run_async(|dialog, response| {
            dialog.close();

            if response == gtk::ResponseType::Accept {
                let settings = gio::Settings::new(APP_ID);

                if let Some(dir) = dialog.file() {
                    settings
                        .set_string("pocket-base-dir", dir.parse_name().as_str())
                        .expect("Unable to set pocket-base-dir setting.");
                    settings
                        .set_boolean("is-form-enabled", true)
                        .expect("Could not set is-form-enabled setting.");
                    println!("{}", dir.parse_name());
                }
            }
        });
    }));

    update_button.connect_clicked(|_| {
        let settings = gio::Settings::new(APP_ID);
        // let cores_to_download = POCKET_CORES
        //     .iter()
        //     .filter(|core| settings.get::<bool>(&core.settings_name()))
        //     .collect::<Vec<&PocketCore>>()
        //     .len();

        for core in POCKET_CORES {
            let should_download_core = settings.get::<bool>(&core.settings_name());

            if !should_download_core {
                continue;
            }

            if let Some(url) = core.download_url {
                fetch_download(url);
            } else {
                fetch_github_release(core.repo);
            }
        }
    });

    row.append(&directory_button);
    row.append(&update_button);

    row
}
