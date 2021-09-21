use iced::{button, text_input, Column, Container, Element, Length, Row, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{
        components::{self, buttons},
        Message,
    },
    IntlString,
};

/// Settings view UI state
#[derive(Debug, Default)]
pub struct State {
    pub save: button::State,
    pub cancel: button::State,
    pub filename_input: text_input::State,
}

impl State {
    pub fn new() -> Self {
        Self {
            save: button::State::new(),
            cancel: button::State::new(),
            filename_input: text_input::State::new(),
        }
    }
}

pub fn view<'a>(
    state: &'a mut State,
    settings: &UserSettings,
    intl: &'a IntlString,
) -> Element<'a, Message> {
    let filename_format =
        components::filename_format(&mut state.filename_input, &settings.file_name_format, intl);
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
    let controls = Row::new()
        .push(Space::with_width(Length::Fill))
        .push(buttons::cancel_settings(&mut state.cancel, intl))
        .push(Space::with_width(Length::Units(10)))
        .push(buttons::save_settings(&mut state.save, intl));

    let settings = Column::new()
        .spacing(5)
        .height(Length::Fill)
        .width(Length::FillPortion(2))
        .push(filename_format)
        .push(modify_tags_checkbox)
        .push(art_in_folder_checkbox)
        .push(art_in_tags_checkbox)
        .push(Space::with_height(Length::Fill))
        .push(controls);

    let content = Row::new().push(settings);

    Container::new(content)
        .width(Length::Units(815))
        .height(Length::Units(440))
        .center_x()
        .center_y()
        .padding(10)
        .into()
}
