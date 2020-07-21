use iced::{button, Button, Element, HorizontalAlignment, Length, Row, Space};

use super::{style, styled_text};
use crate::ui::IntlString;

#[derive(Debug, Clone)]
pub enum EntryMessage {
    Delete,
}

#[derive(Debug, Clone)]
struct EntryState {
    delete_button: button::State,
}

impl Default for EntryState {
    fn default() -> Self {
        Self {
            delete_button: button::State::new(),
        }
    }
}

/// A URL entry
#[derive(Debug, Clone)]
pub struct Entry {
    url: String,
    ui_state: EntryState,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl Entry {
    pub fn new(url: String) -> Self {
        Self {
            url,
            ui_state: EntryState::default(),
        }
    }

    pub fn view(&mut self, intl: &IntlString) -> Element<EntryMessage> {
        let text = styled_text(&self.url);
        Row::new()
            .push(text)
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(
                Button::new(
                    &mut self.ui_state.delete_button,
                    styled_text(&intl.delete_button)
                        .size(12)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .height(Length::Units(22))
                .min_width(50)
                .style(style::Theme::Light)
                // .style(style::Button::Icon),
                .on_press(EntryMessage::Delete),
            )
            .push(Space::new(Length::Units(12), Length::Shrink))
            .into()
    }
}
