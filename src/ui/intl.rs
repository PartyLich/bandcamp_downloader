use serde::Deserialize;
use std::fs;

use crate::{helper::get_root_dir, settings::Language};

/// Display strings used in the user interface
#[derive(Debug, Deserialize)]
pub struct IntlString {
    pub title: String,
    pub settings_title: String,
    pub save_caption: String,
    pub discography_checkbox: String,
    pub log_placeholder: String,
    pub download_button_text: String,
    pub settings_button_text: String,
    pub cancel_button_text: String,
    pub save_input_placeholder: String,
    pub urls_placeholder: String,
    pub delete_button: String,
    pub modify_tags_checkbox: String,
}

impl IntlString {
    pub fn new(language: Language) -> Self {
        const EXTENSION: &str = ".json";
        const DIR: &str = "intl/";
        let file_name = match language {
            Language::EN => "en",
        };
        let path = [DIR, file_name, EXTENSION].concat();

        Self::load_strings(&path)
    }

    fn load_strings(file_path: &str) -> Self {
        let mut path = get_root_dir();
        path.push(file_path);
        let display = path.display();

        // read the contents into a string
        let s = fs::read_to_string(&path)
            .unwrap_or_else(|why| panic!("couldnt open {}: {}", display, why));

        serde_json::from_str(&s).unwrap_or_else(|why| panic!("Deserialization error: {}", why))
    }
}

impl Default for IntlString {
    fn default() -> Self {
        Self::new(Language::EN)
    }
}
