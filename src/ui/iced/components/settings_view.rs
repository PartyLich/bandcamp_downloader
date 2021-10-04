use iced::{button, Column, Container, Element, HorizontalAlignment, Length, Row, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components::buttons, Message},
    IntlString,
};

mod cover_art;
mod downloads;
mod general;
mod naming;
mod playlist;

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    General,
    Naming,
    Art,
    Playlist,
    Downloads,
}

/// Renderable views for Settings sections
#[derive(Debug)]
pub enum View {
    General(general::State),
    Naming(naming::State),
    Art(cover_art::State),
    Playlist(playlist::State),
    Downloads(downloads::State),
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
            Self::Art(state) => state.view(settings, intl),
            Self::Playlist(state) => state.view(settings, intl),
            Self::Downloads(state) => state.view(settings, intl),
        }
    }
}

#[derive(Debug, Default)]
struct Sections {
    general: button::State,
    naming: button::State,
    art: button::State,
    playlist: button::State,
    downloads: button::State,
}

impl Sections {
    fn view<'a>(&'a mut self, intl: &'a IntlString) -> Container<'a, Message> {
        macro_rules! section_button {
            ($state: ident, $label: ident, $message: path) => {
                buttons::button(
                    &mut self.$state,
                    &intl.$label,
                    Some(HorizontalAlignment::Left),
                )
                .width(Length::Fill)
                .on_press($message.into());
            };
        }

        // view select buttons
        let general = section_button!(general, general, SettingsMessage::General);
        let naming = section_button!(naming, naming_and_tags, SettingsMessage::Naming);
        let art = section_button!(art, cover_art, SettingsMessage::Art);
        let playlist = section_button!(playlist, playlist, SettingsMessage::Playlist);
        let downloads = section_button!(downloads, downloads, SettingsMessage::Downloads);

        Container::new(
            Column::new()
                .spacing(5)
                .padding(4)
                .push(general)
                .push(naming)
                .push(art)
                .push(playlist)
                .push(downloads)
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

    pub fn update(&mut self, message: SettingsMessage) {
        match message {
            SettingsMessage::General => self.current_view = View::General(Default::default()),
            SettingsMessage::Naming => self.current_view = View::Naming(Default::default()),
            SettingsMessage::Art => self.current_view = View::Art(Default::default()),
            SettingsMessage::Playlist => self.current_view = View::Playlist(Default::default()),
            SettingsMessage::Downloads => self.current_view = View::Downloads(Default::default()),
        }
    }
}
