use std::{fs, path::PathBuf};

use super::{ConfigOptions, Part, Variant};

pub fn read_from_path(path: PathBuf) -> Option<ConfigOptions> {
    let config_yml = fs::read_to_string(path).ok()?;
    let config = serde_yaml::from_str(&config_yml).ok()?;

    config
}

pub fn default_config() -> ConfigOptions {
    ConfigOptions {
        variants: vec![
            Variant {
                name: String::from("Conventional Commits"),
                parts: vec![
                    Part::ConventionalType,
                    Part::TextInput {
                        prompt: String::from("Scope"),
                        pattern: Some(String::from(" ({0})")),
                    },
                    Part::Literal {
                        value: String::from(": "),
                    },
                    Part::TextInput {
                        prompt: String::from("Description"),
                        pattern: None,
                    },
                ],
            },
            Variant {
                name: String::from("Gitmoji"),
                parts: vec![
                    Part::Gitmoji,
                    Part::Space,
                    Part::TextInput {
                        prompt: String::from("Description"),
                        pattern: None,
                    },
                ],
            },
            Variant {
                name: String::from("CC + Gitmoji"),
                parts: vec![
                    Part::ConventionalType,
                    Part::TextInput {
                        prompt: String::from("Scope"),
                        pattern: Some(String::from(" ({0})")),
                    },
                    Part::Literal {
                        value: String::from(": "),
                    },
                    Part::Gitmoji,
                    Part::Space,
                    Part::TextInput {
                        prompt: String::from("Description"),
                        pattern: None,
                    },
                ],
            },
        ],
    }
}
