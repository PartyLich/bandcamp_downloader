use iced::{Align, Element, Row, TextInput};

use super::styled_text_input::styled_text_input;
use crate::ui::{iced::Message, iced::SettingType, IntlString};

pub fn filename_format_input<'a>(
    state: &'a mut iced::text_input::State,
    value: &str,
    intl: &IntlString,
) -> TextInput<'a, Message> {
    styled_text_input(state, &intl.save_input_placeholder, value, |a| {
        SettingType::FilenameFormat(a).into()
    })
    .on_submit(Message::SetSaveDir)
}

pub fn filename_format<'a>(
    state: &'a mut iced::text_input::State,
    filename_format: &str,
    intl: &IntlString,
) -> Element<'a, Message> {
    Row::new()
        .align_items(Align::Center)
        .push(super::StyledText(&intl.filename_format))
        .push(filename_format_input(state, filename_format, intl))
        .into()
}
