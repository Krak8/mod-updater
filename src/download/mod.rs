use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn download_file_blocking(client: &reqwest::blocking::Client, url: &str, destination: &str) {
    if !Path::new(destination).exists() {
        fs::create_dir(destination).expect("Could not create directory");
    }
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = File::create(format!("./updatedMods/{}", url.split("/").last().unwrap())).expect("Failed to create file");
    io::copy(&mut response, &mut file).expect("Failed to download file");
}