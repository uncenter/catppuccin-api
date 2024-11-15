use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::shared::{Collaborator, StringOrStrings};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub collaborators: Vec<Collaborator>,
    pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserstylesRoot {
    pub userstyles: HashMap<String, Userstyle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Userstyle {
    pub name: StringOrStrings,
    pub categories: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub color: String,
    pub readme: Readme,
    pub current_maintainers: Vec<Collaborator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_maintainers: Option<Vec<Collaborator>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Readme {
    pub app_link: StringOrStrings,
}
