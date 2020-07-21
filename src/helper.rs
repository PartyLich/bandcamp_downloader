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
        let raw_html = r#" var TralbumData = {
            foo: "bar"
        };

        var baz = {
            wombo: "combo"
        };"#;

        let expected = r#"{
            "foo": "bar"
        }"#;
        let actual = get_album_data(&raw_html).unwrap();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn gets_album() {
        let raw_html = r#"var TralbumData = {
    // For the curious:
    // http://bandcamp.com/help/audio_basics#steal
    // http://bandcamp.com/terms_of_use
    current: {"isrc":null,"file_name":null,"title":"Final Lap","album_id":null,"about":null,"encodings_id":1928142095,"auto_repriced":null,"minimum_price":1.0,"lyrics":null,"credits":null,"set_price":1.0,"publish_date":"24 Apr 2020 10:46:44 GMT","release_date":"24 Apr 2020 00:00:00 GMT","download_desc_id":null,"minimum_price_nonzero":1.0,"audit":0,"streaming":1,"preorder_download":null,"track_number":null,"private":null,"new_desc_format":1,"mod_date":"24 Apr 2020 10:46:44 GMT","id":350943074,"license_type":1,"art_id":2129006133,"is_set_price":null,"killed":null,"band_id":1173700968,"artist":null,"require_email":null,"new_date":"24 Apr 2020 10:46:40 GMT","type":"track","pending_encodings_id":null,"selling_band_id":1173700968,"require_email_0":null,"download_pref":2},
    hasAudio: true,
    album_is_preorder: null,
    album_release_date: null,
    art_id: 2129006133,
    trackinfo: [{"play_count":0,"is_draft":false,"free_album_download":false,"title":"Final Lap","video_caption":null,"title_link":"/track/final-lap","track_id":350943074,"is_capped":false,"sizeof_lyrics":0,"encodings_id":1928142095,"video_featured":null,"lyrics":null,"duration":311.327,"has_lyrics":false,"video_source_type":null,"alt_link":null,"streaming":1,"has_info":false,"private":null,"track_license_id":null,"video_source_id":null,"track_num":null,"encoding_error":null,"video_id":null,"is_downloadable":true,"license_type":1,"id":350943074,"video_mobile_url":null,"album_preorder":false,"encoding_pending":null,"has_free_download":null,"file":{"mp3-128":"https://t4.bcbits.com/stream/8e264c1615dca0ab965f6e3b320ea9da/mp3-128/350943074?p=0&ts=1593190558&t=8419ee8e51afb4b6ff82c17a6ada652759a67e61&token=1593190558_506542a3276203145b01723422d77b84d02fe0b2"},"video_poster_url":null,"unreleased_track":false}, {"track_license_id":null,"encoding_error":null,"is_downloadable":true,"license_type":1,"file":{"mp3-128":"https://t4.bcbits.com/stream/db1af2e5f68f45ba5647560e4e81b2ad/mp3-128/120295245?p=0&ts=1595393447&t=645c963dc20bd86ef5b86c569fde8e01b02c5834&token=1595393447_8d168bda6e5bee441f98d072e6ca39a0f647c369"},"video_mobile_url":null,"album_preorder":false,"encoding_pending":null,"has_free_download":null,"video_poster_url":null,"unreleased_track":false,"track_id":120295245,"play_count":null,"is_draft":false,"free_album_download":false,"encodings_id":4242834731,"title":"Glancing Blows // Nights","video_caption":null,"duration":345.882,"title_link":"/track/glancing-blows-nights","is_capped":null,"sizeof_lyrics":0,"video_featured":null,"lyrics":null,"streaming":1,"has_lyrics":false,"private":null,"video_id":null,"video_source_type":null,"alt_link":null,"has_info":false,"id":120295245,"video_source_id":null,"track_num":6}],
    playing_from: "track page",
    packages: null,
    album_url: null,
    url: "http://theracers.bandcamp.com/track/final-lap",
    defaultPrice: 1.0,
    freeDownloadPage: null,
    FREE: 1,
    PAID: 2,
    artist: "The Racers",
    item_type: "track", // xxx: note - don't internationalize this variable
    id: 350943074,
    last_subscription_item: null,
    has_discounts: null,
    is_bonus: null,
    play_cap_data: {"streaming_limits_enabled":true,"streaming_limit":3},
    client_id_sig: "J+6CqXlrQNrrHlOU00LqGujGx/I=",
    is_purchased: null,
    items_purchased: null,
    is_private_stream: null,
    is_band_member: null,
    licensed_version_ids: null,
    package_associated_license_id: null
};

var PaymentData = {
    paymentType: null,
    paymentDownloadPage: null
};
"#;

        let msg = "builds Album object from html";
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
                mp3_url: String::from("https://t4.bcbits.com/stream/8e264c1615dca0ab965f6e3b320ea9da/mp3-128/350943074?p=0&ts=1593190558&t=8419ee8e51afb4b6ff82c17a6ada652759a67e61&token=1593190558_506542a3276203145b01723422d77b84d02fe0b2"),
                number: 1,
                path: String::from("/home/partylich/music/test/The Racers/2020 - Final Lap/01 - Final Lap.mp3"),
                title: String::from("Final Lap")
            },
            Track {
                duration: 345.822,
                lyrics: None,
                mp3_url: String::from("https://t4.bcbits.com/stream/db1af2e5f68f45ba5647560e4e81b2ad/mp3-128/120295245?p=0&ts=1595393447&t=645c963dc20bd86ef5b86c569fde8e01b02c5834&token=1595393447_8d168bda6e5bee441f98d072e6ca39a0f647c369"),
                number: 1,
                path: String::from("/home/partylich/music/test/The Racers/2020 - Final Lap/06 - Glancing Blows  Nights.mp3"),
                title: String::from("Glancing Blows // Nights")
            }],
        };
        let save_dir = "/home/partylich/music/test/{artist}/{year} - {album}";
        let actual = get_album(&raw_html, save_dir).unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn gets_albums_url() {
        let raw_html = r#"
        var band_url = "http://projectmooncircle.bandcamp.com";

        <ol class="editable-grid music-grid columns-4   public" data-edit-callback="/music_reorder">
            <li data-item-id="album-3655789805" data-band-id="4055192856" class="music-grid-item square first-four " data-bind="css: {'featured': featured()}">
    <a href="/album/silent-opera">
        <div class="art">
                <img src="https://f4.bcbits.com/img/a2796464951_2.jpg" alt="">
        </div>
        <p class="title">
            Silent Opera
                <br><span class="artist-override">
                Long Arm
                </span>
        </p>
    </a>
</li>
            <li data-item-id="album-1556143258" data-band-id="4055192856" class="music-grid-item square
    " data-bind="css: {'featured': featured()}">
    <a href="/album/audio-alchemy">
        <div class="art">
                <img class="lazy" src="/img/0.gif" data-original="https://f4.bcbits.com/img/a2189627774_2.jpg" alt="">
        </div>
        <p class="title">
            Audio Alchemy
        </p>
    </a>
</li>
            <li data-item-id="album-1965508264" data-band-id="4055192856" class="music-grid-item square
    " data-bind="css: {'featured': featured()}">
    <a href="/album/the-lucid-effect">
        <div class="art">
                <img class="lazy" src="/img/0.gif" data-original="https://f4.bcbits.com/img/a3631959610_2.jpg" alt="">
        </div>
        <p class="title">
            The Lucid Effect
                <br><span class="artist-override">
                40 Winks
                </span>
        </p>
    </a>
</li>
</ol>
        "#;
        let expected = vec![
            "http://projectmooncircle.bandcamp.com/album/audio-alchemy",
            "http://projectmooncircle.bandcamp.com/album/silent-opera",
            "http://projectmooncircle.bandcamp.com/album/the-lucid-effect",
        ];
        let mut actual = get_albums_url(raw_html).unwrap();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn format_filename() {
        let expected = "Foo_________Bar";
        let actual = sanitize_file_name(r#"Foo?*/\|<>:"Bar   ..."#);
        assert_eq!(actual, expected);
    }
}
