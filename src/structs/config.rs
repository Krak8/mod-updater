use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub minecraft: Minecraft,
    pub fabric: Fabric
}

pub struct Minecraft {
    pub version: String
}

pub struct Fabric {
    pub mods: Vec<String>
}