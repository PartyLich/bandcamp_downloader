use std::hash::{Hash, Hasher};

use crate::settings::UserSettings;

pub use self::iced::IcedUi;
pub use intl::IntlString;

mod iced;
mod intl;

/// Behavior required by a user interface driving the core logic.
pub trait Ui {
    /// Start the user interface with the supplied `UserSettings`
    fn run(&self, user_settings: UserSettings);
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Warn,
    Info,
    Error,
}

/// Download progress event
#[derive(Debug, Clone, Eq)]
pub struct Progress {
    pub path: String,
    pub complete: u64,
    pub total: u64,
}

impl Hash for Progress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.total.hash(state);
    }
}

impl PartialEq for Progress {
    fn eq(&self, other: &Self) -> bool {
        (self.path == other.path) && (self.total == other.total)
    }
}

/// Domain events
#[derive(Debug, Clone)]
pub enum Message {
    StartDownloads,
    CancelDownloads,
    Log(String, LogLevel),
    Progress(Progress),
}
