use crate::config::APP_ID;
use gtk::gio;
use gtk::prelude::*;
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde_json;
use std::fs::File;
use std::path::Path;
use std::{env, fs, io};
use zip::ZipArchive;

fn build_github_api_client() -> reqwest::blocking::Client {
    let mut default_headers = HeaderMap::new();
    let settings = gio::Settings::new(APP_ID);
    let access_token = settings.get::<String>("github-access-token");

    if !access_token.is_empty() {
        let mut auth_value = HeaderValue::from_str(format!("Bearer {access_token}").as_str())
            .expect("Could not create header value.");

        auth_value.set_sensitive(true);
        default_headers.insert(header::AUTHORIZATION, auth_value);
    }

    default_headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    default_headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );

    reqwest::blocking::Client::builder()
        .user_agent("PocketUp")
        .default_headers(default_headers)
        .build()
        .unwrap()
}

fn build_http_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .user_agent("PocketUp")
        .build()
        .unwrap()
}

fn get_filename_from_url(url: &str) -> String {
    let filename = url.split('/').last().unwrap();

    filename.to_string()
}

fn get_owner_and_repo_from_github_url(url: &str) -> [String; 2] {
    let mut collection: Vec<&str> = url.split('/').collect();
    let repo = collection.pop().unwrap().to_string();
    let owner = collection.pop().unwrap().to_string();

    [owner, repo]
}

pub fn unzip_to_pocket_dir(zip_path: &str) {
    let settings = gio::Settings::new(APP_ID);
    let base_dir = settings.get::<String>("pocket-base-dir");
    let base_path = Path::new(&base_dir);
    let file = File::open(zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => base_path.join(path), //.to_owned(),
            None => continue,
        };

        // if (*file.name()).ends_with('/') {
        if file.name().ends_with('/') {
            println!(r#"__File {} extracted to "{}""#, i, out_path.display());

            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                r#"File {} extracted to "{}" ({} bytes)"#,
                i,
                out_path.display(),
                file.size()
            );

            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).unwrap();
                }
            }

            let mut out_file = File::create(&out_path).unwrap();

            io::copy(&mut file, &mut out_file).unwrap();
        }
    }
}

pub fn fetch_download(url: &str) {
    println!("{url}");
    let client = build_http_client();
    let response = client
        .get(url)
        .send()
        .expect("Should have been able to fetch the file");
    let bytes = response
        .bytes()
        .expect("Should have been able to read the bytes of the file");
    let filename = get_filename_from_url(url);
    let temp_dir = env::temp_dir().join("pocket-up");
    let temp_dirname = temp_dir.to_str().unwrap();
    let temp_filepath = format!("{temp_dirname}/{filename}");

    match fs::create_dir_all(&temp_dirname) {
        Ok(_) => {}
        Err(error) => eprintln!("Could not create directory path! {error}"),
    }

    match fs::write(&temp_filepath, bytes) {
        Ok(_) => println!("Successfully downloaded {temp_filepath}"),
        Err(error) => eprintln!("Could not write file! {error}"),
    };

    unzip_to_pocket_dir(&temp_filepath);
}

pub fn fetch_github_release(repo_url: &str) {
    let client = build_github_api_client();
    let [owner, repo] = get_owner_and_repo_from_github_url(repo_url);
    let api_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let response = client
        .get(api_url)
        .send()
        .expect("Should have been able to fetch the JSON document");

    match response.status().as_u16() {
        200 => {
            // We found the latest release so now we download it.
            let text = response
                .text()
                .expect("Should have been able to read the text of the document");
            println!("{text}");
            let json = serde_json::from_str::<serde_json::Value>(&text)
                .expect("Should have been able to parse the JSON document");
            let download_url = json["assets"][0]["browser_download_url"].as_str().unwrap();

            fetch_download(download_url);
        }
        404 => {
            // No latest release found so it's probably a pre-release so we query the standard
            // releases API and grab the first result. Might have to adjust this later since
            // I'm not sure if the first result is always going to be the newest.
            let api_url = format!("https://api.github.com/repos/{owner}/{repo}/releases");
            let response = client
                .get(api_url)
                .send()
                .expect("Should have been able to fetch the JSON document");
            let text = response
                .text()
                .expect("Should have been able to read the text of the document");
            println!("{text}");
            let json = serde_json::from_str::<serde_json::Value>(&text)
                .expect("Should have been able to parse the JSON document");
            let download_url = json[0]["assets"][0]["browser_download_url"]
                .as_str()
                .unwrap();

            fetch_download(download_url);
        }
        _ => eprintln!("Got unexpected HTTP response status: {}", response.status()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_filename_from_url_returns_the_filename() {
        let url = "https://github.com/agg23/openfpga-pong/releases/download/1.2.0/agg23.Pong_1.2.0_2022-09-08.zip";
        let filename = get_filename_from_url(url);

        assert_eq!(filename, "agg23.Pong_1.2.0_2022-09-08.zip");
    }

    #[test]
    fn get_owner_and_repo_from_github_url_returns_the_owner() {
        let url = "https://github.com/Mazamars312/Analogue_Pocket_Neogeo";
        let [owner, _] = get_owner_and_repo_from_github_url(url);

        assert_eq!(owner, "Mazamars312");
    }

    #[test]
    fn get_owner_and_repo_from_github_url_returns_the_repo() {
        let url = "https://github.com/Mazamars312/Analogue_Pocket_Neogeo";
        let [_, repo] = get_owner_and_repo_from_github_url(url);

        assert_eq!(repo, "Analogue_Pocket_Neogeo");
    }
}
