use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "config.logos.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub locations: HashMap<String, Info>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Info {
    pub role: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            locations: HashMap::new()
        }
    }
}
