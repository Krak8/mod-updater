use std::io::Read;

pub fn extract_resources(file: &str) -> String {
    let path = std::path::Path::new(&file);
    let file = std::fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(&file).unwrap();

    let mut resources = archive.by_name("fabric.mod.json").unwrap();

    let mut contents = String::new();
    resources.read_to_string(&mut contents).unwrap();
    contents
}