use std::{env, fs};
use std::io::{Read, Write};
use super::structs::*;

pub fn scan_to_file() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let directory = fs::read_dir(current_dir).expect("Failed to read directory");
    let mut config = config::Root {
        minecraft: config::Minecraft { version: "EDIT_THIS".to_string() },
        fabric: config::Fabric { mods: vec![] }
    };
    for item in directory {
        if item.is_ok() {
            let item = match item {
                Ok(item) => item,
                Err(_) => continue
            };
            let item_name = item.file_name();
            let item_name_str = match item_name.to_str() {
                Some(item_name) => item_name,
                None => continue
            };
            if item_name_str.ends_with(".jar") {
                let item_path = item.path();
                let item_path_str = match item_path.to_str() {
                    Some(item_path) => item_path,
                    None => continue
                };
                let extracted_resources = match extract_resources(item_path_str) {
                    Some(extracted_resources) => extracted_resources,
                    None => continue
                };
                let data = match serde_json::from_str::<fabric_json::Root>(extracted_resources.as_str()) {
                    Ok(data) => data,
                    Err(_) => continue
                };
                if fetch_not_404(format!("https://api.modrinth.com/api/v1/mod/{}", data.id).as_str()) {
                    println!("Added {}.", data.id);
                    config.fabric.mods.push(data.id);
                }
            }
        } else {
            continue;
        }
    }
    let mut scanned_config = fs::File::create("scanned_config.toml").expect("Failed to create file.");
    scanned_config.write_all(toml::to_string(&config).unwrap().as_bytes()).unwrap();
}

fn fetch_not_404(url: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).send().unwrap();
    if res.status().is_success() {
        return true;
    }
    false
}

fn extract_resources(file: &str) -> Option<String> {
    let path = std::path::Path::new(&file);
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => return None
    };
    let mut archive = match zip::ZipArchive::new(&file) {
        Ok(archive) => archive,
        Err(_) => return None
    };

    let mut resources = match archive.by_name("fabric.mod.json") {
        Ok(resources) => resources,
        Err(_) => return None
    };

    let mut contents = String::new();
    return match resources.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(_) => None
    }
}