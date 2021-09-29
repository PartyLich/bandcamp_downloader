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

#[cfg(test)]
mod test {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn updates_album_artist() {
        let msg = "should update the album artist";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        update_album_artist(&mut tag, expected, EditAction::Modify);
        let actual = tag.album_artist().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the album artist unchanged";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        tag.set_album_artist(expected);
        update_album_artist(&mut tag, "Wombo Combo", EditAction::Skip);
        let actual = tag.album_artist().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the album artist";
        let expected = None;
        let mut tag = id3::Tag::new();
        tag.set_album_artist("Foo Mando");
        update_album_artist(&mut tag, "Wombo Combo", EditAction::Empty);
        let actual = tag.album_artist();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn updates_album_title() {
        let msg = "should update the album title";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        update_album_title(&mut tag, expected, EditAction::Modify);
        let actual = tag.album().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the album title unchanged";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        tag.set_album(expected);
        update_album_title(&mut tag, "Wombo Combo", EditAction::Skip);
        let actual = tag.album().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the album title";
        let expected = None;
        let mut tag = id3::Tag::new();
        tag.set_album("Foo Mando");
        update_album_title(&mut tag, "Wombo Combo", EditAction::Empty);
        let actual = tag.album();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn updates_album_date() {
        let year = 2021;
        let expected = id3::Timestamp {
            year,
            month: Some(4),
            day: Some(2),
            hour: None,
            minute: None,
            second: None,
        };
        let release_date = Utc
            .datetime_from_str("02 Apr 2021 00:00:00 +0000", "%d %b %Y %T %z")
            .unwrap();

        let msg = "should update the album date";
        let mut tag = id3::Tag::new();
        update_album_date(&mut tag, &release_date, EditAction::Modify);
        let actual = tag.date_released().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the album date unchanged";
        let mut tag = id3::Tag::new();
        tag.set_date_released(expected);
        tag.set_year(year);
        let other_date = Utc
            .datetime_from_str("20 Mar 2021 00:00:00 +0000", "%d %b %Y %T %z")
            .unwrap();
        update_album_date(&mut tag, &other_date, EditAction::Skip);
        let actual = tag.date_released().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the album date";
        let mut tag = id3::Tag::new();
        tag.set_date_released(expected);
        update_album_date(&mut tag, &other_date, EditAction::Empty);
        let actual = tag.date_released();
        assert!(actual.is_none(), "{}", msg);
    }

    #[test]
    fn updates_artist() {
        let msg = "should update the artist";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        update_artist(&mut tag, expected, EditAction::Modify);
        let actual = tag.artist().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the artist unchanged";
        let expected = "The Foobars";
        let mut tag = id3::Tag::new();
        tag.set_artist(expected);
        update_artist(&mut tag, "Wombo Combo", EditAction::Skip);
        let actual = tag.artist().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the artist";
        let expected = None;
        let mut tag = id3::Tag::new();
        tag.set_artist("Foo Mando");
        update_artist(&mut tag, "Wombo Combo", EditAction::Empty);
        let actual = tag.artist();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn updates_track_num() {
        let msg = "should update the track number";
        let expected = 42;
        let mut tag = id3::Tag::new();
        update_track_number(&mut tag, expected, EditAction::Modify);
        let actual = tag.track().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the track number unchanged";
        let expected = 42;
        let mut tag = id3::Tag::new();
        let track = 1;
        tag.set_track(expected);
        update_track_number(&mut tag, track, EditAction::Skip);
        let actual = tag.track().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the track number";
        let expected = None;
        let mut tag = id3::Tag::new();
        let track = 42;
        tag.set_track(4);
        update_track_number(&mut tag, track, EditAction::Empty);
        let actual = tag.track();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn updates_title() {
        let msg = "should update the title";
        let expected = "foo";
        let mut tag = id3::Tag::new();
        update_track_title(&mut tag, expected, EditAction::Modify);
        let actual = tag.title().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the title unchanged";
        let expected = "foo";
        let mut tag = id3::Tag::new();
        let title = "bar";
        tag.set_title(expected);
        update_track_title(&mut tag, title, EditAction::Skip);
        let actual = tag.title().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the title";
        let expected = None;
        let mut tag = id3::Tag::new();
        let title = "bar";
        tag.set_title("foo");
        update_track_title(&mut tag, title, EditAction::Empty);
        let actual = tag.title();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn updates_comments() {
        let msg = "should update the comments";
        let expected = "Support the artists you enjoy.";
        let mut tag = id3::Tag::new();
        update_comments(&mut tag, EditAction::Modify);
        let comments: Vec<_> = tag.comments().collect();
        let actual = &comments[0].text;
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should leave the comments unchanged";
        let expected = "foo";
        let mut tag = id3::Tag::new();
        tag.add_comment(id3::frame::Comment {
            lang: "eng".to_string(),
            description: "".to_string(),
            text: expected.to_string(),
        });
        update_comments(&mut tag, EditAction::Skip);
        let comments: Vec<_> = tag.comments().collect();
        let actual = &comments[0].text;
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should remove the comments";
        let mut tag = id3::Tag::new();
        tag.add_comment(id3::frame::Comment {
            lang: "eng".to_string(),
            description: "".to_string(),
            text: expected.to_string(),
        });
        update_comments(&mut tag, EditAction::Empty);
        let comments: Vec<_> = tag.comments().collect();
        let actual = comments.len();
        assert_eq!(actual, 0, "{}", msg);
    }

    // TODO: add update_lyrics tests
}
