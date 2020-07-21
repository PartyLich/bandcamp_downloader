use std::path::PathBuf;

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use super::Track;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    /// The album artist.
    pub artist: String,

    /// The local path (full path with file name) where the artwork file should be saved.
    pub artwork_path: String,

    /// The local path (full path with file name) to the %TEMP% folder where the artwork file should be saved.
    pub artwork_temp_path: String,

    /// The URL where the artwork should be downloaded from.
    pub artwork_url: Option<String>,

    /// The local path (full path) to the folder where the album should be saved.
    pub path: String,

    /// The local path (full path with file name) where the playlist file should be saved.
    pub playlist_path: String,

    /// The release date of the album.
    pub release_date: DateTime<Utc>,

    /// The album title.
    pub title: String,

    /// The list of tracks contained in the album.
    pub tracks: Vec<Track>,
}

impl Album {
    /// Initializes a new Album.
    pub fn new(
        artist: &str,
        artwork_url: Option<&str>,
        title: &str,
        release_date: DateTime<Utc>,
        folder_path: &str,
    ) -> Self {
        let mut album = Self {
            artist: artist.to_string(),
            artwork_url: artwork_url.map(|url| url.to_string()),
            title: title.to_string(),
            release_date,
            tracks: Vec::new(),
            path: String::new(),
            playlist_path: String::new(),
            artwork_path: String::new(),
            artwork_temp_path: String::new(),
        };
        album.path = album.parse_folder_path(folder_path);
        album.playlist_path = album.parse_playlist_path();
        album.set_artwork_paths();

        album
    }

    /// True if the album has an artwork; false otherwise.
    pub fn has_artwork(&self) -> bool {
        self.artwork_url.is_some()
    }

    /// Returns the file extension to be used for the playlist, depending of the type of playlist
    /// defined in UserSettings.
    fn get_playlist_file_extension() -> &'static str {
        ""
    }

    /// Format a String by replacing the placeholders strings with their corresponding values.
    fn parse_format_str(&self, format: &str) -> String {
        format
            .replace("{year}", &self.release_date.year().to_string())
            .replace("{month}", &format!("{:02}", self.release_date.month()))
            .replace("{day}", &format!("{:02}", self.release_date.day()))
            .replace("{album}", &self.title)
            .replace("{artist}", &self.artist)
    }

    /// Returns the file name to be used for the cover art of the specified album from the file name
    /// format saved in the UserSettings, by replacing the placeholders strings with their
    /// corresponding values. The returned file name does NOT contain the extension.
    fn parse_cover_art_filename(&self, name_format: &str) -> String {
        self.parse_format_str(name_format)
    }

    /// Returns the folder path from the specified path format, by replacing the placeholders
    /// strings with their corresponding values. If the path is too long (&gt; 247 characters), it
    /// will be stripped.
    fn parse_folder_path(&self, format: &str) -> String {
        // TODO: conditional compilation for different OS path limits
        const MAX_PATH_LEN: usize = 247;
        let mut path = self.parse_format_str(format);

        if cfg!(target_os = "windows") && path.len() > MAX_PATH_LEN {
            // Windows doesn't do well with path >= 248 characters (and path + filename >= 260
            // characters)
            path.truncate(MAX_PATH_LEN);
        }

        path
    }

    /// Returns the file name to be used for the playlist file of the specified album from the file
    /// name format saved in the UserSettings, by replacing the placeholders strings with their
    /// corresponding values.
    fn parse_playlist_filename(&self, format: &str) -> String {
        self.parse_format_str(format)
    }

    // #[cfg(target_os = "windows")]
    fn truncate_win_path(path: &str, file_name: &str, file_ext: &str) -> PathBuf {
        const MAX_PATH_LEN: usize = 259;
        let mut file_path = PathBuf::from(path);
        file_path.push(file_name);
        file_path.set_extension(file_ext);

        if file_path.to_string_lossy().len() > MAX_PATH_LEN {
            // Windows doesn't do well with path + filename >= 260 characters (and path >= 248 characters) Path has
            // been shorten to 247 characters before, so we have 12 characters max left for "\filename.ext", so 11
            // character max for "filename.ext"
            let file_name_max_len = 11 - file_ext.len();
            file_path = PathBuf::from(path);
            file_path.push(&file_name[..file_name_max_len]);
            file_path.set_extension(file_ext);

            return file_path;
        }

        file_path
    }

    /// Returns the path to be used for the playlist file from the file name format saved in the
    /// UserSettings, by replacing the placeholders strings with their corresponding values. If the
    /// path is too long (&gt; 259 characters), it will be stripped.
    fn parse_playlist_path(&self) -> String {
        const PLAYLIST_NAME: &str = "{year}_{album}";

        // Compute paths where to save artwork
        let file_ext = Self::get_playlist_file_extension();
        let playlist_filename = self.parse_playlist_filename(PLAYLIST_NAME);
        let mut file_path = PathBuf::from(&self.path);
        file_path.push(&playlist_filename);
        file_path.set_extension(&file_ext);

        if cfg!(target_os = "windows") {
            file_path = Self::truncate_win_path(&self.path, &playlist_filename, &file_ext);
        }

        file_path.to_string_lossy().into()
    }

    /// Sets the ArtworkPath and ArtworkTempPath properties.
    fn set_artwork_paths(&mut self) {
        if self.artwork_url.is_none() {
            return;
        }
        // TODO
    }
}
