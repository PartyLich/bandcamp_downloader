use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{helper, Result};

#[derive(Debug, Copy, Clone)]
pub enum Language {
    EN,
}

/// Available playlist export formats
#[derive(Debug, Deserialize, Copy, Clone, Serialize)]
pub enum PlaylistFormat {
    /// MP3 url format
    M3U,
    /// PLS multimedia playlist format
    PLS,
}

impl PlaylistFormat {
    pub const ALL: [PlaylistFormat; 2] = [Self::M3U, Self::PLS];

    pub fn value(&self) -> &str {
        match self {
            Self::M3U => "m3u",
            Self::PLS => "pls",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::M3U => "(MP3 url)",
            Self::PLS => "(PLS multimedia playlist)",
        }
    }
}

impl std::fmt::Display for PlaylistFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value(), self.description())
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserSettings {
    pub allowed_file_size_difference: f32,

    pub check_for_updates: bool,

    pub download_artist_discography: bool,

    pub download_max_tries: u32,
    pub download_one_album_at_a_time: bool,
    pub downloads_path: PathBuf,

    /// Time in seconds between retries
    pub download_retry_cooldown: f64,

    pub file_name_format: String,

    pub modify_tags: bool,

    // playlist settings
    pub create_playlist: bool,
    pub playlist_format: PlaylistFormat,
    pub playlist_file_name_format: String,

    pub retrieve_files_size: bool,

    // Cover Art
    pub cover_art_file_name_format: String,
    pub save_cover_art_in_folder: bool,
    pub save_cover_art_in_tags: bool,

    pub show_verbose_log: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        let mut downloads_path = dirs::audio_dir().unwrap_or_else(|| {
            let mut home_dir = dirs::home_dir().unwrap();
            home_dir.push("music");
            home_dir
        });
        downloads_path.push("{artist}");
        downloads_path.push("{year} - {album}");

        Self {
            check_for_updates: true,

            allowed_file_size_difference: 0.05,

            file_name_format: String::from("{tracknum} {artist} - {title}.mp3"),
            cover_art_file_name_format: String::from("{album}"),

            create_playlist: false,
            playlist_format: PlaylistFormat::M3U,
            playlist_file_name_format: String::from("{album}"),

            download_artist_discography: false,
            download_one_album_at_a_time: false,
            downloads_path,
            download_max_tries: 7,
            download_retry_cooldown: 0.2,

            retrieve_files_size: true,

            save_cover_art_in_folder: false,
            save_cover_art_in_tags: true,
            modify_tags: true,

            show_verbose_log: false,
        }
    }
}

impl UserSettings {
    const SETTINGS_FILE: &'static str = "user_settings.json";

    /// Attempt to load user settings from the filesystem
    pub fn load() -> Result<UserSettings> {
        let mut path = helper::get_root_dir();
        path.push(Self::SETTINGS_FILE);

        let settings = fs::read_to_string(&path)?;
        serde_json::from_str(&settings).map_err(From::from)
    }

    /// Attempt to save user settings to the filesystem
    pub fn save(&self) -> Result<()> {
        let mut path = helper::get_root_dir();
        path.push(Self::SETTINGS_FILE);

        let settings = serde_json::to_string_pretty(self)?;
        // create or overwrite settings file
        fs::write(path, settings).map_err(From::from)
    }
}
