use iced::{button, Element, Length, Row, Space};

use crate::ui::{
    iced::{style, Message},
    IntlString,
};

use super::buttons;

pub fn controls<'a>(
    download_state: &'a mut button::State,
    cancel_state: &'a mut button::State,
    settings_state: &'a mut button::State,
    intl: &IntlString,
) -> Element<'a, Message> {
    let start_download = buttons::download(download_state, intl);
    let settings = buttons::settings(settings_state, intl);
    let cancel = buttons::cancel(cancel_state, intl);

    Row::new()
        .push(Space::new(Length::Fill, Length::Fill))
        .push(start_download)
        .push(cancel)
        .push(settings)
        .spacing(5)
        .into()
}
