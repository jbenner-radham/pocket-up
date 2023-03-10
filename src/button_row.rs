use crate::config::{APP_ID, COLUMN_WIDTH, POCKET_CORES};
use crate::downloader::{fetch_bios, fetch_download, fetch_github_release};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{self, gio};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::thread;

fn build_modal_child() -> gtk::Box {
    let margin = 12;

    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .spacing(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build()
}

fn build_error_modal(error: &anyhow::Error, window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("Error")
        .transient_for(window)
        .modal(true)
        .build();
    let child = build_modal_child();
    let text = textwrap::wrap(&error.to_string(), COLUMN_WIDTH).join("\n");
    let label = gtk::Label::new(Some(&text));
    let ok_button = gtk::Button::with_mnemonic("_OK");

    ok_button.add_css_class("error-modal__button--ok");
    ok_button.connect_clicked(clone!(@weak modal => move |_| modal.close()));

    child.append(&label);
    child.append(&ok_button);

    modal.set_child(Some(&child));

    modal
}

fn build_no_openfpga_cores_selected_modal(window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("No openFPGA Cores Selected")
        .transient_for(window)
        .modal(true)
        .build();
    let child = build_modal_child();
    let text = textwrap::wrap(
        "No openFPGA cores have been selected for download. Please select some and try again.",
        COLUMN_WIDTH,
    )
    .join("\n");
    let label = gtk::Label::new(Some(&text));
    let ok_button = gtk::Button::with_mnemonic("_OK");

    ok_button.add_css_class("error-modal__button--ok");
    ok_button.connect_clicked(clone!(@weak modal => move |_| modal.close()));

    child.append(&label);
    child.append(&ok_button);

    modal.set_child(Some(&child));

    modal
}

fn build_success_modal(window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("Success")
        .transient_for(window)
        .modal(true)
        .build();
    let child = build_modal_child();
    let text = textwrap::wrap(
        "Successfully updated your openFPGA core(s)! Have a fun! 🎉",
        COLUMN_WIDTH,
    )
    .join("\n");
    let label = gtk::Label::new(Some(&text));
    let ok_button = gtk::Button::with_mnemonic("_OK");

    ok_button.add_css_class("error-modal__button--ok");
    ok_button.connect_clicked(clone!(@weak modal => move |_| modal.close()));

    child.append(&label);
    child.append(&ok_button);

    modal.set_child(Some(&child));

    modal
}

fn build_partial_success_modal(window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("Partial Success")
        .transient_for(window)
        .modal(true)
        .build();
    let child = build_modal_child();
    let text = textwrap::wrap(
        "Your openFPGA core(s) have been updated but some errors were encountered.",
        COLUMN_WIDTH,
    )
    .join("\n");
    let label = gtk::Label::new(Some(&text));
    let ok_button = gtk::Button::with_mnemonic("_OK");

    ok_button.connect_clicked(clone!(@weak modal => move |_| modal.close()));

    child.append(&label);
    child.append(&ok_button);

    modal.set_child(Some(&child));

    modal
}

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
                }
            }
        });
    }));

    update_button.connect_clicked(clone!(@weak window => move |button| {
        let button_label = button.label().expect("Could not get button label.").to_string();
        let settings = gio::Settings::new(APP_ID);
        let cores: Vec<_> = POCKET_CORES.iter().filter(|core| settings.get::<bool>(&core.settings_name())).collect();
        let cores_to_download = cores.len();

        button.set_sensitive(false);
        button.set_label("Updating...");

        if cores_to_download == 0 {
            build_no_openfpga_cores_selected_modal(&window).present();
        } else {
            let number_of_cores_downloaded = Rc::new(RefCell::new(0));
            let number_of_errors = Rc::new(RefCell::new(0));
            let (tx_error, rx_error) = glib::MainContext::channel::<anyhow::Error>(glib::PRIORITY_DEFAULT);
            let (tx_success, rx_success) = glib::MainContext::channel::<&str>(glib::PRIORITY_DEFAULT);


            for core in cores {
                thread::spawn(clone!(@strong tx_error, @strong tx_success => move || {
                    if let Some(url) = core.download_url {
                        match fetch_download(url) {
                            Ok(_) => {}
                            Err(error) => {
                                tx_error.send(error).expect("Could not send error message.");
                            }
                        };
                    } else {
                        match fetch_github_release(core.repo) {
                            Ok(_) => {}
                            Err(error) => {
                                tx_error.send(error).expect("Could not send error message.");
                            }
                        };
                    }

                    for bios in core.bios_files {
                        match fetch_bios(bios) {
                            Ok(_) => {}
                            Err(error) => {
                                tx_error.send(error).expect("Could not send error message.");
                            }
                        };
                    }

                    tx_success.send("Core updated!").expect("Could not send success message.");
                }));
            }

            rx_error.attach(None, clone!(@weak window, @strong number_of_errors => @default-return glib::Continue(false), move |error| {
                *number_of_errors.borrow_mut() += 1;

                build_error_modal(&error, &window).present();

                glib::Continue(true)
            }));

            rx_success.attach(None, clone!(@strong button, @strong number_of_errors, @strong number_of_cores_downloaded, @strong cores_to_download => move |_| {
                *number_of_cores_downloaded.borrow_mut() += 1;

                if *number_of_cores_downloaded.borrow() != cores_to_download {
                    return glib::Continue(true);
                }

                if *number_of_errors.borrow() ==  0 {
                    build_success_modal(&window).present();
                } else {
                    build_partial_success_modal(&window).present();
                }

                button.set_sensitive(true);
                button.set_label(&button_label);

                glib::Continue(false)
            }));
        }
    }));

    row.append(&directory_button);
    row.append(&update_button);

    row
}
