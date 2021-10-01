//! User interface module
use std::hash::{Hash, Hasher};

use crate::settings::UserSettings;

pub use self::iced::IcedUi;
pub use intl::IntlString;

mod iced;
mod intl;

/// Behavior required by a user interface driving the core logic.
pub trait Ui {
    /// Start the user interface with the supplied [`UserSettings`]
    fn run(&self, user_settings: UserSettings);
}

/// Log message severity levels.
#[derive(Debug, Clone)]
pub enum LogLevel {
    /// A notice of normal event progress, for information only.
    Info,
    /// A potential concern that doesn't *require* intervention.
    Warn,
    /// A problem requiring intervention.
    Error,
}

/// Download progress state
#[derive(Debug, Clone, Eq)]
pub struct Progress {
    /// File download path
    pub path: String,
    /// Bytes completed
    pub complete: u64,
    /// Total bytes expected
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
    /// Start file downloads
    StartDownloads,
    /// Cancel all in-progress downloads
    CancelDownloads,
    /// Log some text at the specified log level
    Log(String, LogLevel),
    /// Update file download progress
    Progress(Progress),
}

/// UI theme (colorscheme)
#[derive(Debug, PartialEq)]
pub enum Theme {
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

impl Theme {
    pub const ALL: [Theme; 1] = [Theme::Light];
}
