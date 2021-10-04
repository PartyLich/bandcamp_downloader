//! Naming and Tag settings view
use iced::{pick_list, text_input, Align, Column, Element, Length, PickList, Row, Space};

use crate::core::EditAction;
use crate::settings::UserSettings;
use crate::ui::{
    iced::{components, Message, SettingType},
    IntlString,
};

/// Naming and Tag settings view state
#[derive(Debug, Default)]
pub struct State {
    filename_input: text_input::State,
    tag_album_artist: pick_list::State<EditAction>,
    tag_album_title: pick_list::State<EditAction>,
    tag_artist: pick_list::State<EditAction>,
    tag_comments: pick_list::State<EditAction>,
    tag_lyrics: pick_list::State<EditAction>,
    tag_track_number: pick_list::State<EditAction>,
    tag_track_title: pick_list::State<EditAction>,
    tag_date: pick_list::State<EditAction>,
}

impl State {
    pub fn view(&mut self, settings: &UserSettings, intl: &IntlString) -> Element<Message> {
        let filename_format =
            components::filename_format(&mut self.filename_input, &settings.file_name_format, intl);
        let modify_tags_checkbox = components::checkbox_row(
            settings.modify_tags,
            &intl.modify_tags_checkbox,
            Message::ModifyTagsToggled,
        );
        let art_in_folder_checkbox = components::checkbox_row(
            settings.save_cover_art_in_folder,
            &intl.art_in_folder,
            Message::ArtInFolderToggled,
        );
        let art_in_tags_checkbox = components::checkbox_row(
            settings.save_cover_art_in_tags,
            &intl.art_in_tags,
            Message::ArtInTagsToggled,
        );

        Column::new()
            .spacing(5)
            .height(Length::Fill)
            .width(Length::FillPortion(3))
            .push(filename_format)
            .push(modify_tags_checkbox)
            .push(album_title_picker(
                &mut self.tag_album_title,
                &settings.tag_album_title,
                intl,
            ))
            .push(album_artist_picker(
                &mut self.tag_album_artist,
                &settings.tag_album_artist,
                intl,
            ))
            .push(date_picker(&mut self.tag_date, &settings.tag_year, intl))
            .push(artist_picker(
                &mut self.tag_artist,
                &settings.tag_artist,
                intl,
            ))
            .push(track_number_picker(
                &mut self.tag_track_number,
                &settings.tag_track_number,
                intl,
            ))
            .push(track_title_picker(
                &mut self.tag_track_title,
                &settings.tag_track_title,
                intl,
            ))
            .push(lyrics_picker(
                &mut self.tag_lyrics,
                &settings.tag_lyrics,
                intl,
            ))
            .push(comments_picker(
                &mut self.tag_comments,
                &settings.tag_comments,
                intl,
            ))
            // TODO: move to art view
            .push(art_in_folder_checkbox)
            .push(art_in_tags_checkbox)
            //
            .push(Space::with_height(Length::Fill))
            .into()
    }
}

/// Pick list with label
fn picker_row<'a>(
    pick_list: PickList<'a, EditAction, Message>,
    label: impl Into<String>,
) -> Element<'a, Message> {
    let label = components::StyledText(format!("{}:", label.into())).width(Length::Units(100));

    Row::new()
        .spacing(5)
        .align_items(Align::Center)
        .push(label)
        .push(pick_list)
        .into()
}

/// Generate an EditAction pick list widget fn
macro_rules! edit_action_picker {
    ($name: ident, $message: expr, $intl_field: ident) => {
        fn $name<'a>(
            pick_list_state: &'a mut pick_list::State<EditAction>,
            edit_action: &EditAction,
            intl: &IntlString,
        ) -> Element<'a, Message> {
            let pick_list = components::styled_pick_list(
                pick_list_state,
                &EditAction::ALL[..],
                Some(*edit_action),
                |a| $message(a).into(),
            );

            picker_row(pick_list.into(), &intl.$intl_field)
        }
    };
}

edit_action_picker!(
    album_artist_picker,
    SettingType::TagAlbumArtist,
    album_artist
);
edit_action_picker!(album_title_picker, SettingType::TagAlbumTitle, album_title);
edit_action_picker!(artist_picker, SettingType::TagArtist, artist);
edit_action_picker!(comments_picker, SettingType::TagComments, comments);
edit_action_picker!(lyrics_picker, SettingType::TagLyrics, lyrics);
edit_action_picker!(
    track_number_picker,
    SettingType::TagTrackNumber,
    track_number
);
edit_action_picker!(track_title_picker, SettingType::TagTrackTitle, track_title);
edit_action_picker!(date_picker, SettingType::TagYear, album_date);
