pub use album::Album;
pub use json::*;
pub use track::Track;

mod album;
mod json;
mod track;

enum FileType {
    Artwork,
    Track,
}

#[derive(Debug, PartialEq)]
pub struct TrackFile {
    pub bytes_received: usize,
    pub downloaded: bool,
    pub size: usize,
    pub url: String,
}

impl TrackFile {
    pub fn new(url: String, bytes_received: usize, size: usize) -> Self {
        Self {
            url,
            bytes_received,
            size,
            downloaded: false,
        }
    }
}
