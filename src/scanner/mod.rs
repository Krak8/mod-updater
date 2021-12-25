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
            let item = item.unwrap();
            if item.file_name().to_str().unwrap().ends_with(".jar") {
                let data = serde_json::from_str::<fabric_json::Root>(extract_resources(item.path().to_str().unwrap()).as_str()).unwrap();
                if fetch_not_404(format!("https://api.modrinth.com/api/v1/mod/{}", data.id).as_str()) {
                    config.fabric.mods.push(data.id);
                }
            }
        } else {
            continue;
        }
    }
    let mut scanned_config = fs::File::create("scanned_config.toml").expect("Failed to create file");
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

fn extract_resources(file: &str) -> String {
    let path = std::path::Path::new(&file);
    let file = std::fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(&file).unwrap();

    let mut resources = archive.by_name("fabric.mod.json").unwrap();

    let mut contents = String::new();
    resources.read_to_string(&mut contents).unwrap();
    contents
}