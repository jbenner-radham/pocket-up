use gtk::glib::{self, clone};
use gtk::prelude::*;

fn build_file_chooser(window: &gtk::ApplicationWindow) -> gtk::FileChooserDialog {
    let title = Some("Select a Directory");
    let parent = Some(window);
    let action = gtk::FileChooserAction::SelectFolder;
    let buttons = &[
        ("_Cancel", gtk::ResponseType::Cancel),
        ("_Select", gtk::ResponseType::Accept),
    ];

    gtk::FileChooserDialog::new(title, parent, action, buttons)
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

    directory_button.connect_clicked(clone!(@weak window => move |_| {
        let file_chooser = build_file_chooser(&window);

        file_chooser.run_async(|dialog, response| {
            dialog.close();

            if response == gtk::ResponseType::Accept {
                if let Some(dir) = dialog.file() {
                    println!("{}", dir.parse_name());
                }
            }

            println!("{:?}", response);
        });
    }));

    row.append(&directory_button);
    row.append(&update_button);

    row
}
