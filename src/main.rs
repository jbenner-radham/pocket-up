use gtk::glib::{self,clone};
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

fn build_file_chooser(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
    let title = Some("Select a Directory");
    let parent = Some(window);
    let action = gtk::FileChooserAction::SelectFolder;
    let buttons = &[
        ("_Cancel", gtk::ResponseType::Cancel),
        ("_Select", gtk::ResponseType::Accept)
    ];
    // let buttons = gtk::ButtonsType::OkCancel;

    gtk::FileChooserDialog::new(title, parent, action, buttons)
}

fn build_button_row(window: &gtk::ApplicationWindow) -> gtk::Box {
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
        .icon_name("folder")
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
        .build();

    directory_button.connect_clicked(clone!(@weak window => move |_| {
        let file_chooser = build_file_chooser(&window);

        file_chooser.run_async(|dialog, response| {
            dialog.close();

            if response == gtk::ResponseType::Accept {
                if let Some(dir) = dialog.file() {
                    println!("{}", dir.parse_name().to_string());
                }
            }

            println!("{:?}", response);
        });
    }));

    row.append(&directory_button);
    row.append(&update_button);

    row
}

// When the application is launched...
fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let cores = vec!["Core One", "Core Two", "Core Three"];
    let margin = 12;
    let parent = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .halign(gtk::Align::Center)
        .build();
    let button_row = build_button_row(&window);

    for core in cores {
        let row = build_core_row(core);
        parent.append(&row);
    }

    parent.append(&button_row);
    window.set_title(Some("PocketUp"));
    window.set_child(Some(&parent));
    window.present();
}

fn main() {
    if gtk::gio::Application::id_is_valid(APP_ID) {
        println!("ID is valid!");
    }

    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(on_activate);
    app.run();
}
