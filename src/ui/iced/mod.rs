//! UI implementation using iced crate
use iced::Application;

use crate::core::tag;
use crate::{settings::UserSettings, ui};
use app::{App, AppFlags};
use components::settings_view;
use components::EntryMessage;

mod app;
mod components;
mod style;
mod subscription;

/// User interface using iced framework
#[derive(Debug, Default)]
pub struct IcedUi {}

impl ui::Ui for IcedUi {
    fn run(&self, user_settings: UserSettings) {
        App::run(App::default_settings(AppFlags { user_settings })).unwrap();
    }
}

/// user settings as an enum
#[derive(Debug, Clone)]
pub enum SettingType {
    Language(crate::settings::Language),
    Theme(style::Theme),
    SaveDir(String),
    FilenameFormat(String),
    Discography(bool),

    ArtFilename(String),
    ArtInFolder(bool),
    ArtInTags(bool),

    ModifyTags(bool),
    TagYear(tag::EditAction),
    TagAlbumArtist(tag::EditAction),
    TagAlbumTitle(tag::EditAction),
    TagArtist(tag::EditAction),
    TagComments(tag::EditAction),
    TagLyrics(tag::EditAction),
    TagTrackNumber(tag::EditAction),
    TagTrackTitle(tag::EditAction),

    CreatePlaylist(bool),
}

/// UI event messages
#[derive(Debug, Clone)]
pub enum Message {
    Domain(ui::Message),
    UrlsChanged(String),
    OpenSettings,
    OpenMain,
    AddUrl,
    ClearUrls,
    SetSaveDir,
    DownloadsComplete(()),
    Url(usize, EntryMessage),
    SettingsChanged(SettingType),
    SettingsSaved,
    Settings(settings_view::SettingsMessage),
}

impl From<settings_view::SettingsMessage> for Message {
    fn from(message: settings_view::SettingsMessage) -> Self {
        Self::Settings(message)
    }
}

impl From<SettingType> for Message {
    fn from(message: SettingType) -> Self {
        Self::SettingsChanged(message)
    }
}
