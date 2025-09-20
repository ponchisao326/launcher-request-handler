use std::{io::copy, path::Path, fs::{create_dir_all, File}};
use reqwest::blocking;
use serde::{de::DeserializeOwned, Deserialize};

pub struct Downloader;
pub struct JsonGrabber;
pub struct Update;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePanelDetails {
    pub new_version: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub date: String,
    pub change_log: Vec<String>,
    pub download_url: String
}

impl Update {
    /// Check if a new version is available by comparing the current version with the latest version from a URL.
    pub fn check_for_update(current_version: &str, url: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let latest_version: Version = JsonGrabber::verify_json(url)?;
        Ok(latest_version.version != current_version)
    }

    /// Fetch update panel details from a given URL.
    pub fn update_panel_details(url: &str) -> Result<UpdatePanelDetails, Box<dyn std::error::Error>> {
        let details: UpdatePanelDetails = JsonGrabber::verify_json(url)?;
        Ok(details)
    }
}

impl Downloader {
    /// Download a ZIP file from a given URL and save it to the specified destination.
    pub fn download_zip(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
        if Self::folder_exists(Path::new(destination).parent().unwrap().to_str().unwrap()) == false {
            create_dir_all(Path::new(destination).parent().unwrap())?;
        }

        let response = blocking::get(url)?;
        let mut file = File::create(destination)?;
        let content = response.bytes()?;
        copy(&mut content.as_ref(), &mut file)?;
        Ok(())
    }

    /// Check if the folder exists in the local path.
    pub fn folder_exists(path: &str) -> bool {
        Path::new(path).is_dir()
    }
}

impl JsonGrabber {
    /// Download a JSON file from a given URL and parse it into a Response struct.
    pub fn verify_json<T: DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
        let response = blocking::get(url)?;
        let json = response.json::<T>()?;
        Ok(json)
    }
}