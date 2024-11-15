use serde::{Deserialize, Serialize};

use super::{ports::Port, userstyles::Userstyle};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for StringOrStrings {
    fn default() -> Self {
        StringOrStrings::Single(String::new())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
    pub name: Option<String>,
    pub url: String,
}

impl From<Userstyle> for Port {
    fn from(userstyle: Userstyle) -> Self {
        Port {
            name: match userstyle.name {
                StringOrStrings::Single(s) => s,
                StringOrStrings::Multiple(s) => s.join("/"),
            },
            categories: userstyle.categories,
            upstreamed: Some(false),
            platform: StringOrStrings::Single("agnostic".to_string()),
            url: Some(match userstyle.readme.app_link {
                StringOrStrings::Single(s) => s,
                StringOrStrings::Multiple(s) => s[0].clone(),
            }),
            links: None,
            icon: userstyle.icon,
            color: userstyle.color,
            alias: None,
            current_maintainers: userstyle.current_maintainers,
            past_maintainers: userstyle.past_maintainers,

            is_userstyle: Some(true),
        }
    }
}
