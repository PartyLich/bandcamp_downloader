use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Album;
use crate::helper;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Track {
    /// The track length (in seconds).
    pub duration: f32,

    /// The track lyrics.
    pub lyrics: Option<String>,

    /// The URL where the track should be downloaded from.
    pub mp3_url: String,

    /// The track number.
    pub number: u32,

    /// The local path (full path with file name) where the track file should be saved.
    pub path: String,

    /// The track title.
    pub title: String,
}

impl Track {
    /// Initializes a new Track.
    pub fn new(
        album: &Album,
        duration: f32,
        lyrics: Option<String>,
        mp3_url: String,
        number: u32,
        title: String,
        filename_format: &str,
    ) -> Self {
        let mut track = Self {
            duration,
            lyrics,
            mp3_url,
            number,
            title,
            path: String::new(),
        };
        track.path = track.parse_track_file_path(album, filename_format);

        track
    }

    /// Returns the file name to be used for the track from the provided file name format, by
    /// replacing the placeholders strings with their corresponding values.
    /// The returned file name DOES contain the extension.
    fn parse_track_filename(&self, filename_format: &str, album: &Album) -> String {
        let file_name = helper::parse_filename(filename_format, album)
            .replace("{title}", &self.title)
            .replace("{tracknum}", &format!("{:02}", self.number));

        helper::sanitize_file_name(&file_name)
    }

    /// Returns the file path to be used for the track from the file name format saved in the UserSettings, by
    /// replacing the placeholders strings with their corresponding values. The returned file path DOES contain the extension.
    fn parse_track_file_path(&self, album: &Album, filename_format: &str) -> String {
        let file_name = self.parse_track_filename(filename_format, album);
        let path: PathBuf = [&album.path, &file_name].iter().collect();

        path.to_string_lossy().into()
    }
}
