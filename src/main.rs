use gtk::prelude::*;

const APP_ID: &str = "com.radioactivehamster.pocket_up";

fn build_core_row(core_name: &str) -> gtk::Box {
    let margin = 8;
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .spacing(2)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Start)
        .build();
    let switch = gtk::Switch::builder()
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .state(false)
        .build();
    let label = gtk::Label::builder()
        .label(core_name)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    row.append(&switch);
    row.append(&label);

    row
}

// When the application is launched...
fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let parent_margin = 12;
    let parent = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(parent_margin)
        .margin_bottom(parent_margin)
        .margin_start(parent_margin)
        .margin_end(parent_margin)
        .spacing(parent_margin)
        .halign(gtk::Align::Center)
        .build();
    let cores = vec!["Core One", "Core Two", "Core Three"];

    for core in cores {
        let row = build_core_row(core);
        parent.append(&row);
    }

    let update_button = gtk::Button::builder().label("Update").build();

    parent.append(&update_button);
    window.set_title(Some("PocketUp"));
    window.set_child(Some(&parent));
    window.present();
}

fn main() {
    if gtk::gio::Application::id_is_valid(APP_ID) {
        println!("ID is valid!");
    }

    // Create a new application with the builder pattern
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(on_activate);
    app.run();
}
