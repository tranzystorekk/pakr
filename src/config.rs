use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "pakr.toml";

lazy_static! {
    static ref CONFIG_PATH: Option<PathBuf> = {
        dirs::config_dir().map(|mut path| {
            path.push(CONFIG_FILE_NAME);

            path
        })
    };
}

#[derive(Debug, Deserialize)]
pub struct Wrapper {
    pub command: String,
    pub requires_root: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub wrapper: Wrapper,
}

impl Config {
    pub fn from_file() -> Option<Self> {
        let config_path = CONFIG_PATH.as_ref()?;
        let content = std::fs::read_to_string(config_path).ok()?;

        toml::from_str(&content).ok()
    }
}

impl Default for Wrapper {
    fn default() -> Self {
        Self {
            command: "pacman".into(),
            requires_root: true,
        }
    }
}
