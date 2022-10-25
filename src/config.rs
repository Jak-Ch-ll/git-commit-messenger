use std::{fmt::Display, fs, path::PathBuf};

use home::home_dir;

use serde::Deserialize;

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

const _TEST_YML: &str = "\
- type: ConventionalType
- type: Literal
  value: ' ('
- type: Select
  prompt: 'Scope: '
  options:
    - key: fileui5
    - key: listui5
- type: Literal
  value: '): '
- type: Gitmoji
- type: Space
- type: TextInput
  prompt: 'Description: '
";

pub struct Config {
    config: ConfigOptions,
}

impl Config {
    pub fn new() -> Self {
        let mut config_path = PathBuf::from(home_dir().unwrap());
        config_path.push(".config/gcm/config.yml");
        let config_yml = fs::read_to_string(config_path).unwrap();
        let config = serde_yaml::from_str(&config_yml).unwrap();

        Config { config }
    }

    pub fn variants(&self) -> &Vec<Variant> {
        &self.config.variants
    }
}
