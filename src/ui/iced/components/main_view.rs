use std::collections::HashSet;

use iced::{button, scrollable, text_input, Column, Container, Element, Length};

use super::UrlState;
use crate::ui::{
    iced::{components, Message},
    IntlString, LogLevel, Progress,
};

/// UI state
#[derive(Debug)]
pub struct State {
    pub intl: IntlString,

    pub url_state: UrlState,

    pub save_dir: String,
    pub save_input: text_input::State,

    pub download_discography: bool,
    pub download_progress: f32,

    pub log: Vec<String>,
    pub scroll_state: scrollable::State,

    pub download: button::State,
    pub cancel: button::State,
    pub settings: button::State,

    pub downloading_files: HashSet<Progress>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            intl: IntlString::default(),

            url_state: UrlState::default(),

            save_dir: String::default(),
            save_input: text_input::State::new(),

            download_discography: false,
            download_progress: 0.0,

            log: Vec::new(),
            scroll_state: scrollable::State::new(),

            download: button::State::new(),
            cancel: button::State::new(),
            settings: button::State::new(),

            downloading_files: HashSet::new(),
        }
    }
}

impl State {
    pub fn add_log<T: ToString>(&mut self, value: T, _type: LogLevel) {
        self.log.push(value.to_string());
    }

    pub fn title(&self) -> String {
        self.intl.title.clone()
    }
}

pub fn view(state: &mut State) -> Element<Message> {
    let url_section = components::url_section(&mut state.url_state, &state.intl);
    let save_dir = components::save_dir(&mut state.save_input, &state.save_dir, &state.intl);
    let event_log = components::event_log(&mut state.scroll_state, &state.log, &state.intl);
    let discog_checkbox = components::discography_checkbox(state.download_discography, &state.intl);
    let progress_bar = components::download_progress_bar(&state.downloading_files);
    let controls = components::controls(
        &mut state.download,
        &mut state.cancel,
        &mut state.settings,
        &state.intl,
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
