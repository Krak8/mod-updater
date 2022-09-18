use clap::Parser;
use std::{sync::Arc, fs};

use crate::structs::config::Config;

mod download;
mod scanner;
mod structs;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to the config file
    #[clap(long, default_value = "config.toml")]
    config_path: String,

    /// Scan the directory for mods and update them
    #[clap(long)]
    scan: bool,

    /// The output config file for scanning
    #[clap(long, default_value = "scanned_config.toml")]
    scan_output: String,
}

fn main() {
    let args: Args = Args::parse();
    let reqwest = Arc::new(reqwest::blocking::Client::new());
    if args.scan {
        scanner::scan_to_file(reqwest, &args.scan_output);
        println!("Saved config file to {}! Manually add any missing mods.", &args.scan_output);
        return;
    }

    let config: Config = toml::from_str(
        fs::read_to_string(args.config_path.as_str())
            .expect("Failed to read config.toml")
            .as_str(),
    ).expect("Failed to parse config.toml");

    download::download(config, reqwest);

    println!("Downloaded all the mods to the output folder! Manually add any missing mods.");
    return;
}
