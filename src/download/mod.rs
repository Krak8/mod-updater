use std::fs::File;
use std::io;

pub fn download_file_blocking(client: &reqwest::blocking::Client, url: &str) -> io::Result<()> {
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = File::create(format!("./updatedMods/{}", url.split("/").last().unwrap())).expect("Failed to create file");
    io::copy(&mut response, &mut file).expect("Failed to download file");
    Ok(())
}