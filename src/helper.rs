use std::collections::HashSet;

use futures::channel::mpsc;
use regex::Regex;

use crate::{
    error::Error,
    model::{Album, JsonAlbum},
    ui::{LogLevel, Message},
    Result, ALBUM_RE, BAND_RE,
};

fn log_channel<T: ToString>(mut sender: mpsc::Sender<Message>, level: LogLevel, msg: T) {
    sender
        .try_send(Message::Log(msg.to_string(), level))
        .expect("Failed to send message");
}

pub fn log_info<T: ToString>(sender: mpsc::Sender<Message>, msg: T) {
    log_channel(sender, LogLevel::Info, msg)
}

pub fn log_warn<T: ToString>(sender: mpsc::Sender<Message>, msg: T) {
    log_channel(sender, LogLevel::Warn, msg)
}

pub fn log_error<T: ToString>(sender: mpsc::Sender<Message>, msg: T) {
    log_channel(sender, LogLevel::Error, msg)
}

/// Get the TralbumData content from the page
fn get_album_data(raw_html: &str) -> Result<String> {
    lazy_static! {
        static ref ALBUM_DATA_RE: Regex =
            regex::Regex::new(r"(?s)var TralbumData = (?P<data>\{.*?\});").unwrap();
        static ref COMMENTS_RE: Regex = Regex::new(r"(?m)^\s*?//.*$").unwrap();
        static ref TAIL_COMMENTS_RE: Regex = Regex::new(r"(?m)^(?:.*\s*?,)(\s//.*)$").unwrap();
        static ref QUOTE_KEYS_RE: Regex =
            Regex::new(r"(?m)^(?P<whitespace>\s*)(?P<key>\w*?):").unwrap();
    }

    let album_data = ALBUM_DATA_RE
        .captures(raw_html)
        .ok_or(Error::NoAlbumData)?
        .name("data")
        .unwrap()
        .as_str();

    // using a js object that has unquoted keys, comments, etc
    let album_data = COMMENTS_RE.replace_all(album_data, "");
    let album_data = TAIL_COMMENTS_RE.replace_all(&album_data, "");
    let album_data = QUOTE_KEYS_RE.replace_all(&album_data, r#"${whitespace}"${key}":"#);

    Ok(album_data.to_string())
}

// We're pulling from a javascript object literal, so we need to turn it into valid JSON before we
// can deserialize it.
// In trackinfo property, we have for instance:
//     url: "http://verbalclick.bandcamp.com" + "/album/404"
// -> Remove the " + "
fn fix_json(album_data: &str) -> String {
    lazy_static! {
        static ref JSON_FIX_RE: regex::Regex =
            regex::Regex::new(r#"(?P<root>url: ".+)" \+ "(?P<album>.+",)"#).unwrap();
    }
    let fixed = JSON_FIX_RE.replace(album_data, "${root}${album}");

    fixed.to_string()
}

/// Retrieves the data on the album of the specified Bandcamp page.  Takes the HTML source code of
/// a Bandcamp album page and returns the data on the album of the specified Bandcamp page.
pub fn get_album(raw_html: &str, folder_path: &str) -> Result<Album> {
    // Keep the necessary part of the html only
    // it's a js object literal, which isnt JSON, so we need to adjust it to match the actual
    // spec prior to deserialization
    let album_data = fix_json(raw_html);
    let album_data = get_album_data(&album_data)?;
    // Deserialize JSON
    // TODO serializer interface
    let album = serde_json::from_str::<JsonAlbum>(&album_data)?.into_album(folder_path);

    // TODO lyrics
    // Extract lyrics from album page

    Ok(album)
}

/// Retrieves all the album URLs existing in the provided raw HTML source code of a Bandcamp page.
pub fn get_albums_url(raw_html: &str) -> Result<Vec<String>> {
    // Get artist bandcamp page
    let artist_url = BAND_RE
        .captures(raw_html)
        .ok_or(Error::NoAlbumFound)?
        .name("url")
        .unwrap()
        .as_str();

    // Get albums ("real" albums or track-only pages) relative urls
    let captures = ALBUM_RE.captures_iter(&raw_html);

    let mut album_urls = HashSet::new();
    for cap in captures {
        album_urls.insert(format!(
            "{}{}",
            artist_url,
            cap.name("album_url").unwrap().as_str()
        ));
    }

    if album_urls.is_empty() {
        return Err(Error::NoAlbumFound);
    }
    Ok(album_urls.into_iter().collect())
}

// Windows rules: https://docs.microsoft.com/en-us/windows/desktop/FileIO/naming-a-file
pub fn sanitize_file_name(file_name: &str) -> String {
    lazy_static! {
        static ref TRAIL_DOTS: Regex = regex::Regex::new(r"\.+$").unwrap();
        static ref WHITESPACE: Regex = regex::Regex::new(r"\s+").unwrap();
        static ref RESERVED_CHARS: Regex = regex::Regex::new(r#"[\\/:*?"<>|]"#).unwrap();
    }

    // Replace reserved characters by '_'
    let file_name = RESERVED_CHARS.replace_all(file_name, "_");

    // Remove trailing dot(s)
    let file_name = TRAIL_DOTS.replace(&file_name, "");

    // Replace whitespace(s) by ' '
    let file_name = WHITESPACE.replace_all(&file_name, " ");

    // Remove trailing whitespace
    file_name.trim_end().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Track;
    use chrono::{TimeZone, Utc};

    mod strings;

    #[test]
    fn fixes_json() {
        let raw = r#"url: "http://verbalclick.bandcamp.com" + "/album/404","#;

        let msg = "fixes bad json";
        let expected = r#"url: "http://verbalclick.bandcamp.com/album/404","#;
        let actual = fix_json(raw);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn gets_album_data() {
        let expected = strings::TRALBUM_DATA;
        let actual = get_album_data(strings::TRALBUM_HTML).unwrap();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn gets_album() {
        let msg = "should build Album object from html string";
        let expected = Album {
            artist: String::from("The Racers"),
            artwork_path: String::from(""),
            artwork_temp_path: String::from(""),
            artwork_url: Some(String::from("https://f4.bcbits.com/img/a2129006133_0.jpg")),
            path: String::from("/home/partylich/music/test/The Racers/2020 - Final Lap"),
            playlist_path: String::from(
                "/home/partylich/music/test/The Racers/2020 - Final Lap/2020_Final Lap",
            ),
            title: String::from("Final Lap"),
            release_date: Utc
                .datetime_from_str("24 Apr 2020 00:00:00 +0000", "%d %b %Y %T %z")
                .unwrap(),
            tracks: vec![Track {
                duration: 311.327,
                lyrics: None,
                mp3_url: String::from("https://t4.bcbits.com/stream/8e264c1615dca0ab965f6e3b320ea9da/mp3-128/350943074?p=0&amp;ts=1631806573&amp;t=1c02736b48124fcde7acb2743812134a3e4b25de&amp;token=1631806573_49c0e23c8c2b500fcf206501d703e81527972f5b"),
                number: 1,
                path: String::from("/home/partylich/music/test/The Racers/2020 - Final Lap/01 - Final Lap.mp3"),
                title: String::from("Final Lap")
            },
            ],
        };
        let save_dir = "/home/partylich/music/test/{artist}/{year} - {album}";
        let actual = get_album(strings::TRALBUM_HTML, save_dir).unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn gets_albums_url() {
        let mut expected = vec![
            "https://moter.bandcamp.com/album/moter-ep",
            "https://moter.bandcamp.com/album/last-train-to-synthville",
            "https://moter.bandcamp.com/album/wave-transmission",
            "https://moter.bandcamp.com/album/omegadriver",
            "https://moter.bandcamp.com/album/aerodnmx",
        ];
        let mut actual = get_albums_url(strings::ALBUM_HTML).unwrap();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn format_filename() {
        let msg = "should replace reserved chars with '_'";
        let expected = "Foo_________Bar";
        let actual = sanitize_file_name(r#"Foo?*/\|<>:"Bar   ..."#);
        assert_eq!(actual, expected, "{}", msg);
    }
}
