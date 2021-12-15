use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub slug: String,
    pub team: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "body_url")]
    pub body_url: Value,
    pub published: String,
    pub updated: String,
    pub status: String,
    pub license: License,
    #[serde(rename = "client_side")]
    pub client_side: String,
    #[serde(rename = "server_side")]
    pub server_side: String,
    pub downloads: i64,
    pub followers: i64,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    #[serde(rename = "wiki_url")]
    pub wiki_url: Value,
    #[serde(rename = "discord_url")]
    pub discord_url: String,
    #[serde(rename = "donation_urls")]
    pub donation_urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: String,
}
