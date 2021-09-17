use iced::{Column, Container, Element, Length, Row, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message},
    IntlString,
};

/// Settings view UI state
#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn view<'a>(
    state: &'a mut State,
    settings: &UserSettings,
    intl: &'a IntlString,
) -> Element<'a, Message> {
    let modify_tags_checkbox = components::checkbox_row(
        settings.modify_tags,
        &intl.modify_tags_checkbox,
        Message::ModifyTagsToggled,
    );
    let controls = Row::new().push(Space::with_width(Length::Fill));

    let settings = Column::new()
        .spacing(5)
        .height(Length::Fill)
        .width(Length::FillPortion(2))
        .push(modify_tags_checkbox)
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
