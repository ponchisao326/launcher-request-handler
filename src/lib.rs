use std::fs::File;
use reqwest::blocking;
use serde::Deserialize;

pub struct Downloader;
pub struct JsonGrabber;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
}

impl Downloader {
    /// Download a ZIP file from a given URL and save it to the specified destination.
    pub fn download_zip(url: &str, destino: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = blocking::get(url)?;
        let mut file = File::create(destino)?;
        let content = response.bytes()?;
        std::io::copy(&mut content.as_ref(), &mut file)?;
        Ok(())
    }
}

impl JsonGrabber {
    /// Download a JSON file from a given URL and parse it into a Response struct.
    pub fn verify_json(url: &str) -> Result<Version, Box<dyn std::error::Error>> {
        let response = blocking::get(url)?;
        let json: Version = response.json()?;
        Ok(json)
    }
}