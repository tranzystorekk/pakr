use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wrapper {
    pub command: String,
    pub requires_root: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub wrapper: Wrapper,
}

impl Default for Wrapper {
    fn default() -> Self {
        Self {
            command: "pacman".into(),
            requires_root: true,
        }
    }
}
