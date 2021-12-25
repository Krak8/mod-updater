use std::{env, fs};
use std::io::Read;

pub fn scan_to_file() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let directory = fs::read_dir(current_dir).expect("Failed to read directory");
    for item in directory {
        if item.is_ok() {
            let item = item.unwrap();
            if item.file_name().to_str().unwrap().ends_with(".jar") {
                let json = serde_json::from_str::<super::structs::fabric_json::Root>(extract_resources(item.path().to_str().unwrap()).as_str()).unwrap();
                println!("{:#?}", json);
            }
        } else {
            continue;
        }
    }
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