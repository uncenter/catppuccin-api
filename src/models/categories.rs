use serde::{Deserialize, Serialize};

pub type Root = Vec<Category>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    #[serde(skip_serializing)]
    pub key: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
}
