use std::fs;
use std::sync::{Arc, Mutex};

mod structs;
mod download;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

fn main() {
    let config: structs::config::Config = toml::from_str(
        fs::read_to_string("config.toml")
            .expect("Failed to read config.toml")
            .as_str()
    ).expect("Failed to parse config.toml");

    let minecraft_version = &config.minecraft.version;
    let blocking = reqwest::blocking::Client::new();

    let _ = &config.fabric.mods.par_iter().for_each(move |modid| {
        let res = blocking.get(&format!("https://api.modrinth.com/api/v1/mod/{}", modid))
            .send().expect("Failed to send request")
            .text().expect("Failed to get response");

        let data = match serde_json::from_str::<structs::modrinth_mod::Root>(res.as_str()) {
            Ok(data) => data,
            Err(_) => return
        };

        let downloads = Arc::new(Mutex::new(Vec::new()));

        let _ = &data.versions.par_iter().for_each(|version| {
            let res = &blocking.get(format!("https://api.modrinth.com/api/v1/version/{}", version))
                .send().expect("Failed to send request")
                .text().expect("Failed to get response");

            let data = match serde_json::from_str::<structs::modrinth_version::Root>(res.as_str()) {
                Ok(data) => data,
                Err(_) => return
            };

            if data.game_versions.contains(&minecraft_version) {
                let download_url = data.files[0].url.clone();
                let _ = &downloads.clone().lock().unwrap().push(download_url);
            }
        });

        let download_clone = &downloads.clone();

        let download_url = match download_clone.lock().unwrap().pop() {
            Some(str) => str,
            None => {
                println!("No download found for {}", modid);
                return
            }
        };

        match download::download_file_blocking(&blocking, &download_url) {
            Ok(_) => println!("Downloaded {}.", &modid),
            Err(e) => println!("Failed to download {}: {}", &modid, e)
        }
    });
}
