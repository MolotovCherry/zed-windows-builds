use std::path::Path;
use std::{env, fs};
use std::{error::Error, io::Write as _};
use std::{io::Cursor, str::FromStr as _};

use crossterm::{event::read, event::Event as CEvent};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use strum::{EnumProperty, EnumString, VariantNames};
use zip::read::ZipArchive;

#[derive(EnumString, VariantNames, EnumProperty)]
#[strum(ascii_case_insensitive)]
enum Asset {
    #[strum(props(name = "zed-opengl.exe"))]
    OpenGl,
    #[strum(props(name = "zed-opengl.zip"))]
    ZipOpenGl,
    #[strum(props(name = "zed.exe"))]
    Vulkan,
    #[strum(props(name = "zed.zip"))]
    ZipVulkan,
}

fn help() {
    let mut msg = "zed-dl <ASSET>\n\nAsset types are as follows (case-insensitive):\n".to_owned();

    for name in Asset::VARIANTS {
        msg.push_str(name);
        msg.push('\n');
    }

    println!("{msg}");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Some(asset) = env::args().nth(1) else {
        help();
        return Ok(());
    };

    match &*asset {
        "--help" | "-h" => {
            help();

            return Ok(());
        }

        _ => (),
    }

    let looking_for_asset = Asset::from_str(&asset)?.get_str("name").unwrap();

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
        .iter()
        .find(|asset| asset.name.eq_ignore_ascii_case(looking_for_asset))
        .ok_or_else(|| format!("No asset found on latest release {tag}"))?;

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

    match &*ext.to_ascii_lowercase() {
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
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&body, options);
        let data = parser
            .map(|event| match event {
                Event::Start(t) => match t {
                    Tag::Heading { level, .. } => match level {
                        HeadingLevel::H1 => "# ".to_owned(),
                        HeadingLevel::H2 => "## ".to_owned(),
                        HeadingLevel::H3 => "### ".to_owned(),
                        HeadingLevel::H4 => "#### ".to_owned(),
                        HeadingLevel::H5 => "##### ".to_owned(),
                        HeadingLevel::H6 => "###### ".to_owned(),
                    },
                    Tag::Table(_) => "\n| - | - |".to_owned(),
                    Tag::TableHead => "\n".to_owned(),
                    Tag::TableRow => "\n".to_owned(),
                    Tag::TableCell => "|".to_owned(),
                    Tag::Link { .. } => "".to_owned(),
                    Tag::CodeBlock(_) => "```\n".to_owned(),
                    Tag::Strikethrough => "~~".to_owned(),
                    Tag::Strong => "**".to_owned(),
                    Tag::List(_) => "".to_owned(),
                    Tag::Item => "* ".to_owned(),
                    _ => unimplemented!("tag unimplemented: {t:?}"),
                },
                Event::End(t) => match t {
                    TagEnd::Table => "".to_owned(),
                    TagEnd::TableHead => "|".to_owned(),
                    TagEnd::TableRow => "|".to_owned(),
                    TagEnd::TableCell => "".to_owned(),
                    TagEnd::Heading(_) => "".to_owned(),
                    TagEnd::Link => "".to_owned(),
                    TagEnd::CodeBlock => "\n```".to_owned(),
                    TagEnd::Strikethrough => "~~".to_owned(),
                    TagEnd::Strong => "**".to_owned(),
                    TagEnd::Item => "".to_owned(),
                    TagEnd::List(_) => "".to_owned(),
                    t => unimplemented!("tagend unimplemented: {t:?}"),
                },
                Event::Text(t) => t.to_string(),
                Event::Code(c) => format!("`{c}`"),

                e => unimplemented!("event unimplemented: {e:?}"),
            })
            .collect::<String>();

        println!("\n{}", termimad::term_text(&data));
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
