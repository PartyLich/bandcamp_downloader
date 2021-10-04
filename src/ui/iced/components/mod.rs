use std::collections::HashSet;

use iced::{Checkbox, Length, ProgressBar, Row, Text};

use crate::{
    ui::iced::{style, Message, SettingType},
    ui::{IntlString, Progress},
};

use controls::controls;
use event_log::event_log;
use file_format_input::filename_format;
use styled_pick_list::styled_pick_list;
use styled_text as StyledText;
use styled_text_input::save_dir;
use url_section::{url_section, UrlState};

pub use entry::*;

mod buttons;
mod controls;
mod entry;
mod event_log;
mod file_format_input;
pub mod main_view;
pub mod settings_view;
mod styled_pick_list;
mod styled_text_input;
mod url_section;

const TEXT_SIZE: u16 = 14;

/// Returns a styled Text element
fn styled_text<T: Into<String>>(text: T) -> Text {
    Text::new(text).size(TEXT_SIZE)
}

pub fn checkbox_row<'a, F>(state: bool, label: &str, msg: F) -> Row<'a, Message>
where
    F: 'static + Fn(bool) -> Message,
{
    let checkbox = Checkbox::new(state, label, msg)
        .size(16)
        .text_size(TEXT_SIZE);

    Row::new().push(checkbox)
}

/// Checkbox for toggling discography download option
pub fn discography_checkbox<'a>(state: bool, intl: &IntlString) -> Row<'a, Message> {
    checkbox_row(state, &intl.discography_checkbox, |a| {
        SettingType::Discography(a).into()
    })
}

/// Creates a ProgressBar to display the completion percentage calculated from a set of Progress
/// events
pub fn download_progress_bar(files: &HashSet<Progress>) -> ProgressBar {
    let (downloaded, total_size) = files.iter().fold((0.0, 0.0), |(done, total), file| {
        (done + file.complete as f32, total + file.total as f32)
    });
    let download_progress = (downloaded / total_size) * 100.0;

    ProgressBar::new(0.0..=100.0, download_progress)
        .height(Length::Units(18))
        .style(style::Theme::Light)
}
