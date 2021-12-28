use clap::Parser;
use std::sync::Arc;

mod download;
mod scanner;
mod structs;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to the config file
    #[clap(long, default_value = "config.toml")]
    config_path: String,

    /// The path to the directory where the files will be downloaded
    #[clap(long, default_value = "mods")]
    output_path: String,

    /// Scan the directory for mods and update them
    #[clap(long)]
    scan: bool,

    /// The output config file for scanning
    #[clap(long), default_value = "scanned_config.toml"]
    scan_output: String,
}

fn main() {
    let args: Args = Args::parse();
    let reqwest = Arc::new(reqwest::blocking::Client::new());
    if args.scan {
        scanner::scan_to_file(reqwest, &args.scan_output);
        println!("Saved config file to config.toml! Manually add any missing mods.");
        return;
    }

    download::download(
        args.config_path.as_str(),
        args.output_path.as_str(),
        reqwest,
    );
    println!(
        "Downloaded all the mods to {} folder! Manually add any missing mods.",
        args.output_path
    );
    return;
}
