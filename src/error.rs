use std::fmt;

use serde_json::error::Category;

#[derive(Debug, Clone)]
pub enum Error {
    API,
    Download,
    Io(String),
    NoAlbumData,
    NoAlbumFound,
    NoDiscography,
    NoArtwork,
    Serialization(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::API => write!(f, "API error occured"),
            Self::Download => write!(f, "Download error"),
            Self::Io(_) => write!(f, "IO error"),
            Self::NoAlbumData => write!(f, "No album data found for this artist"),
            Self::NoAlbumFound => write!(f, "No album found for this artist"),
            Self::NoArtwork => write!(f, "No artwork found for this album"),
            Self::NoDiscography => write!(f, "No discography could be found on the supplied url"),
            Self::Serialization(_) => write!(f, "A serialization error occured"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::API
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        let msg = match error.classify() {
            Category::Data => "Serialization error: input data that is semantically incorrect",
            Category::Syntax => "Serialization error: input that is not syntactically valid JSON",
            _ => "A serialization error occured",
        };

        Error::Serialization(format!("{}:\n\t{:?}", msg, error))
    }
}

impl From<tokio::io::Error> for Error {
    fn from(error: tokio::io::Error) -> Error {
        let msg = match error.kind() {
            tokio::io::ErrorKind::NotFound => "IO error: an entity was not found",
            tokio::io::ErrorKind::PermissionDenied => {
                "IO error: operation lacked the necessary privileges to complete."
            }
            _ => "An IO error occured",
        };

        Error::Io(format!("{}:\n\t{:?}", msg, error))
    }
}
