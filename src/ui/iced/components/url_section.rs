use iced::{scrollable, text_input, Column, Container, Element, Length, Scrollable};

use super::{styled_text_input::styled_text_input, Entry};
use crate::{
    ui::iced::{style, Message},
    ui::IntlString,
};

fn url_input<'a>(
    state: &'a mut iced::text_input::State,
    urls: &str,
    intl: &IntlString,
) -> Element<'a, Message> {
    let input = styled_text_input(state, &intl.urls_placeholder, urls, Message::UrlsChanged)
        .on_submit(Message::AddUrl);

    Container::new(input).width(Length::Fill).into()
}

#[derive(Debug)]
pub struct UrlState {
    pub input_state: text_input::State,
    pub input_value: String,
    pub url_list: Vec<Entry>,
    pub scroll_state: scrollable::State,
}

impl Default for UrlState {
    fn default() -> Self {
        Self {
            input_state: text_input::State::default(),
            input_value: String::default(),
            url_list: Vec::new(),
            scroll_state: scrollable::State::default(),
        }
    }
}

impl std::fmt::Display for UrlState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let urls = self
            .url_list
            .iter()
            .map(|entry| entry.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", urls)
    }
}

/// Url entry and display component
pub fn url_section<'a>(state: &'a mut UrlState, intl: &IntlString) -> Container<'a, Message> {
    let url_container = Column::new().padding(2).spacing(2).width(Length::Fill);
    let url_list =
        state
            .url_list
            .iter_mut()
            .enumerate()
            .fold(url_container, |container, (i, entry)| {
                container.push(
                    entry
                        .view(intl)
                        .map(move |message| Message::Url(i, message)),
                )
            });
    let scroll = Scrollable::new(&mut state.scroll_state)
        .width(Length::Fill)
        .push(Container::new(url_list));
    let input = url_input(&mut state.input_state, &state.input_value, intl);

    let content = Column::new().spacing(5).push(input).push(scroll);

    Container::new(content)
        .height(Length::Units(111))
        .width(Length::Fill)
}
