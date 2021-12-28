use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Root {
    pub id: String,
}
