use clap::Parser;


mod structs;
mod download;
mod scanner;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// The path to the config file
    #[clap(short, long, default_value = "config.toml")]
    config_path: String,

    /// The path to the directory where the files will be downloaded
    #[clap(short, long, default_value = "mods")]
    output_path: String,

    /// Scan the directory for mods and update them
    #[clap(short, long)]
    scan: bool,
}

fn main() {
    let args: Args = Args::parse();

    if args.scan {
        scanner::scan_to_file();
        println!("Saved config file to config.toml! Manually add any missing mods.");
        return
    } else {
        download::download(args.config_path.as_str(), args.output_path.as_str());
    }
}
