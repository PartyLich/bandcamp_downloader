//! Cover art settings view
use iced::{text_input, Column, Element, Length, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message, SettingType},
    IntlString,
};

/// Cover art settings view state
#[derive(Debug, Default)]
pub struct State {
    filename_input: text_input::State,
}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        macro_rules! checkbox {
            ($setting: ident, $intl_field: ident, $message: path) => {
                components::checkbox_row(settings.$setting, &intl.$intl_field, |a| {
                    $message(a).into()
                })
            };
        }

        let art_in_folder_checkbox = checkbox!(
            save_cover_art_in_folder,
            art_in_folder,
            SettingType::ArtInFolder
        );
        let art_in_tags_checkbox =
            checkbox!(save_cover_art_in_tags, art_in_tags, SettingType::ArtInTags);

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(art_in_folder_checkbox)
            .push(art_in_tags_checkbox)
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
