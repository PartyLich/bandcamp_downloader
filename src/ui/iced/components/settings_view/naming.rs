//! Naming and Tag settings view
use iced::{text_input, Column, Element, Length, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message},
    IntlString,
};

// Naming and Tag settings view state
#[derive(Debug, Default)]
pub struct State {
    pub filename_input: text_input::State,
}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        let filename_format =
            components::filename_format(&mut self.filename_input, &settings.file_name_format, intl);
        let modify_tags_checkbox = components::checkbox_row(
            settings.modify_tags,
            &intl.modify_tags_checkbox,
            Message::ModifyTagsToggled,
        );
        let art_in_folder_checkbox = components::checkbox_row(
            settings.save_cover_art_in_folder,
            &intl.art_in_folder,
            Message::ArtInFolderToggled,
        );
        let art_in_tags_checkbox = components::checkbox_row(
            settings.save_cover_art_in_tags,
            &intl.art_in_tags,
            Message::ArtInTagsToggled,
        );

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(filename_format)
            .push(modify_tags_checkbox)
            // TODO: move to art view
            .push(art_in_folder_checkbox)
            .push(art_in_tags_checkbox)
            //
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
