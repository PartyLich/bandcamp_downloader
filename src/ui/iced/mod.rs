//! UI implementation using iced crate
use iced::Application;

use crate::{core::DownloadService, settings::UserSettings, ui};
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
        let download_service = DownloadService::new(user_settings.clone());

        App::run(App::default_settings(AppFlags {
            download_service,
            user_settings,
        }));
    }
}

/// UI event messages
#[derive(Debug, Clone)]
pub enum Message {
    Domain(ui::Message),
    UrlsChanged(String),
    SaveDirChanged(String),
    DiscographyToggled(bool),
    OpenSettings,
    AddUrl,
    ClearUrls,
    SetSaveDir,
    DownloadsComplete(()),
    UrlMessage(usize, EntryMessage),
}
