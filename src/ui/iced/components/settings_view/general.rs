//! General settings view
use iced::{pick_list, Align, Column, Container, Element, Length, Row};

use crate::settings::{Language, UserSettings};
use crate::ui::{
    iced::{components, style::Theme, Message, SettingType},
    IntlString,
};

#[derive(Debug)]
pub struct State {
    language_list: pick_list::State<Language>,
    theme_list: pick_list::State<Theme>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            language_list: Default::default(),
            theme_list: Default::default(),
        }
    }
}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        let content = Column::new()
            .spacing(5)
            .push(language_picker(
                &mut self.language_list,
                &settings.language,
                intl,
            ))
            .push(theme_picker(&mut self.theme_list, &settings.theme, intl));

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
        |a| SettingType::Language(a).into(),
    );

    Row::new()
        .spacing(5)
        .align_items(Align::Center)
        .push(label)
        .push(pick_list)
        .into()
}

fn theme_picker<'a>(
    pick_list_state: &'a mut pick_list::State<Theme>,
    selected_theme: &Theme,
    intl: &IntlString,
) -> Element<'a, Message> {
    let label = components::StyledText(format!("{}:", &intl.theme));

    let pick_list = components::styled_pick_list(
        pick_list_state,
        &Theme::ALL[..],
        Some(*selected_theme),
        |a| SettingType::Theme(a).into(),
    );

    Row::new()
        .spacing(5)
        .align_items(Align::Center)
        .push(label)
        .push(pick_list)
        .into()
}
