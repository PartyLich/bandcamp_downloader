use iced::{button, Column, Container, Element, Length, Row, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components::buttons, Message},
    IntlString,
};

mod general;
mod naming;

/// Renderable views for Settings sections
#[derive(Debug)]
pub enum View {
    General(general::State),
    Naming(naming::State),
}

impl Default for View {
    fn default() -> Self {
        Self::General(Default::default())
    }
}

impl View {
    fn view<'a>(
        &'a mut self,
        settings: &UserSettings,
        intl: &'a IntlString,
    ) -> Element<'a, Message> {
        match self {
            Self::Naming(state) => state.view(settings, intl),
            Self::General(state) => state.view(settings, intl),
        }
    }
}

#[derive(Debug, Default)]
struct Sections {
    general: button::State,
    naming: button::State,
}

impl Sections {
    fn view<'a>(&'a mut self, intl: &'a IntlString) -> Container<'a, Message> {
        // view select buttons
        let general = buttons::button(&mut self.general, &intl.general).width(Length::Fill);
        // TODO: intl strings
        let naming = buttons::button(&mut self.naming, "Naming and Tags").width(Length::Fill);

        Container::new(
            Column::new()
                .spacing(5)
                .padding(4)
                .push(general)
                .push(naming)
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::FillPortion(1))
    }
}

/// Settings view UI state
#[derive(Debug, Default)]
pub struct State {
    pub current_view: View,
    save: button::State,
    cancel: button::State,
    sections: Sections,
}

impl State {
    // render function
    pub fn view<'a>(
        &'a mut self,
        settings: &UserSettings,
        intl: &'a IntlString,
    ) -> Element<'a, Message> {
        let sections = self.sections.view(intl);

        let controls = Row::new()
            .spacing(5)
            .push(Space::with_width(Length::Fill))
            .push(buttons::cancel_settings(&mut self.cancel, intl))
            .push(buttons::save_settings(&mut self.save, intl));

        let current_view = self.current_view.view(settings, intl);
        let current_view = Column::new()
            .padding(4)
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(current_view)
            .push(controls);

        let content = Row::new()
            .height(Length::Fill)
            .push(sections)
            .push(Space::with_width(Length::Units(16)))
            .push(current_view);

        Container::new(content)
            .width(Length::Units(815))
            .height(Length::Units(440))
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }
}
