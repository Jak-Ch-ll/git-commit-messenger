use std::fmt::Display;

use crate::config::SelectOption;

use super::{conventional_type::select_conventional_type, gitmoji::select_gitmoji};

use inquire::{Select, Text};

impl Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.description {
            Some(d) => write!(f, "{} - {}", self.key, d),
            None => write!(f, "{}", self.key),
        }
    }
}

pub struct MessageBuilder {
    message: String,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        MessageBuilder {
            message: String::from(""),
        }
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn consume(self) -> String {
        self.message
    }

    pub fn select_prompt(
        &mut self,
        prompt: &str,
        select_options: &[SelectOption],
        pattern: &Option<String>,
    ) -> &Self {
        let select = Select::new(prompt, select_options.to_vec())
            .prompt()
            .unwrap();

        let text = if select.key != "None" {
            MessageBuilder::replace_pattern(pattern, select.key)
        } else {
            String::from("")
        };

        self.add_text(&text)
    }

    pub fn select_gitmoji(&mut self) -> &Self {
        let gitmoji = select_gitmoji();

        self.add_text(&gitmoji)
    }

    pub fn text_input(&mut self, prompt: &str, pattern: &Option<String>) -> &Self {
        let input = Text::new(prompt).prompt().unwrap();

        let input = if !input.is_empty() {
            MessageBuilder::replace_pattern(pattern, input)
        } else {
            input
        };

        self.add_text(&input)
    }

    pub fn add_literal(&mut self, literal: &str) -> &Self {
        self.add_text(literal)
    }

    pub fn select_conventional_type(&mut self) -> &Self {
        let text = select_conventional_type();

        self.add_text(&text)
    }

    fn add_text(&mut self, text: &str) -> &Self {
        self.message.push_str(text);

        self
    }

    fn replace_pattern(pattern: &Option<String>, text: String) -> String {
        match pattern {
            Some(p) => p.replace("{0}", &text),
            None => text,
        }
    }
}
