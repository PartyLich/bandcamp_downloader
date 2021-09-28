//! User configurable application settings
use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::core::tag::EditAction;
use crate::{helper, Result};

/// UI localization option
#[derive(Debug, Copy, Clone)]
pub enum Language {
    /// English (US)
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

/// User configurable application settings
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct UserSettings {
    /// Allowed difference between expected filesize and actual size on disk
    pub allowed_file_size_difference: f32,

    /// Check for newer releases on startup
    pub check_for_updates: bool,

    /// Download entire artist discography
    pub download_artist_discography: bool,
    /// Maximum number of download attempts
    pub download_max_tries: u32,
    /// If true, download albums serially; concurrent download otherwise.
    pub download_one_album_at_a_time: bool,
    /// Base path for album downloads
    pub downloads_path: PathBuf,

    /// Time in seconds between retries
    pub download_retry_cooldown: f64,

    /// Format for audio file names
    pub file_name_format: String,

    // id3 tagging
    /// Modify id3 tags for downloaded tracks
    pub modify_tags: bool,
    /// Action to apply if modifying id3 Album Artist field
    pub tag_album_artist: EditAction,
    /// Action to apply if modifying id3 Album Title field
    pub tag_album_title: EditAction,
    /// Action to apply if modifying id3 Artist field
    pub tag_artist: EditAction,
    /// Action to apply if modifying id3 Comments field
    pub tag_comments: EditAction,
    /// Action to apply if modifying id3 Lyrics field
    pub tag_lyrics: EditAction,
    /// Action to apply if modifying id3 Track Number field
    pub tag_track_number: EditAction,
    /// Action to apply if modifying id3 Track Title field
    pub tag_track_title: EditAction,
    /// Action to apply if modifying id3 Date fields
    pub tag_year: EditAction,

    // playlist settings
    /// Create playlists for downloaded albums
    pub create_playlist: bool,
    /// File format to write playlists
    pub playlist_format: PlaylistFormat,
    /// Format for playlist file names
    pub playlist_file_name_format: String,

    /// Fetch file sizes before (ie. separate request) downloading files
    pub retrieve_files_size: bool,

    // Cover Art
    /// Format for cover art file names
    pub cover_art_file_name_format: String,
    /// Save album cover art in the album directory
    pub save_cover_art_in_folder: bool,
    /// Save album cover art in the id3 tag
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

            create_playlist: false,
            playlist_format: PlaylistFormat::M3U,
            playlist_file_name_format: String::from("{album}"),

            downloads_path,
            download_artist_discography: false,
            download_one_album_at_a_time: false,
            download_max_tries: 7,
            download_retry_cooldown: 0.2,

            retrieve_files_size: true,

            cover_art_file_name_format: String::from("{album}"),
            save_cover_art_in_folder: false,
            save_cover_art_in_tags: true,

            modify_tags: true,
            tag_album_artist: EditAction::Modify,
            tag_album_title: EditAction::Modify,
            tag_artist: EditAction::Modify,
            tag_comments: EditAction::Empty,
            tag_lyrics: EditAction::Modify,
            tag_track_number: EditAction::Modify,
            tag_track_title: EditAction::Modify,
            tag_year: EditAction::Modify,

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
