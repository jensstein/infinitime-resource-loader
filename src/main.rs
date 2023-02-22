//! InfiniTime external resource loader
//!
//! Loads resources into an InfiniTime device.
//! <https://github.com/InfiniTimeOrg/InfiniTime/blob/develop/doc/ExternalResources.md>

#![warn(missing_docs)]
#![deny(missing_docs)]

mod errors;

use std::io::Read;
use std::path::Path;

use adafruit_ble_fs_client::AdafruitFileTransferClient;
use adafruit_ble_fs_client::providers::btleplug_provider::BtleplugDevice;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use zip::ZipArchive;

use crate::errors::Error;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Flashes contents of a resource archive unto the device
    Flash {
        /// Path for the resources archive
        #[arg(value_name = "resource-path")]
        resource_path: String,
    },
    /// List files on the device
    List {
        /// Path to list
        path: String,
    },
}

#[derive(Deserialize, Debug)]
struct Resource {
    filename: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct ObsoleteFile {
    path: String,
    #[allow(dead_code)]
    since: String,
}

#[derive(Deserialize, Debug)]
struct ResourceCollection {
    resources: Vec<Resource>,
    obsolete_files: Vec<ObsoleteFile>,
}

async fn flash_resources(client: &AdafruitFileTransferClient<BtleplugDevice>, resource_path: &str) -> Result<(), Error> {
    let f = std::fs::File::open(resource_path)?;
    let reader = std::io::BufReader::new(f);
    let mut zip_archive = ZipArchive::new(reader)?;
    if zip_archive.file_names().filter(|f| f == &"resources.json").count() == 0 {
        return Err(Error::new("No resources.json in archive"));
    }
    // Read resources in a block here so that the handle on zip_archive is dropped again and can be
    // borrowed again when we need to pull the resource files out.
    let resources: ResourceCollection = {
        let mut resources_spec = zip_archive.by_name("resources.json")?;
        let mut buf: String = Default::default();
        resources_spec.read_to_string(&mut buf)?;
        serde_json::from_str(&buf)?
    };

    for res in resources.resources {
        let path = Path::new(&res.path);
        let mut zipfile = zip_archive.by_name(&res.filename)?;
        let mut buf: Vec<u8> = vec![];
        zipfile.read_to_end(&mut buf)?;

        let bar_style = match indicatif::ProgressStyle::with_template("[{elapsed}] {bar:40} {bytes:>7}/{total_bytes:7} {msg}") {
            Ok(bar_style) => bar_style,
            Err(error) => return Err(Error::new(&format!("Error setting progress bar style: {error}"))),
        };
        let bar = indicatif::ProgressBar::new(buf.len() as u64)
            .with_style(bar_style)
            .with_message(format!("Sending {}", res.filename))
            .with_finish(indicatif::ProgressFinish::WithMessage(format!("{} sent.", res.filename).into()));

        // For some reason InfiniTime has chosen not to comply with the Adafruit spec when it comes
        // to status codes, so some operations are erroneously reported as failures.
        // https://github.com/InfiniTimeOrg/InfiniTime/blob/develop/doc/BLEFS.md
        if let Err(error) = client.make_directory(
                path.parent().ok_or(Error::new("No parent found for {path}"))?
                .to_str().ok_or(Error::new("Unable to decode parent of {path} to string"))?).await {
            log::warn!("Error when trying to make directory: {error}. The could be due to InfiniTime not following the Adafruit spec completely with regards to status codes.");
        }
        let mut previous_pos = 0;
        client.write_file(&res.path, &buf, 128, |c| {
            let written = c.offset - previous_pos;
            previous_pos = c.offset;
            bar.inc(written.into());
        }).await?;
    }

    for obsolete in resources.obsolete_files {
        client.delete_file_or_directory(&obsolete.path).await?;
    }

    Ok(())
}

async fn cli() -> Result<(), Error> {
    let args = Args::parse();
    let client = AdafruitFileTransferClient::<BtleplugDevice>::new_from_device_name("InfiniTime").await?;
    match args.command {
        Command::Flash { resource_path } => flash_resources(&client, &resource_path).await?,
        Command::List { path } => {
            let files = client.list_directory(&path).await?;
            for file in files {
                println!("{}", file.path.unwrap_or("".into()));
            }
        },
    }
    client.disconnect().await?;
    Ok(())
}

// To use this method in the `or_else` call, its signature must be FnOnce(Error) -> Result
fn exit_with_message(error: Error) -> Result<(), ()> {
    eprintln!("{}", error);
    std::process::exit(1);
}

// https://users.rust-lang.org/t/exiting-gracefully-instead-of-panic/3758
#[tokio::main]
async fn main() {
    if let Err(error) = env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or("warn")).try_init() {
        eprintln!("Error setting logger: {error}. Continuing regardless.");
    }
    cli().await.or_else(exit_with_message).expect("Unexpected error when trying to exit");
}
