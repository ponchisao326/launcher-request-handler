use serde::Deserialize;

pub struct Downloader;
pub struct JsonGrabber;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String,
}