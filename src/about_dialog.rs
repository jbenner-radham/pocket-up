use crate::config::APP_NAME;

pub fn build_about_dialog() -> gtk::AboutDialog {
    let manifest_src = include_str!("../Cargo.toml");
    let manifest: toml::Value = toml::from_str(manifest_src).unwrap();
    let version = manifest["package"]["version"].as_str().unwrap();

    gtk::AboutDialog::builder()
        .program_name(APP_NAME)
        .title(format!("About {APP_NAME}"))
        .version(version)
        .authors(vec!["James Benner https://www.jamesbenner.com/".to_string()])
        .license_type(gtk::License::MitX11)
        .build()
}
