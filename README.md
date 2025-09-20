# Minecraft Launcher Updater Library

This Rust library is designed to facilitate version checking, file downloading, and retrieving information from remote JSON endpoints—perfect for Minecraft launchers or any project needing a simple and robust update system.

## Features

- **Version check:** Check if a new version is available by comparing the current version to one published at a remote endpoint.
- **Update panel:** Retrieve details to display update info (description, changelog, image, etc.) from a remote JSON file.
- **File download:** Download ZIP files from a URL and store them at a specified location.
- **JSON utilities:** Download and deserialize JSON files into any type implementing `Deserialize`.

## Example Usage

```rust
use launcher-request-handler::{Downloader, Update};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for a new version
    let has_update = Update::check_for_update("1.0.0", "https://myapi.com/version.json")?;
    if has_update {
        println!("A new version is available!");
    }

    // Get update panel details
    let details = Update::update_panel_details("https://myapi.com/update_panel.json")?;
    println!("Title: {}", details.title);

    // Download the update ZIP
    Downloader::download_zip(&details.download_url, "path/to/destination/file.zip")?;
    
    Ok(())
}
```

## Main Structures

- `Downloader`: Methods for downloading files.
- `JsonGrabber`: Methods for downloading and deserializing JSON.
- `Update`: Integrates version checking and update panel logic.

### Example Version JSON

```json
{
  "version": "1.2.3"
}
```

### Example Update Panel JSON

```json
{
  "new_version": "1.2.3",
  "title": "New update available!",
  "description": "New features have been added.",
  "image_url": "https://example.com/image.png",
  "date": "2025-09-20",
  "change_log": [
    "Performance improvements",
    "Bug fixes"
  ],
  "download_url": "https://example.com/update.zip"
}
```

## Installation

Add these dependencies to your `Cargo.toml`:

```toml
reqwest = { version = "0.11", features = ["blocking", "json"] }
serde = { version = "1.0", features = ["derive"] }
```

## Motivation

Originally created for my own Minecraft launcher, but published to share knowledge on how to handle automatic updates in desktop applications using Rust.
