//! Download settings view
use iced::{Column, Element, Length, Space};

use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message, SettingType},
    IntlString,
};

/// Download settings view state
#[derive(Debug, Default)]
pub struct State {}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        macro_rules! checkbox {
            ($setting: ident, $intl_field: ident, $message: path) => {
                components::checkbox_row(settings.$setting, &intl.$intl_field, |a| {
                    $message(a).into()
                })
            };
        }

        let serial_checkbox = checkbox!(
            download_one_album_at_a_time,
            download_serial,
            SettingType::DownloadSerial
        );

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(serial_checkbox)
            .push(Space::with_height(Length::Fill))
            .into()
    }
}
