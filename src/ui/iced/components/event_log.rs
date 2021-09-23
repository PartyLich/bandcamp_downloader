use iced::{scrollable, Column, Container, Element, Length, Scrollable};

use super::StyledText;
use crate::ui::{iced::Message, IntlString};

pub fn event_log<'a>(
    state: &'a mut scrollable::State,
    value: &[String],
    intl: &IntlString,
) -> Element<'a, Message> {
    const LOG_HEIGHT: u16 = 200;
    let container = Column::new();
    let content = if value.is_empty() {
        container.push(StyledText(&intl.log_placeholder))
    } else {
        container.push(StyledText(value.join("\n")))
    };

    let scroll = Scrollable::new(state).push(Container::new(content).width(Length::Fill));

    Container::new(scroll)
        .height(Length::Units(LOG_HEIGHT))
        .into()
}
