use std::path::PathBuf;

use chrono::Datelike;
use serde::{Deserialize, Serialize};

use super::Album;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Track {
    /// The track length (in seconds).
    pub duration: f32,

    /// The track lyrics.
    pub lyrics: Option<String>,

    /// The URL where the track should be downloaded from.
    pub mp3_url: Option<String>,

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
        mp3_url: Option<String>,
        number: u32,
        title: String,
    ) -> Self {
        let mut track = Self {
            duration,
            lyrics,
            mp3_url,
            number,
            title,
            path: String::new(),
        };
        track.path = track.parse_track_file_path(album);

        track
    }

    /// Returns the file name to be used for the track from the provided file name format, by
    /// replacing the placeholders strings with their corresponding values.
    /// The returned file name DOES contain the extension.
    fn parse_track_filename(&self, filename_format: &str, album: &Album) -> String {
        filename_format
            .replace("{year}", &album.release_date.year().to_string())
            .replace("{month}", &format!("{:02}", album.release_date.month()))
            .replace("{day}", &format!("{:02}", album.release_date.day()))
            .replace("{album}", &album.title)
            .replace("{artist}", &album.artist)
            .replace("{title}", &self.title)
            .replace("{tracknum}", &format!("{:02}", self.number))
    }

    /// Returns the file path to be used for the track from the file name format saved in the UserSettings, by
    /// replacing the placeholders strings with their corresponding values. The returned file path DOES contain the extension.
    fn parse_track_file_path(&self, album: &Album) -> String {
        let filename_format = "{tracknum} - {title}.mp3";
        let file_name = self.parse_track_filename(filename_format, album);
        let path: PathBuf = [&album.path, &file_name].iter().collect();

        path.to_string_lossy().into()
    }
}
