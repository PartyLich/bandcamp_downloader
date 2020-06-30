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
