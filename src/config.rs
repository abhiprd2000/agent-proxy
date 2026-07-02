use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub deny: Option<Vec<String>>,
    pub review: Option<Vec<String>>,
}

impl Config {
    pub fn load() -> Self {
        let content = fs::read_to_string("agentproxy.yml");

        match content {
            Ok(text) => {
                serde_yaml::from_str(&text).unwrap_or(Self {
                    deny: None,
                    review: None,
                })
            }
            Err(_) => Self {
                deny: None,
                review: None,
            },
        }
    }
}