use futures::channel::mpsc;
use std::sync::Arc;

use iced::{Application, Command, Element, Settings, Subscription};
use tokio::sync::{Mutex, RwLock};

use super::{
    components::{main_view, Entry, EntryMessage},
    subscription, Message, SettingType,
};
use crate::{
    core::DownloadService,
    helper::{log_error, log_info, log_warn},
    settings::UserSettings,
    ui,
};

/// Application flags
#[derive(Debug)]
pub struct AppFlags {
    pub user_settings: UserSettings,
}

type SharedReceiver<T> = Arc<Mutex<mpsc::Receiver<T>>>;

#[derive(Debug)]
pub struct App {
    user_settings: Arc<RwLock<UserSettings>>,
    download_service: Arc<DownloadService>,
    ui_state: main_view::State,

    sender: mpsc::Sender<ui::Message>,
    receiver: SharedReceiver<ui::Message>,
}

impl App {
    /// Create a new instance
    pub fn new(flags: AppFlags) -> Self {
        let AppFlags { user_settings } = flags;
        let (sender, receiver) = mpsc::channel(50);
        let mut ui_state = main_view::State::default();
        ui_state.download_discography = user_settings.download_artist_discography;
        let user_settings = Arc::new(RwLock::new(user_settings));
        let download_service = DownloadService::new(Arc::clone(&user_settings));

        Self {
            user_settings,
            download_service: Arc::new(download_service),
            ui_state,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    /// Get default application settings
    pub fn default_settings(flags: AppFlags) -> Settings<AppFlags> {
        Settings {
            window: Self::window_settings(),
            ..Settings::with_flags(flags)
        }
    }

    /// Get default window settings
    pub fn window_settings() -> iced::window::Settings {
        const WIDTH: u32 = 815;
        const HEIGHT: u32 = 440;
        iced::window::Settings {
            size: (WIDTH, HEIGHT),
            ..iced::window::Settings::default()
        }
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.ui_state.intl.title
    }

    fn set_url_input(&mut self, value: String) {
        self.ui_state.url_state.input_value = value;
    }

    fn clear_urls(&mut self) {
        self.ui_state.url_state.url_list.clear();
    }

    fn urls(&self) -> String {
        self.ui_state.url_state.to_string()
    }
}

// Main window
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = AppFlags;

    fn new(flags: AppFlags) -> (Self, Command<Message>) {
        (Self::new(flags), Command::none())
    }

    fn title(&self) -> String {
        self.ui_state.title()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UrlsChanged(value) => {
                self.set_url_input(value);
            }
            Message::SaveDirChanged(value) => {
                self.ui_state.save_dir = value;
            }
            Message::DiscographyToggled(value) => {
                self.ui_state.download_discography = value;

                let settings = self.user_settings.clone();
                return Command::perform(
                    async move {
                        let mut settings = settings.write().await;
                        settings.download_artist_discography = value;
                    },
                    |_| Message::SettingsChanged(SettingType::Other),
                );
            }
            Message::AddUrl => {
                if !self.ui_state.url_state.input_value.is_empty() {
                    self.ui_state
                        .url_state
                        .url_list
                        .push(Entry::new(self.ui_state.url_state.input_value.clone()));
                    self.ui_state.url_state.input_value.clear();
                }
            }
            Message::ClearUrls => {
                self.clear_urls();
            }
            Message::UrlMessage(i, entry_message) => match entry_message {
                EntryMessage::Delete => {
                    self.ui_state.url_state.url_list.remove(i);
                }
            },
            Message::OpenSettings => {
                println!("open settings");
            }
            Message::Domain(ui::Message::StartDownloads) => {
                log_info(
                    self.sender.clone(),
                    format!("Start download\n{}", self.urls()),
                );
                self.ui_state.downloading_files.clear();
                return Command::perform(
                    Arc::clone(&self.download_service)
                        .start_downloads(self.urls(), self.sender.clone()),
                    Message::DownloadsComplete,
                );
            }
            Message::Domain(ui::Message::CancelDownloads) => {
                log_info(self.sender.clone(), "cancel download clicked");
            }
            Message::Domain(ui::Message::Log(value, level)) => {
                println!("{}", value);
                self.ui_state.add_log(&value, level);
            }
            Message::Domain(ui::Message::Progress(dl_progress)) => {
                self.ui_state.downloading_files.replace(dl_progress);
            }
            Message::DownloadsComplete(_) => {
                log_info(self.sender.clone(), "All downloads complete");
            }
            Message::SetSaveDir => {}
            Message::SettingsChanged(..) => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        main_view::view(&mut self.ui_state)
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::log(Arc::clone(&self.receiver)).map(Message::Domain)
    }
}
