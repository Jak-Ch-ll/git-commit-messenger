use std::io::{self, Write};
use std::process::Command;

mod config;
mod prompt;
use config::{Config, Part};
use inquire::{Confirm, Select};

use crate::prompt::message_builder::MessageBuilder;

macro_rules! _option {
    ($key:literal) => {
        SelectOption {
            key: String::from($key),
            description: None,
        }
    };
    ($key:literal, $description:literal ) => {
        SelectOption {
            key: String::from($key),
            description: Some(String::from($description)),
        }
    };
}

fn main() {
    let config = Config::new();

    let selected_variant = Select::new("Select variant: ", config.variants().to_vec())
        .prompt()
        .unwrap();

    let message = prompt_user(&selected_variant.parts);
    println!("{message}");

    //let confirmed = Confirm::new(&format!(
    //    "Commit with with the following message?\n{}",
    //    message
    //))
    let confirmed = Confirm::new("Commit with with the following message?")
        .with_default(true)
        .with_help_message(&message)
        .prompt()
        .unwrap();

    if confirmed {
        git_commit(message);
    }
}

fn prompt_user(parts: &[Part]) -> String {
    let mut message_builder = MessageBuilder::new();

    for part in parts {
        match part {
            Part::Select {
                prompt,
                options,
                pattern,
            } => message_builder.select_prompt(&prompt, options, pattern),
            Part::Space => message_builder.add_literal(" "),
            Part::Gitmoji => message_builder.select_gitmoji(),
            Part::Literal { value } => message_builder.add_literal(&value),
            Part::TextInput { prompt, pattern } => message_builder.text_input(&prompt, &pattern),
            Part::ConventionalType => message_builder.select_conventional_type(),
        };
    }

    message_builder.consume()
}

fn git_commit(message: String) {
    let stdout = duct::cmd!("git", "commit", "-m", message).read().unwrap();
    println!("{stdout}");
}

//fn git_commit(message: String) {
//    Command::new("git")
//        .current_dir("./testrepo")
//        .args(["commit", "-e", "-m", &message]).stdout(Stdio::piped()).spawn()
//        .output()
//        .unwrap();
//}

fn _reading_git() {
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("Failed to run git");

    io::stdout()
        .write_all(&output.stdout)
        .expect("Failed to write");
}

/*
use yaml_rust::{Yaml, YamlLoader};
    let yml = "
Projects:
    - Name: aconso-ux
      Steps:
          - type
          - scope
          - gitmoji
          - description
    - Name: Admin-Center
      Steps:
          - type
          - scope
          - gitmoji
          - description
";

    let docs = YamlLoader::load_from_str(yml).unwrap();
    let doc = &docs[0];
    doc.as_arr
    println!("{:?}", doc);
 */
