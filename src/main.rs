use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::{error::Error, io::Write as _};

use crossterm::{event::read, event::Event as CEvent};
use zip::read::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let octocrab = octocrab::instance();

    let latest = octocrab
        .repos("MolotovCherry", "zed-windows-builds")
        .releases()
        .get_latest()
        .await?;

    let tag = latest.tag_name;

    println!("Found release {tag}");

    let asset = latest
        .assets
        .first()
        .ok_or(format!("No asset found on latest release {tag}"))?;

    println!("Downloading asset {}", asset.name);

    let data = reqwest::get(asset.browser_download_url.clone())
        .await?
        .bytes()
        .await?;

    let path = Path::new(&asset.name);

    let ext = path
        .extension()
        .ok_or("asset has no extension")?
        .to_string_lossy();

    match &*ext {
        "zip" => {
            let cursor = Cursor::new(&data);
            let mut zip = ZipArchive::new(cursor)?;

            zip.extract(".")?;

            for filename in zip.file_names() {
                println!("File: {filename}");
            }
        }

        "exe" => {
            fs::write(&asset.name, data)?;
            println!("File: {}", asset.name);
        }

        _ => Err(format!("extension {ext} is unsupported"))?,
    }

    if let Some(body) = latest.body {
        println!("\n{}", termimad::term_text(&body));
        pause();
    }

    Ok(())
}

pub fn pause() {
    print!("Press any key to continue...");
    std::io::stdout().flush().unwrap();

    loop {
        match read().unwrap() {
            CEvent::Key(_event) => break,
            _ => continue,
        }
    }
}
