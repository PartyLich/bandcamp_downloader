use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    APIError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::APIError => write!(f, "API error occured"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
