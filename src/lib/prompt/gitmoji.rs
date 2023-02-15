use inquire::Select;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
struct Gitmoji {
    emoji: String,
    code: String,
    description: String,
}

const GITMOJI_JSON: &str = include_str!("../../data/gitmojis.json");

impl Display for Gitmoji {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.emoji, self.code, self.description)
    }
}

pub fn select_gitmoji() -> String {
    let gitmojis: Vec<Gitmoji> =
        serde_json::from_str(GITMOJI_JSON).expect("Error parsing GITMOJI_JSON");
    let selected_gitmoji = Select::new("Gitmoji:", gitmojis).prompt().unwrap();

    selected_gitmoji.emoji
}
