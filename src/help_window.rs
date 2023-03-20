use crate::config::{APP_NAME, COLUMN_WIDTH};
use adw::prelude::*;

fn build_section_header_markup(header: &str) -> String {
    format!(r#"<span size="large"><b>{header}</b></span>"#)
}

fn build_section_text(sentences: &[&str]) -> String {
    let text = sentences.join(" ");

    textwrap::wrap(&text, COLUMN_WIDTH).join("\n")
}

pub fn build_help_window() -> adw::Window {
    let help_window = adw::Window::builder()
        .title(format!("{APP_NAME} Help"))
        .build();
    let margin = 12;
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .spacing(margin)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    let main_header = gtk::Label::new(None);
    let main_section = gtk::Label::new(Some(&format!("Welcome to the {APP_NAME} help guide.")));

    main_header.set_markup(&format!(
        r#"<span size="xx-large"><b>{APP_NAME}</b></span>"#
    ));

    let selecting_your_folder_header = gtk::Label::new(None);
    let selecting_your_folder_section_text = build_section_text(&[
        "Before you start, the first thing you need to do is select the destination folder for your Analogue Pocket's microSD card.",
        r#"You can do this by clicking the folder icon with the "Select Folder" tooltip in the lower left-hand corner of the app."#,
        "This will open up a file chooser dialog, simply select your card and hit OK."
    ]);
    let selecting_your_folder_section = gtk::Label::new(Some(&selecting_your_folder_section_text));

    selecting_your_folder_header.set_markup(&build_section_header_markup("Selecting Your Folder"));

    let selecting_your_openfpga_cores_header = gtk::Label::new(None);
    let selecting_your_openfpga_cores_section_text = build_section_text(&[
        "Before adding or updating your openFPGA cores you'll need to select them.",
        r#"Simply browse the "openFPGA Cores" section to find your desired cores and click their corresponding switches to change them to the on position."#,
    ]);
    let selecting_your_openfpga_cores_section =
        gtk::Label::new(Some(&selecting_your_openfpga_cores_section_text));

    selecting_your_openfpga_cores_header.set_markup(&build_section_header_markup(
        "Selecting Your openFPGA Cores",
    ));

    let updating_your_analogue_pocket_header = gtk::Label::new(None);
    let updating_your_analogue_pocket_section_text = build_section_text(&[
        r#"Now that your ready to load up or update your Analogue Pocket just click the "Update" button in the bottom-right corner of the app."#,
        "Give the app a few seconds to download and extract the files and you're good to go.",
    ]);
    let updating_your_analogue_pocket_section =
        gtk::Label::new(Some(&updating_your_analogue_pocket_section_text));

    updating_your_analogue_pocket_header.set_markup(&build_section_header_markup(
        "Updating Your Analogue Pocket",
    ));

    let setting_a_github_access_token_header = gtk::Label::new(None);
    let setting_a_github_access_token_section_text = build_section_text(&[
        "The step is optional.",
        "But if you ever have issues downloading openFPGA cores it's probably because you need to set a GitHub access token for the app.",
        "Please reference the GitHub Docs[1] website for instructions on how to do so.",
        r#"Once you've done this click the hamburger menu with the tooltip "Main Menu" in the right hand side of the app's titlebar."#,
        r#"From there simply click the "Set GitHub Access Token" menu option and enter your access token in the entry field and click "OK"."#
    ]);
    let setting_a_github_access_token_section =
        gtk::Label::new(Some(&setting_a_github_access_token_section_text));

    setting_a_github_access_token_header.set_markup(&build_section_header_markup(
        "Setting a GitHub Access Token",
    ));

    let reference_link = gtk::Label::new(None);

    reference_link.set_markup(r#"1. <a href="https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token#creating-a-personal-access-token-classic">GitHub Docs - Creating a Personal Access Token</a>"#);

    container.append(&main_header);
    container.append(&main_section);
    container.append(&selecting_your_folder_header);
    container.append(&selecting_your_folder_section);
    container.append(&selecting_your_openfpga_cores_header);
    container.append(&selecting_your_openfpga_cores_section);
    container.append(&updating_your_analogue_pocket_header);
    container.append(&updating_your_analogue_pocket_section);
    container.append(&setting_a_github_access_token_header);
    container.append(&setting_a_github_access_token_section);
    container.append(&reference_link);
    help_window.set_child(Some(&container));

    help_window
}
