use crate::config::APP_ID;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{self, gio};

pub fn build_set_github_access_token_modal(window: &gtk::ApplicationWindow) -> gtk::Window {
    let modal = gtk::Window::builder()
        .title("Set GitHub Access Token")
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
