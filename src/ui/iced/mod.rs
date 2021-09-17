//! UI implementation using iced crate
use iced::Application;

use crate::{settings::UserSettings, ui};
use app::{App, AppFlags};
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
        App::run(App::default_settings(AppFlags { user_settings }));
    }
}

#[derive(Debug, Clone)]
pub enum SettingType {
    Language,
    Other,
}

/// UI event messages
#[derive(Debug, Clone)]
pub enum Message {
    Domain(ui::Message),
    UrlsChanged(String),
    SaveDirChanged(String),
    FilenameFormatChanged(String),
    DiscographyToggled(bool),
    ModifyTagsToggled(bool),
    OpenSettings,
    OpenMain,
    AddUrl,
    ClearUrls,
    SetSaveDir,
    DownloadsComplete(()),
    Url(usize, EntryMessage),
    SettingsChanged(SettingType),
}
