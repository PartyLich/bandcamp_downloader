//! Playlist settings view
use iced::{pick_list, text_input, Align, Column, Element, Length, Row, Space};

use crate::settings::{PlaylistFormat, UserSettings};
use crate::ui::{
    iced::{
        components::{self, indent, labeled_input},
        Message, SettingType,
    },
    IntlString,
};

labeled_input!(
    #[doc = "Playlist filename format input"]
    filename_input,
    filename_format,
    art_input_placeholder,
    SettingType::PlaylistFilename
);

fn format_picker<'a>(
    pick_list_state: &'a mut pick_list::State<PlaylistFormat>,
    selected_theme: &PlaylistFormat,
    intl: &IntlString,
) -> Element<'a, Message> {
    let label = components::StyledText(format!("{}:", &intl.playlist_format));

    let pick_list = components::styled_pick_list(
        pick_list_state,
        &PlaylistFormat::ALL[..],
        Some(*selected_theme),
        |a| SettingType::PlaylistFormat(a).into(),
    );

    Row::new()
        .spacing(5)
        .align_items(Align::Center)
        .push(label)
        .push(pick_list)
        .into()
}

/// Playlist settings view state
#[derive(Debug, Default)]
pub struct State {
    format_list: pick_list::State<PlaylistFormat>,
    filename_input: text_input::State,
}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        const INDENT: u16 = 30;
        macro_rules! checkbox {
            ($setting: ident, $intl_field: ident, $message: path) => {
                components::checkbox_row(settings.$setting, &intl.$intl_field, |a| {
                    $message(a).into()
                })
            };
        }

        let playlist_checkbox = checkbox!(
            create_playlist,
            create_playlist,
            SettingType::CreatePlaylist
        );
        let format_list = format_picker(&mut self.format_list, &settings.playlist_format, intl);
        let filename_format = filename_input(
            &mut self.filename_input,
            &settings.playlist_file_name_format,
            intl,
        );

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(playlist_checkbox)
            .push(indent(INDENT).push(format_list))
            .push(indent(INDENT).push(filename_format))
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
