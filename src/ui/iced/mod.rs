//! UI implementation using iced crate

use components::EntryMessage;

mod components;
mod style;

/// UI event messages
#[derive(Debug, Clone)]
pub enum Message {
    Domain(ui::Message),
    UrlsChanged(String),
    SaveDirChanged(String),
    OpenSettings,
    AddUrl,
    SetSaveDir,
    UrlMessage(usize, EntryMessage),
}
