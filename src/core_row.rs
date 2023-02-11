use gtk::prelude::*;

pub fn build_core_row(core_description: &str) -> gtk::Box {
    let margin = 8;
    let row_horizontal_margin = 28;
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(row_horizontal_margin)
        .margin_end(row_horizontal_margin)
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
        .sensitive(false) // Disable initially.
        .state(false)
        .build();
    let label = gtk::Label::builder()
        .label(core_description)
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
