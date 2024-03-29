use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CONFIG_FILE_NAME: &str = "config.logos.toml";

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Config {
    pub locations: HashMap<String, Info>
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Info {
    pub role: String
}
