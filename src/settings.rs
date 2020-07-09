#[derive(Debug, Copy, Clone)]
pub enum Language {
    EN,
}

/// Available playlist export formats
#[derive(Debug, Copy, Clone)]
pub enum PlaylistFormat {
    /// MP3 url format
    M3U,
    /// PLS multimedia playlist format
    PLS,
    /// Windows Media Player format
    WPL,
    /// Zune Media Player format
    ZPL,
}

impl PlaylistFormat {
    pub const ALL: [PlaylistFormat; 4] = [Self::M3U, Self::PLS, Self::WPL, Self::ZPL];

    pub fn value(&self) -> &str {
        match self {
            Self::M3U => "m3u",
            Self::PLS => "pls",
            Self::WPL => "wpl",
            Self::ZPL => "zpl",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::M3U => "(MP3 url)",
            Self::PLS => "(PLS multimedia playlist)",
            Self::WPL => " (Windows Media Player)",
            Self::ZPL => " (Zune Media Player)",
        }
    }
}

impl std::fmt::Display for PlaylistFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value(), self.description())
    }
}

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub allowed_file_size_difference: f32,

    pub check_for_updates: bool,

    pub download_artist_discography: bool,
    pub download_one_album_at_a_time: bool,
    pub download_albums_serial: bool,
    pub downloads_path: String,

    /// Time in seconds between retries
    pub download_retry_cooldown: f64,

    pub file_name_format: String,
    pub playlist_file_name_format: String,
    pub cover_art_file_name_format: String,

    pub create_playlist: bool,
    pub playlist_format: PlaylistFormat,

    pub retrieve_files_size: bool,

    pub save_cover_art_in_folder: bool,
    pub save_cover_art_in_tags: bool,

    pub show_verbose_log: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            check_for_updates: true,

            allowed_file_size_difference: 0.05,

            file_name_format: String::from("{tracknum} {artist} - {title}.mp3"),
            cover_art_file_name_format: String::from("{album}"),
            playlist_file_name_format: String::from("{album}"),

            playlist_format: PlaylistFormat::M3U,
            create_playlist: false,

            download_artist_discography: false,
            download_one_album_at_a_time: false,
            download_albums_serial: false,
            downloads_path: String::from(""),

            download_retry_cooldown: 0.2,

            retrieve_files_size: true,

            save_cover_art_in_folder: false,
            save_cover_art_in_tags: true,

            show_verbose_log: false,
        }
    }
}
