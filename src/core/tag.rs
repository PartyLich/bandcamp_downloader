//! id3 tagging utility functions
use std::convert::TryFrom;

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

/// id3 tag modification modes
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EditAction {
    /// Save or update the field in the tag
    Modify,
    /// Empty the field in the tag
    Empty,
    /// Do not modify the tag
    Skip,
}

/// Updates the [`id3::Tag`] in place with the specified album artist based on the specified [`EditAction`].
pub fn update_album_artist(tag: &mut id3::Tag, album_artist: &str, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_album_artist(),
        EditAction::Modify => tag.set_album_artist(album_artist),
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified album title based on the specified
/// [`EditAction`].
pub fn update_album_title(tag: &mut id3::Tag, album_title: &str, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_album(),
        EditAction::Modify => tag.set_album(album_title),
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified album date based on the specified
/// [`EditAction`].
pub fn update_album_date(tag: &mut id3::Tag, album_date: &DateTime<Utc>, edit_action: EditAction) {
    let year = album_date.year();
    let month = u8::try_from(album_date.month()).ok();
    let day = u8::try_from(album_date.day()).ok();

    match edit_action {
        EditAction::Empty => {
            tag.remove_year();
            tag.remove_date_released();
        }
        EditAction::Modify => {
            tag.set_date_released(id3::Timestamp {
                year,
                month,
                day,
                hour: None,
                minute: None,
                second: None,
            });
            tag.set_year(year);
        }

        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified artist based on the specified
/// [`EditAction`].
pub fn update_artist(tag: &mut id3::Tag, artist: &str, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_artist(),
        EditAction::Modify => tag.set_artist(artist),
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place by changing the comments based on the specified [`EditAction`].
pub fn update_comments(tag: &mut id3::Tag, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_comment(None, None),
        EditAction::Modify => {
            tag.add_comment(id3::frame::Comment {
                lang: "eng".to_string(),
                description: "".to_string(),
                text: "Support the artists you enjoy.".to_string(),
            });
        }
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified lyrics based on the specified
/// [`EditAction`].
pub fn update_track_lyrics(
    tag: &mut id3::Tag,
    track_lyrics: &Option<String>,
    edit_action: EditAction,
) {
    match edit_action {
        EditAction::Empty => tag.remove_all_lyrics(),
        EditAction::Modify => {
            if let Some(lyrics) = track_lyrics {
                tag.add_lyrics(id3::frame::Lyrics {
                    lang: String::default(),
                    description: String::default(),
                    text: lyrics.to_string(),
                })
            }
        }
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified track number based on the specified
/// [`EditAction`].
pub fn update_track_number(tag: &mut id3::Tag, track_number: u32, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_track(),
        EditAction::Modify => tag.set_track(track_number),
        // tag.set_total_tracks(album.tracks.len() as u32);
        EditAction::Skip => {}
    }
}

/// Updates the [`id3::Tag`] in place with the specified track title based on the specified
/// [`EditAction`].
pub fn update_track_title(tag: &mut id3::Tag, track_title: &str, edit_action: EditAction) {
    match edit_action {
        EditAction::Empty => tag.remove_title(),
        EditAction::Modify => tag.set_title(track_title),
        EditAction::Skip => {}
    }
}
