use serde::{Deserialize, Serialize};

#[derive(Hash, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub url: String,
    pub title: String
}

impl Link {
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
}
