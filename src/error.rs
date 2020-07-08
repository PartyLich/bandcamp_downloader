use std::fmt;

use serde_json::error::Category;

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    NoAlbumData,
    Serialization(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::APIError => write!(f, "API error occured"),
            Self::NoAlbumData => write!(f, "No album data found for this artist"),
            Self::Serialization(_) => write!(f, "A serialization error occured"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
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
