use std::fs::File;
use std::io;

pub fn download_file_blocking(client: &reqwest::blocking::Client, url: &str) -> io::Result<()> {
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let mut file = match File::create(format!("./mods/{}", match url.split("/").last() {
        Some(file) => file,
        None => return Err(io::Error::new(io::ErrorKind::Other, "No file name found")),
    })) {
        Ok(file) => file,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    io::copy(&mut response, &mut file).expect("Failed to download file");
    Ok(())
}
