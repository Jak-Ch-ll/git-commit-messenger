use inquire::Select;

use crate::config::SelectOption;

const CONVENTIONAL_TYPE_JSON: &str = include_str!("../data/conventional_types.json");

pub fn select_conventional_type() -> String {
    let conventional_types: Vec<SelectOption> =
        serde_json::from_str(CONVENTIONAL_TYPE_JSON).expect("Error parsing CONVENTIONAL_TYPE_JSON");
    let selected_type = Select::new("Type of commit:", conventional_types)
        .prompt()
        .unwrap();

    selected_type.key
}
