use std::fs;
use std::env;
use clap::Parser;


mod structs;
mod download;
mod scanner;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to the config file
    #[clap(short, long)]
    config_path: String,

    /// The path to the directory where the files will be downloaded
    #[clap(short, long)]
    output_path: String,

    /// Scan the directory for mods and update them
    #[clap(short, long)]
    scan: bool,
}

fn main() {
    let args: Args = Args::parse();

    if args.scan {
        println!("Scanning for mods...");
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let directory = fs::read_dir(current_dir).expect("Failed to read directory");
        for item in directory {
            if item.is_ok() {
                let item = item.unwrap();
                if item.file_name().to_str().unwrap().ends_with(".jar") {
                    println!("{:#?}", scanner::extract_resources(item.path().to_str().unwrap()));
                }
            } else {
                continue;
            }
        }
        return
    } else {
        download::download(args.config_path.as_str(), args.output_path.as_str());
    }
}
