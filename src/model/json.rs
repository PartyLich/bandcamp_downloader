use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

/// Convert bandcamp datetime string format to a chrono DateTime object
fn datetime_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &str = "%d %b %Y %T %z";
    let s = String::deserialize(deserializer)?;
    let s = s.replace("GMT", "+0000");
    println!("{}", s);
    Utc.datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonMp3File {
    // Some tracks do not have their URL filled on some albums (pre-release...)
    #[serde(rename = "mp3-128")]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonTrack {
    #[serde(rename = "duration")]
    pub duration: f32,

    #[serde(rename = "file")]
    pub file: JsonMp3File,

    #[serde(rename = "lyrics")]
    pub lyrics: Option<String>,

    #[serde(rename = "track_num")]
    pub number: Option<u32>,

    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonAlbumData {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "release_date")]
    #[serde(deserialize_with = "datetime_from_str")]
    pub release_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonAlbum {
    #[serde(rename = "current")]
    pub album_data: JsonAlbumData,

    #[serde(rename = "art_id")]
    pub art_id: Option<usize>,

    #[serde(rename = "artist")]
    pub artist: String,

    #[serde(rename = "trackinfo")]
    pub tracks: Vec<JsonTrack>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_jsonmp3() {
        let test_str = r#"{
            "mp3-128": "foo.bar"
        }"#;
        let actual = serde_json::from_str::<JsonMp3File>(test_str).unwrap();
        let expected = JsonMp3File {
            url: Some(String::from("foo.bar")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize_json_track() {
        let test_str = r#"{"video_mobile_url":null,"album_preorder":false,"file":{"mp3-128":"https://t4.bcbits.com/stream/f19f73f3022113d2e0362cc017a2640f/mp3-128/3291645056?p=0&ts=1593226703&t=c000e57bbab5d336049099dbdad88ee289a8706a&token=1593226703_7712a8c4b48e9e7c5d5658b30794b2bf02cf9392"},"encoding_pending":null,"lyrics":null,"has_free_download":null,"streaming":1,"video_poster_url":null,"unreleased_track":false,"play_count":null,"is_draft":false,"free_album_download":false,"video_caption":null,"title_link":"/track/sleepover","is_capped":null,"sizeof_lyrics":0,"video_featured":null,"has_lyrics":false,"video_source_type":null,"private":null,"title":"Sleepover","alt_link":null,"has_info":false,"track_id":3291645056,"track_license_id":null,"video_source_id":null,"track_num":1,"encodings_id":3274042554,"id":3291645056,"encoding_error":null,"video_id":null,"duration":157.204,"is_downloadable":true,"license_type":1}"#;
        let actual: JsonTrack = serde_json::from_str(test_str).unwrap();
        let expected = JsonTrack {
            duration:  157.204,
            title: String::from("Sleepover"),
            number: Some(1),
            lyrics: None,
            file: JsonMp3File {
                url:Some(String::from("https://t4.bcbits.com/stream/f19f73f3022113d2e0362cc017a2640f/mp3-128/3291645056?p=0&ts=1593226703&t=c000e57bbab5d336049099dbdad88ee289a8706a&token=1593226703_7712a8c4b48e9e7c5d5658b30794b2bf02cf9392")),
            }
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize_json_album_data() {
        let test_str = r#"{"type":"album","purchase_title":null,"release_date":"06 Oct 2017 00:00:00 GMT","require_email":null,"auto_repriced":null,"require_email_0":null,"audit":0,"upc":null,"art_id":107831136,"band_id":4055192856,"artist":"submerse","mod_date":"28 Aug 2018 09:56:45 GMT","killed":null,"selling_band_id":4055192856,"set_price":7.0,"download_pref":2,"publish_date":"03 Jul 2017 14:11:10 GMT","new_date":"03 Jul 2017 13:51:19 GMT","download_desc_id":null,"minimum_price_nonzero":10.0,"private":null,"title":"Are You Anywhere","about":"Late night music from the back seat. ‘Are You Anywhere‘, sophomore album from Tokyo based UK native submerse blends early 90’s slow-jams and instrumental hip hop wrapped in smooth DX7 keys hybridized with his own unique sound. ‘Are You Anywhere’ is the second full length release from submerse on Project: Mooncircle with features from fellow Tokyo based beat maker fitz ambro$e.\r\n\r\n‘Are You Anywhere‘ comes out worldwide on limited green colored vinyl (including download code) & limited edition CD via Perfect Touch in Japan.\r\n\r\nMore information: http://projectmooncircle.com/releases/submerse-are-you-anywhere/","id":366983914,"featured_track_id":1408359341,"purchase_url":null,"new_desc_format":1,"minimum_price":10.0,"credits":null,"is_set_price":null}
"#;
        let actual: JsonAlbumData = serde_json::from_str(test_str).unwrap();
        let expected = JsonAlbumData {
            title: String::from("Are You Anywhere"),
            release_date: Utc
                .datetime_from_str("06 Oct 2017 00:00:00 +0000", "%d %b %Y %T %z")
                .unwrap(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize_json_album() {
        let test_str = r#"{
    "current": {"isrc":null,"file_name":null,"new_desc_format":1,"title":"Final Lap","license_type":1,"album_id":null,"about":null,"is_set_price":null,"encodings_id":1928142095,"minimum_price":1.0,"require_email":null,"lyrics":null,"credits":null,"pending_encodings_id":null,"release_date":"24 Apr 2020 00:00:00 GMT","auto_repriced":null,"require_email_0":null,"streaming":1,"preorder_download":null,"track_number":null,"private":null,"mod_date":"24 Apr 2020 10:46:44 GMT","audit":0,"id":350943074,"art_id":2129006133,"killed":null,"band_id":1173700968,"artist":null,"set_price":1.0,"publish_date":"24 Apr 2020 10:46:44 GMT","type":"track","download_desc_id":null,"minimum_price_nonzero":1.0,"selling_band_id":1173700968,"download_pref":2,"new_date":"24 Apr 2020 10:46:40 GMT"},
    "hasAudio": true,
    "album_is_preorder": null,
    "album_release_date": null,
    "art_id": 2129006133,
    "trackinfo": [{"track_license_id":null,"video_source_id":null,"track_num":null,"title":"Final Lap","encoding_error":null,"video_id":null,"is_downloadable":true,"license_type":1,"track_id":350943074,"video_mobile_url":null,"album_preorder":false,"encodings_id":1928142095,"encoding_pending":null,"lyrics":null,"duration":311.327,"has_free_download":null,"video_poster_url":null,"unreleased_track":false,"play_count":0,"is_draft":false,"free_album_download":false,"streaming":1,"private":null,"video_caption":null,"title_link":"/track/final-lap","is_capped":false,"sizeof_lyrics":0,"id":350943074,"video_featured":null,"has_lyrics":false,"video_source_type":null,"file":{"mp3-128":"https://t4.bcbits.com/stream/8e264c1615dca0ab965f6e3b320ea9da/mp3-128/350943074?p=0&ts=1593267872&t=925e5bd0f7f97122898d7e97dc80e17a3effa022&token=1593267872_b7f87b378480f8743677ad5f9adedd8bcfc33dd9"},"alt_link":null,"has_info":false}],
    "playing_from": "track page",
    "packages": null,
    "album_url": null,
    "url": "http://theracers.bandcamp.com/track/final-lap",
    "defaultPrice": 1.0,
    "freeDownloadPage": null,
    "FREE": 1,
    "PAID": 2,
    "artist": "The Racers",
    "item_type": "track",
    "id": 350943074,
    "last_subscription_item": null,
    "has_discounts": null,
    "is_bonus": null,
    "play_cap_data": {"streaming_limits_enabled":true,"streaming_limit":3},
    "client_id_sig": null,
    "is_purchased": null,
    "items_purchased": null,
    "is_private_stream": null,
    "is_band_member": null,
    "licensed_version_ids": null,
    "package_associated_license_id": null
}"#;
        let actual: JsonAlbum = serde_json::from_str(test_str).unwrap();
        let expected = JsonAlbum {
            artist: String::from("The Racers"),
            art_id: Some(2129006133),
            album_data: JsonAlbumData {
                title: String::from("Final Lap"),
                release_date: Utc
                    .datetime_from_str("24 Apr 2020 00:00:00 +0000", "%d %b %Y %T %z")
                    .unwrap(),
            },
            tracks: vec![JsonTrack {
                duration:  311.327,
                title: String::from("Final Lap"),
                number: None,
                lyrics: None,
                file: JsonMp3File {
                    url: Some(String::from("https://t4.bcbits.com/stream/8e264c1615dca0ab965f6e3b320ea9da/mp3-128/350943074?p=0&ts=1593267872&t=925e5bd0f7f97122898d7e97dc80e17a3effa022&token=1593267872_b7f87b378480f8743677ad5f9adedd8bcfc33dd9"))
                }
            }],
        };
        assert_eq!(actual, expected);
    }
}
