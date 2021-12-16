use std::fs;
mod structs;
mod download;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let config: structs::config::Config = toml::from_str(
        fs::read_to_string("config.toml")
            .expect("Failed to read config.toml")
            .as_str()
    ).expect("Failed to parse config.toml");

    let minecraft_version = &config.minecraft.version;
    let blocking = reqwest::blocking::Client::new();
    let total_bar = ProgressBar::new(config.fabric.mods.len() as u64);

    for modid in &config.fabric.mods {
        let response = &blocking.get(format!("https://api.modrinth.com/api/v1/mod/{}", modid))
            .send().expect("Failed to send request")
            .text().expect("Failed to get response");

        let data = match serde_json::from_str::<structs::modrinth_mod::Root>(response.as_str()) {
            Ok(data) => data,
            Err(e) => {
                println!("Failed to parse response: {}, Skipping mod {}...", e, modid);
                continue;
            }
        };

        let mut counter = 1;
        let total = data.versions.len() as u64;
        let mut downloads = Vec::new();
        let version_bar = ProgressBar::new_spinner();
        let style = ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {wide_msg}");
        version_bar.set_style(style);

        for version in &data.versions {
            let response = &blocking.get(format!("https://api.modrinth.com/api/v1/version/{}", version))
                .send().expect("Failed to send request")
                .text().expect("Failed to get response");

            let data = match serde_json::from_str::<structs::modrinth_version::Root>(response.as_str()) {
                Ok(data) => data,
                Err(e) => {
                    println!("Failed to parse response: {}, Skipping version {}...", e, version);
                    continue;
                }
            };

            if data.game_versions.contains(&minecraft_version) {
                let download_url = data.files[0].url.clone();
                let _ = downloads.push(download_url);
            }
            counter += 1;

            version_bar.set_message(format!("[{}] Checking {} out of {}", &modid, &counter, &total));
            version_bar.tick();
        }
        version_bar.finish();

        let download_url = match downloads.pop() {
            Some(str) => str,
            None => {
                println!("Cannot find mod {} for {}", &modid, &minecraft_version);
                continue;
            }
        };

        match download::download_file_blocking(&blocking, &download_url) {
            Ok(_) => println!("Downloaded {}.", &modid),
            Err(e) => println!("Failed to download {}: {}", &modid, e)
        }
        total_bar.inc(1);
    }
    total_bar.finish();
}
