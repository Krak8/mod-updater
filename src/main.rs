use std::fs;
mod structs;
mod download;

fn main() {
    let config: structs::config::Config = toml::from_str(
        fs::read_to_string("config.toml")
            .expect("Failed to read config.toml")
            .as_str()
    ).expect("Failed to parse config.toml");

    let minecraft_version = &config.minecraft.version;
    let blocking = reqwest::blocking::Client::new();

    for modid in &config.fabric.mods {
        let response = &blocking.get(format!("https://api.modrinth.com/api/v1/mod/{}", modid))
            .send().expect("Failed to send request")
            .text().expect("Failed to get response");

        let data = serde_json::from_str::<structs::modrinth_mod::Root>(response.as_str())
            .expect("Failed to parse response");

        let mut counter = 1;
        let mut downloads = Vec::new();

        for version in &data.versions {
            println!("[{}]Checking {} out of {}", &modid, &counter, &data.versions.len());
            let response = &blocking.get(format!("https://api.modrinth.com/api/v1/version/{}", version))
                .send().expect("Failed to send request")
                .text().expect("Failed to get response");

            let data = serde_json::from_str::<structs::modrinth_version::Root>(response.as_str())
                .expect("Failed to parse response");

            if data.game_versions.contains(&minecraft_version) {
                println!("Mod {}, Versions: {:#?}, Download Link: {}", &modid, &data.game_versions, &data.files[0].url);
                let download_url = data.files[0].url.clone();
                let _ = downloads.push(download_url);
            }
            counter += 1;
        }

        let download_url = match downloads.pop() {
            Some(str) => str,
            None => {
                println!("Cannot find mod {} for {}", &modid, &minecraft_version);
                continue;
            }
        };

        download::download_file_blocking(&blocking, download_url.as_str(), "./updatedMods/");
    }
}
