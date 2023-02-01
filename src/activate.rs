use crate::button_row::build_button_row;
use crate::core_row::build_core_row;
use gtk::prelude::*;

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

    for core in cores {
        let core_row = build_core_row(core);

        parent.append(&core_row);
    }

    parent.append(&button_row);
    window.set_title(Some("PocketUp"));
    window.set_child(Some(&parent));
    window.present();
}
