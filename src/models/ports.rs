use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shared::{Collaborator, SingleOrMultiple};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub collaborators: Vec<Collaborator>,
    pub ports: HashMap<String, Port>,
    pub categories: Vec<Category>,
    pub showcases: Vec<Showcase>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Port {
    pub name: String,
    pub categories: Vec<String>,
    pub upstreamed: Option<bool>,
    pub platform: SingleOrMultiple<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub links: Option<Vec<Link>>,
    pub icon: Option<String>,
    pub color: String,
    pub alias: Option<String>,
    pub current_maintainers: Vec<Collaborator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_maintainers: Option<Vec<Collaborator>>,

    #[serde(default)]
    pub is_userstyle: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    #[serde(skip_serializing)]
    pub key: String,
    pub name: String,
    pub description: String,
    pub emoji: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Showcase {
    title: String,
    description: String,
    link: String,
}
