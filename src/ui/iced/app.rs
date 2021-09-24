use std::sync::Arc;

use futures::channel::mpsc;
use iced::{Application, Command, Element, Settings, Subscription};
use tokio::sync::Mutex;

use super::{
    components::{main_view, settings_view, Entry, EntryMessage},
    subscription, Message,
};
use crate::{core::DownloadService, helper::log_info, settings::UserSettings, ui};

/// Application flags
#[derive(Debug)]
pub struct AppFlags {
    pub user_settings: UserSettings,
}

type SharedReceiver<T> = Arc<Mutex<mpsc::Receiver<T>>>;

/// UI state for each view
#[derive(Debug)]
pub struct UiState {
    main: main_view::State,
    settings: settings_view::State,
}

/// Renderable views
#[derive(Debug)]
enum View {
    Main,
    Settings,
}

impl View {
    /// Returns the localized window title for this view
    fn title(&self, intl: &ui::IntlString) -> String {
        // todo: anything that doesnt require these explicit lines for every view
        // a trait or something?
        match self {
            Self::Main => intl.title.clone(),
            Self::Settings => intl.settings_title.clone(),
        }
    }
}

#[derive(Debug)]
pub struct App {
    user_settings: Arc<std::sync::Mutex<UserSettings>>,
    download_service: Arc<DownloadService>,
    intl: Arc<ui::IntlString>,

    ui_state: UiState,
    cur_view: View,

    sender: mpsc::Sender<ui::Message>,
    receiver: SharedReceiver<ui::Message>,
}

impl App {
    /// Create a new instance
    pub fn new(flags: AppFlags) -> Self {
        let AppFlags { user_settings } = flags;
        let (sender, receiver) = mpsc::channel(50);

        let intl = Arc::new(ui::IntlString::default());
        let user_settings = Arc::new(std::sync::Mutex::new(user_settings));
        let download_service = DownloadService::new();
        let ui_state = UiState {
            main: main_view::State::new(),
            settings: settings_view::State::new(),
        };

        Self {
            user_settings,
            download_service: Arc::new(download_service),
            intl,

            ui_state,
            cur_view: View::Main,

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

    fn set_url_input(&mut self, value: String) {
        self.ui_state.main.url_state.input_value = value;
    }

    fn clear_urls(&mut self) {
        self.ui_state.main.url_state.url_list.clear();
    }

    fn urls(&self) -> String {
        self.ui_state.main.url_state.to_string()
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
        self.cur_view.title(&self.intl)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UrlsChanged(value) => {
                self.set_url_input(value);
            }
            Message::SaveDirChanged(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.downloads_path = value.into();
            }
            Message::FilenameFormatChanged(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.file_name_format = value;
            }
            Message::DiscographyToggled(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.download_artist_discography = value;
            }
            Message::ArtInFolderToggled(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.save_cover_art_in_folder = value;
            }
            Message::ArtInTagsToggled(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.save_cover_art_in_tags = value;
            }
            Message::ModifyTagsToggled(value) => {
                let mut user_settings = self.user_settings.lock().unwrap();
                user_settings.modify_tags = value;
            }
            Message::AddUrl => {
                if self.ui_state.main.url_state.input_value.is_empty() {
                    return Command::none();
                }

                self.ui_state
                    .main
                    .url_state
                    .url_list
                    .push(Entry::new(self.ui_state.main.url_state.input_value.clone()));
                self.ui_state.main.url_state.input_value.clear();
            }
            Message::ClearUrls => {
                self.clear_urls();
            }
            Message::Url(i, entry_message) => match entry_message {
                EntryMessage::Delete => {
                    self.ui_state.main.url_state.url_list.remove(i);
                }
            },
            Message::OpenSettings => {
                self.cur_view = View::Settings;
            }
            Message::OpenMain => {
                self.cur_view = View::Main;
            }
            Message::Domain(ui::Message::StartDownloads) => {
                let urls = self.urls();
                log_info(self.sender.clone(), format!("Start download\n{}", urls));
                self.ui_state.main.downloading_files.clear();

                let settings = self.user_settings.lock().unwrap().clone();
                return Command::perform(
                    Arc::clone(&self.download_service).start_downloads(
                        urls,
                        self.sender.clone(),
                        settings,
                    ),
                    Message::DownloadsComplete,
                );
            }
            Message::Domain(ui::Message::CancelDownloads) => {
                log_info(self.sender.clone(), "cancel download clicked");
            }
            Message::Domain(ui::Message::Log(value, level)) => {
                println!("{}", value);
                self.ui_state.main.add_log(&value, level);
            }
            Message::Domain(ui::Message::Progress(dl_progress)) => {
                self.ui_state.main.downloading_files.replace(dl_progress);
            }
            Message::DownloadsComplete(_) => {
                log_info(self.sender.clone(), "All downloads complete");
            }
            Message::SettingsSaved => {
                let settings = self.user_settings.clone();
                return Command::perform(async move { settings.lock().unwrap().save() }, |_| {
                    Message::OpenMain
                });
            }
            Message::SetSaveDir => {}
            Message::SettingsChanged(..) => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let settings = self.user_settings.lock().unwrap();
        match self.cur_view {
            View::Main => main_view::view(&mut self.ui_state.main, &settings, &self.intl),
            View::Settings => {
                settings_view::view(&mut self.ui_state.settings, &settings, &self.intl)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::log(Arc::clone(&self.receiver)).map(Message::Domain)
    }
}
