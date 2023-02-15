use gcm::{
    config::{Config, Part},
    error::Error,
    git,
    prompt::message_builder::MessageBuilder,
};
use inquire::{Confirm, Select};

#[termination::display]
fn main() -> Result<(), Error> {
    git::check_ready()?;

    let config = Config::new();

    let selected_variant = Select::new("Select variant: ", config.variants().to_vec())
        .prompt()
        .unwrap();

    let message = prompt_user(&selected_variant.parts);

    println!("\n{message}\n");

    let confirmed = Confirm::new("Commit with the given message?")
        .with_default(true)
        .prompt()
        .unwrap();

    if confirmed {
        git::commit(message);
    }

    return Ok(());
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
