//! Playlist settings view
use iced::{Column, Element, Length, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message, SettingType},
    IntlString,
};

/// Playlist settings view state
#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        macro_rules! checkbox {
            ($setting: ident, $intl_field: ident, $message: path) => {
                components::checkbox_row(settings.$setting, &intl.$intl_field, |a| {
                    $message(a).into()
                })
            };
        }

        let playlist_checkbox = checkbox!(
            create_playlist,
            create_playlist,
            SettingType::CreatePlaylist
        );

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(playlist_checkbox)
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
