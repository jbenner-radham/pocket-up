use crate::config::APP_NAME;

pub fn build_about_dialog() -> gtk::AboutDialog {
    gtk::AboutDialog::builder()
        .program_name(APP_NAME)
        .title(&format!("About {APP_NAME}"))
        .authors(vec!["James Benner https://www.jamesbenner.com/".to_string()])
        .license_type(gtk::License::MitX11)
        .build()
}
