use serde::{Deserialize, Serialize};

use super::{ports::Port, userstyles::Userstyle};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SingleOrMultiple<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl<T: Default> Default for SingleOrMultiple<T> {
    fn default() -> Self {
        SingleOrMultiple::Single(T::default())
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
                SingleOrMultiple::<String>::Single(s) => s,
                SingleOrMultiple::<String>::Multiple(s) => s.join("/"),
            },
            categories: userstyle.categories,
            upstreamed: Some(false),
            platform: SingleOrMultiple::<String>::Single("agnostic".to_string()),
            url: Some(match userstyle.readme.app_link {
                SingleOrMultiple::<String>::Single(s) => s,
                SingleOrMultiple::<String>::Multiple(s) => s[0].clone(),
            }),
            links: None,
            icon: userstyle.icon,
            color: userstyle.color,
            alias: None,
            current_maintainers: userstyle.current_maintainers,
            past_maintainers: userstyle.past_maintainers,

            is_userstyle: true,
        }
    }
}
