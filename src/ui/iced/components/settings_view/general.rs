//! General settings view
use iced::{Column, Element, Row, Space};

use super::Message;
use crate::settings::Language;
use crate::settings::UserSettings;
use crate::ui::{iced::components, IntlString};

#[derive(Debug)]
pub struct State {
    selected_language: Language,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected_language: Language::EN,
        }
    }
}

impl State {
    pub fn view(&mut self, _settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        Column::new()
            .push(language_picker(intl))
            .push(Space::with_height(300.into()))
            .into()
    }
}

fn language_picker<'a>(intl: &IntlString) -> Element<'a, Message> {
    // TODO: from intl
    let label = format!("{}:", "Language");
    let label = components::StyledText(label);

    Row::new().spacing(5).push(label).into()
}
