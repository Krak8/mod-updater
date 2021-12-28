use std::fs;
use std::fs::File;
use std::io;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;
use std::sync::{Arc, Mutex};

pub fn download(config_path: &str, output_path: &str, client: Arc<Client>) {
    let config: super::structs::config::Root = toml::from_str(
        fs::read_to_string(config_path)
            .expect("Failed to read config.toml")
            .as_str(),
    )
    .expect("Failed to parse config.toml");

    let minecraft_version = &config.minecraft.version;

    let _ = &config.fabric.mods.par_iter().for_each(move |modid| {
        let cloned_client = client.clone();
        let res = cloned_client
            .get(&format!("https://api.modrinth.com/api/v1/mod/{}", modid))
            .send()
            .expect("Failed to send request")
            .text()
            .expect("Failed to get response");

        let data = match serde_json::from_str::<super::structs::modrinth_mod::Root>(res.as_str()) {
            Ok(data) => data,
            Err(_) => return,
        };

        let downloads = Arc::new(Mutex::new(Vec::new()));

        let _ = &data.versions.par_iter().for_each(|version| {
            let res = cloned_client
                .get(format!(
                    "https://api.modrinth.com/api/v1/version/{}",
                    version
                ))
                .send()
                .expect("Failed to send request")
                .text()
                .expect("Failed to get response");

            let data = match serde_json::from_str::<super::structs::modrinth_version::Root>(
                res.as_str(),
            ) {
                Ok(data) => data,
                Err(_) => return,
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
                println!("No download found for {}.", modid);
                return;
            }
        };

        if !fs::metadata(output_path).is_ok() {
            let _ = fs::create_dir_all(output_path);
        }

        match download_file_blocking(cloned_client, &download_url, output_path) {
            Ok(_) => println!("Downloaded {}.", &modid),
            Err(e) => println!("Failed to download {}: {}", &modid, e),
        }
    });
}

fn download_file_blocking(client: Arc<Client>, url: &str, output_path: &str) -> io::Result<()> {
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let mut file = match File::create(format!(
        "{}/{}",
        output_path,
        match url.split("/").last() {
            Some(file) => file,
            None => return Err(io::Error::new(io::ErrorKind::Other, "No file name found")),
        }
    )) {
        Ok(file) => file,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    io::copy(&mut response, &mut file).expect("Failed to download file");
    Ok(())
}
