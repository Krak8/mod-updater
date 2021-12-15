use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub minecraft: Minecraft,
    pub fabric: Fabric
}

#[derive(Deserialize)]
pub struct Minecraft {
    pub version: String
}

#[derive(Deserialize)]
pub struct Fabric {
    pub mods: Vec<String>
}