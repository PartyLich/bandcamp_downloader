use std::collections::HashSet;

use iced::{button, scrollable, text_input, Column, Container, Element, Length};

use super::UrlState;
use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message},
    IntlString, LogLevel, Progress,
};

/// main view UI state
#[derive(Debug, Default)]
pub struct State {
    pub url_state: UrlState,

    // TODO: move to app state
    pub download_progress: f32,
    pub downloading_files: HashSet<Progress>,
    pub log: Vec<String>,

    pub save_input: text_input::State,
    pub scroll_state: scrollable::State,
    pub download: button::State,
    pub cancel: button::State,
    pub settings: button::State,
}

impl State {
    pub fn new() -> Self {
        Self {
            url_state: UrlState::default(),

            download_progress: 0.0,
            downloading_files: HashSet::new(),
            log: Vec::new(),

            scroll_state: scrollable::State::new(),
            save_input: text_input::State::new(),
            download: button::State::new(),
            cancel: button::State::new(),
            settings: button::State::new(),
        }
    }

    pub fn add_log<T: ToString>(&mut self, value: T, _type: LogLevel) {
        self.log.push(value.to_string());
    }
}

pub fn view<'a>(
    state: &'a mut State,
    settings: &UserSettings,
    intl: &'a IntlString,
) -> Element<'a, Message> {
    let url_section = components::url_section(&mut state.url_state, intl);
    let save_dir = components::save_dir(
        &mut state.save_input,
        &settings.downloads_path.to_string_lossy(),
        intl,
    );
    let event_log = components::event_log(&mut state.scroll_state, &state.log, intl);
    let discog_checkbox =
        components::discography_checkbox(settings.download_artist_discography, intl);
    let progress_bar = components::download_progress_bar(&state.downloading_files);
    let controls = components::controls(
        &mut state.download,
        &mut state.cancel,
        &mut state.settings,
        intl,
    );

    let content = Column::new()
        .spacing(5)
        .push(url_section)
        .push(save_dir)
        .push(discog_checkbox)
        .push(progress_bar)
        .push(event_log)
        .push(controls);

    Container::new(content)
        .width(Length::Units(815))
        .height(Length::Units(440))
        .center_x()
        .center_y()
        .padding(10)
        .into()
}
