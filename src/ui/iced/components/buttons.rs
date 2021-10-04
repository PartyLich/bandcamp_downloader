use iced::{button, Button, HorizontalAlignment, Length};

use super::StyledText;
use crate::{
    ui::iced::{style, Message},
    ui::{self, IntlString},
};

/// Create an iced Button with application styling
pub fn button<'a, M: Clone>(
    state: &'a mut button::State,
    text: &str,
    alignment: Option<HorizontalAlignment>,
) -> Button<'a, M> {
    let text_element = StyledText(text)
        .size(16)
        .horizontal_alignment(alignment.unwrap_or(HorizontalAlignment::Center));
    Button::new(state, text_element)
        .height(Length::Units(24))
        .width(Length::Shrink)
        .min_width(100)
        .style(style::Theme::Light)
}

pub fn download<'a>(state: &'a mut button::State, intl: &IntlString) -> Button<'a, Message> {
    button(state, &intl.download_button_text, None)
        .on_press(Message::Domain(ui::Message::StartDownloads))
}

pub fn settings<'a>(state: &'a mut button::State, intl: &IntlString) -> Button<'a, Message> {
    button(state, &intl.settings_button_text, None).on_press(Message::OpenSettings)
}

pub fn cancel_settings<'a>(state: &'a mut button::State, intl: &IntlString) -> Button<'a, Message> {
    button(state, &intl.main_button_text, None).on_press(Message::OpenMain)
}

pub fn save_settings<'a>(state: &'a mut button::State, intl: &IntlString) -> Button<'a, Message> {
    button(state, &intl.save_settings_button, None).on_press(Message::SettingsSaved)
}

pub fn cancel<'a>(state: &'a mut button::State, intl: &IntlString) -> Button<'a, Message> {
    button(state, &intl.cancel_button_text, None)
        .on_press(Message::Domain(ui::Message::CancelDownloads))
}
