//! Download settings view
use iced::{Column, Element, Length, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message, SettingType},
    IntlString,
};

/// Download settings view state
#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
