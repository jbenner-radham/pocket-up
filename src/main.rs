mod activate;
mod button_row;
mod core_row;

use activate::on_activate;
use gtk::prelude::*;

const APP_ID: &str = "com.radioactivehamster.pocket_up";

fn main() {
    if gtk::gio::Application::id_is_valid(APP_ID) {
        println!("ID is valid!");
    }

    // Create a new application with the builder pattern
    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(on_activate);
    app.run();
}
