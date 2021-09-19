use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    #[serde(default)]
    pub checked: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            title,
            checked: false,
        }
    }
}
