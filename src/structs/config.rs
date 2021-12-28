use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub minecraft: Minecraft,
    pub fabric: Fabric,
}

#[derive(Serialize, Deserialize)]
pub struct Minecraft {
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fabric {
    pub mods: Vec<String>,
}
