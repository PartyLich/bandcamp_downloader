//! General settings view
use iced::{pick_list, Align, Column, Container, Element, Length, Row};

use crate::settings::{Language, UserSettings};
use crate::ui::{
    iced::{components, style::Theme, Message},
    IntlString,
};

#[derive(Debug)]
pub struct State {
    language_list: pick_list::State<Language>,
    selected_language: Language,
}

impl Default for State {
    fn default() -> Self {
        Self {
            language_list: Default::default(),
            selected_language: UserSettings::default().language,
        }
    }
}

impl State {
    pub fn view(&mut self, _settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        let content = Column::new().spacing(5).push(language_picker(
            &mut self.language_list,
            &self.selected_language,
            intl,
        ));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn language_picker<'a>(
    pick_list_state: &'a mut pick_list::State<Language>,
    selected_language: &Language,
    intl: &IntlString,
) -> Element<'a, Message> {
    let label = components::StyledText(format!("{}:", &intl.language));

    let pick_list = components::styled_pick_list(
        pick_list_state,
        &Language::ALL[..],
        Some(*selected_language),
        Message::LanguageChanged,
    );

    Row::new()
        .spacing(5)
        .align_items(Align::Center)
        .push(label)
        .push(pick_list)
        .into()
}
