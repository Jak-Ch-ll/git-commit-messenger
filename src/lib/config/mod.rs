use home::home_dir;
use serde::Deserialize;
use std::{fmt::Display, path::PathBuf};

use self::generate::{default_config, read_from_path};

mod generate;

#[derive(Deserialize)]
pub struct ConfigOptions {
    pub variants: Vec<Variant>,
}

#[derive(Deserialize, Clone)]
pub struct Variant {
    name: String,
    pub parts: Vec<Part>,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Part {
    Select {
        prompt: String,
        options: Vec<SelectOption>,
        pattern: Option<String>,
    },
    ConventionalType,
    Gitmoji,
    Space,
    Literal {
        value: String,
    },
    TextInput {
        prompt: String,
        pattern: Option<String>,
    },
}

#[derive(Deserialize, Clone)]
pub struct SelectOption {
    pub key: String,
    pub description: Option<String>,
}

pub struct Config {
    config: ConfigOptions,
}

impl Config {
    pub fn new() -> Self {
        let mut config_path = PathBuf::from(home_dir().unwrap());
        config_path.push(".config/gcm/config.yml");

        if let Some(config) = read_from_path(config_path) {
            Config { config }
        } else {
            Config {
                config: default_config(),
            }
        }
    }

    pub fn variants(&self) -> &Vec<Variant> {
        &self.config.variants
    }
}
