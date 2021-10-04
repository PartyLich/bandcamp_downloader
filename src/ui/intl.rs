use serde::Deserialize;
use std::fs;

use crate::{helper::get_root_dir, settings::Language};

/// Display strings used for localization in the user interface
#[derive(Debug, Deserialize)]
pub struct IntlString {
    /// Main view window title
    pub title: String,
    /// Settings view window title
    pub settings_title: String,
    pub save_caption: String,
    /// Discography download toggle label
    pub discography_checkbox: String,
    /// Main view event log placeholder text
    pub log_placeholder: String,
    /// Start downloads button label
    pub download_button_text: String,
    /// Open settings view button label
    pub settings_button_text: String,
    /// Open main view button label
    pub main_button_text: String,
    /// Save settings button label
    pub save_settings_button: String,
    /// Cancel settings changes button label
    pub cancel_button_text: String,
    /// URL list placeholder text
    pub urls_placeholder: String,
    /// URL list item delete button label
    pub delete_button: String,
    /// Filename format input box label
    pub filename_format: String,
    /// Filename format placeholder text
    pub save_input_placeholder: String,
    /// Modify id3 tags toggle label
    pub modify_tags_checkbox: String,
    /// Cover art settings view title
    pub cover_art: String,
    /// Cover art in folder toggle label
    pub art_in_folder: String,
    /// Cover art in tags toggle label
    pub art_in_tags: String,
    /// General settings view title
    pub general: String,
    /// Naming and Tags settings view title
    pub naming_and_tags: String,
    /// Language label
    pub language: String,
    /// Theme label
    pub theme: String,

    pub album_artist: String,
    pub album_title: String,
    pub artist: String,
    pub comments: String,
    pub lyrics: String,
    pub track_number: String,
    pub track_title: String,
    pub album_date: String,
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
