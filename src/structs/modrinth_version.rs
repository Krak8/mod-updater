use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    #[serde(rename = "mod_id")]
    pub mod_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: String,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Value,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: i64,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub files: Vec<File>,
    pub dependencies: Vec<Value>,
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha512: Option<String>,
    pub sha1: Option<String>,
}
