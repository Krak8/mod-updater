use std::fs;
use std::fs::File;
use std::io;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reqwest::blocking::Client;
use std::sync::{Arc, Mutex};

use super::structs::config::Config;

pub fn download(config: Config, client: Arc<Client>) {
    let output_path = &config.download.output_path;
    let minecraft_version = &config.minecraft.version;
    let fabric_mods = config.fabric.mods;

    let progress_bar = ProgressBar::new(fabric_mods.len() as u64);
    let progress_style = ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    );
    progress_bar.set_style(progress_style);

    let missed_mods: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    let _ = &fabric_mods
        .par_iter()
        .progress_with(progress_bar)
        .for_each(|modid| {
            let cloned_client = client.clone();
            let res = cloned_client
                .get(&format!("https://api.modrinth.com/api/v1/mod/{}", modid))
                .send()
                .expect("Failed to send request")
                .text()
                .expect("Failed to get response");

            let data =
                match serde_json::from_str::<super::structs::modrinth_mod::Root>(res.as_str()) {
                    Ok(data) => data,
                    Err(_) => {
                        let _ = &missed_mods.clone().lock().unwrap().push(modid.to_string());
                        return
                    },
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
                    Err(_) => {
                        return
                    },
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
                    let _ = &missed_mods.clone().lock().unwrap().push(modid.to_string());
                    return;
                }
            };

            if !fs::metadata(output_path).is_ok() {
                let _ = fs::create_dir_all(output_path);
            }

            match download_file_blocking(cloned_client, &download_url, output_path) {
                Ok(_) => return,
                Err(_) => {
                    let _ = &missed_mods.clone().lock().unwrap().push(modid.to_string());
                    return
                },
            }
        });
        println!("Could not download these mods: {:#?}", &missed_mods.clone().lock().unwrap())
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
