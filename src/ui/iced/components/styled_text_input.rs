use iced::{Align, Row};
use iced::{Element, TextInput};

use super::TEXT_SIZE;
use crate::ui::{
    iced::{Message, SettingType},
    IntlString,
};

/// Returns a styled TextInput
pub fn styled_text_input<'a, F>(
    state: &'a mut iced::text_input::State,
    placeholder: &str,
    value: &str,
    message: F,
) -> TextInput<'a, Message>
where
    F: 'static + Fn(String) -> Message,
{
    TextInput::new(state, placeholder, value, message)
        .size(TEXT_SIZE)
        .padding(5)
}

pub fn save_input<'a>(
    state: &'a mut iced::text_input::State,
    value: &str,
    intl: &IntlString,
) -> TextInput<'a, Message> {
    styled_text_input(state, &intl.save_input_placeholder, value, |a| {
        SettingType::SaveDir(a).into()
    })
    .on_submit(Message::SetSaveDir)
}

pub fn save_dir<'a>(
    state: &'a mut iced::text_input::State,
    save_dir: &str,
    intl: &IntlString,
) -> Element<'a, Message> {
    Row::new()
        .align_items(Align::Center)
        .push(super::StyledText(&intl.save_caption))
        .push(save_input(state, save_dir, intl))
        .into()
}
