use crate::config::{PocketCoreBios, APP_ID};
use anyhow::anyhow;
use gtk::gio;
use gtk::prelude::*;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::StatusCode;
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

pub fn unzip_bios(bios: &PocketCoreBios, zip_path: &str) -> anyhow::Result<()> {
    let settings = gio::Settings::new(APP_ID);
    let base_dir = settings.get::<String>("pocket-base-dir");
    let base_path = Path::new(&base_dir);
    let zip_file = File::open(zip_path).unwrap();
    let mut archive = match ZipArchive::new(zip_file) {
        Ok(archive) => archive,
        Err(error) => return Err(anyhow!("Could not create new zip archive: {error}")),
    };
    if let Some(path_in_zip) = bios.path_in_zip {
        let mut file = archive.by_name(path_in_zip).unwrap();
        let bios_path = Path::new(bios.path);
        let out_path = base_path.join(bios_path);

        if let Some(parent) = out_path.parent() {
            if !parent.exists() {
                match fs::create_dir_all(parent) {
                    Ok(_) => {}
                    Err(error) => {
                        return Err(anyhow!(
                            "Could not create directory path {}: {error}",
                            parent.file_name().unwrap().to_str().unwrap()
                        ))
                    }
                };
            }
        }

        let mut out_file = match File::create(&out_path) {
            Ok(out_file) => out_file,
            Err(error) => {
                return Err(anyhow!(
                    "Could not create file {}: {error}",
                    out_path.file_name().unwrap().to_str().unwrap()
                ))
            }
        };

        match io::copy(&mut file, &mut out_file) {
            Ok(_) => {}
            Err(error) => return Err(anyhow!("Could not copy file {}: {error}", file.name())),
        };
    }

    Ok(())
}

pub fn fetch_bios(bios: &PocketCoreBios) -> anyhow::Result<()> {
    let client = build_http_client();
    let response = match client.get(bios.url).send() {
        Ok(response) => response,
        Err(error) => return Err(anyhow!("An error occurred while downloading: {error}")),
    };
    let bytes = match response.bytes() {
        Ok(bytes) => bytes,
        Err(error) => {
            return Err(anyhow!(
                "An error occurred when reading the bytes of the download: {error}"
            ))
        }
    };
    let filename = get_filename_from_url(bios.url);

    if filename.ends_with(".zip") {
        let temp_dir = env::temp_dir().join("pocket-up");
        let temp_dirname = temp_dir.to_str().unwrap();
        let temp_filepath = format!("{temp_dirname}/{filename}");

        match fs::create_dir_all(temp_dirname) {
            Ok(_) => {}
            Err(error) => return Err(anyhow!("Could not create directory path: {error}")),
        };

        match fs::write(&temp_filepath, bytes) {
            Ok(_) => {}
            Err(error) => return Err(anyhow!("Could not write file: {error}")),
        };

        unzip_bios(bios, &temp_filepath)
    } else {
        let settings = gio::Settings::new(APP_ID);
        let base_dir = settings.get::<String>("pocket-base-dir");
        let base_path = Path::new(&base_dir);
        let bios_path = Path::new(bios.path);
        let filepath = base_path.join(bios_path);

        if let Some(parent) = filepath.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => {}
                Err(error) => return Err(anyhow!("Could not create directory path: {error}")),
            };

            match fs::write(filepath, bytes) {
                Ok(_) => {}
                Err(error) => return Err(anyhow!("Could not write file: {error}")),
            };
        }

        Ok(())
    }
}

pub fn unzip_to_pocket_dir(zip_path: &str) -> anyhow::Result<()> {
    let settings = gio::Settings::new(APP_ID);
    let base_dir = settings.get::<String>("pocket-base-dir");
    let base_path = Path::new(&base_dir);
    let file = File::open(zip_path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for index in 0..archive.len() {
        let mut file = archive.by_index(index).unwrap();
        let out_path = match file.enclosed_name() {
            Some(path) => base_path.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            match fs::create_dir_all(&out_path) {
                Ok(_) => {}
                Err(error) => {
                    return Err(anyhow!(
                        "Could not create directory path {}: {error}",
                        out_path.display()
                    ))
                }
            };
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    match fs::create_dir_all(parent) {
                        Ok(_) => {}
                        Err(error) => {
                            return Err(anyhow!(
                                "Could not create directory path {}: {error}",
                                parent.file_name().unwrap().to_str().unwrap()
                            ))
                        }
                    };
                }
            }

            let mut out_file = match File::create(&out_path) {
                Ok(out_file) => out_file,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not create file {}: {error}",
                        out_path.file_name().unwrap().to_str().unwrap()
                    ))
                }
            };

            match io::copy(&mut file, &mut out_file) {
                Ok(_) => {}
                Err(error) => return Err(anyhow!("Could not copy file {}: {error}", file.name())),
            };
        }
    }

    Ok(())
}

pub fn fetch_download(url: &str) -> anyhow::Result<()> {
    let client = build_http_client();
    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(error) => return Err(anyhow!("An error occurred while downloading: {error}")),
    };
    let bytes = match response.bytes() {
        Ok(bytes) => bytes,
        Err(error) => {
            return Err(anyhow!(
                "An error occurred when reading the bytes of the download: {error}"
            ))
        }
    };
    let filename = get_filename_from_url(url);
    let temp_dir = env::temp_dir().join("pocket-up");
    let temp_dirname = temp_dir.to_str().unwrap();
    let temp_filepath = format!("{temp_dirname}/{filename}");

    match fs::create_dir_all(temp_dirname) {
        Ok(_) => {}
        Err(error) => return Err(anyhow!("Could not create directory path: {error}")),
    };

    match fs::write(&temp_filepath, bytes) {
        Ok(_) => {}
        Err(error) => return Err(anyhow!("Could not write file: {error}")),
    };

    unzip_to_pocket_dir(&temp_filepath)
}

pub fn fetch_github_release(repo_url: &str) -> anyhow::Result<()> {
    let client = build_github_api_client();
    let [owner, repo] = get_owner_and_repo_from_github_url(repo_url);
    let api_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let response = client
        .get(api_url)
        .send()
        .expect("Should have been able to fetch the JSON document");

    match response.status() {
        StatusCode::OK => {
            // We found the latest release so now we download it.
            let text = match response.text() {
                Ok(text) => text,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not read the text of the GitHub API response: {error}"
                    ))
                }
            };
            let json = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => json,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not parse the GitHub API response as JSON: {error}"
                    ))
                }
            };
            let download_url = match json["assets"][0]["browser_download_url"].as_str() {
                Some(download_url) => download_url,
                None => {
                    return Err(anyhow!(
                        "Could not find the download URL in the GitHub API response."
                    ))
                }
            };

            fetch_download(download_url)
        }
        StatusCode::UNAUTHORIZED => {
            // Got "Unauthorized" from the server.
            return Err(anyhow!(
                r#"The server returned "Unauthorized". If you have a GitHub access token set it may be incorrect or expired."#
            ));
        }
        StatusCode::NOT_FOUND => {
            // No latest release found so it's probably a pre-release so we query the standard
            // releases API and grab the first result. Might have to adjust this later since
            // I'm not sure if the first result is always going to be the newest.
            let api_url = format!("https://api.github.com/repos/{owner}/{repo}/releases");
            let response = match client.get(api_url).send() {
                Ok(response) => response,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not fetch the JSON document from the GitHub API: {error}"
                    ))
                }
            };
            let text = match response.text() {
                Ok(text) => text,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not read the text of the GitHub API response: {error}"
                    ))
                }
            };
            let json = match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(json) => json,
                Err(error) => {
                    return Err(anyhow!(
                        "Could not parse the GitHub API response as JSON: {error}"
                    ))
                }
            };
            let download_url = match json[0]["assets"][0]["browser_download_url"].as_str() {
                Some(download_url) => download_url,
                None => {
                    return Err(anyhow!(
                        "Could not find the download URL in the GitHub API response."
                    ))
                }
            };

            fetch_download(download_url)
        }
        _ => {
            return Err(anyhow!(
                "Got unexpected HTTP response status from the GitHub API: {}",
                response.status()
            ))
        }
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
