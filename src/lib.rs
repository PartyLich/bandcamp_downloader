use std::{collections::HashSet, convert::TryFrom, path::Path, sync::Arc};

use chrono::Datelike;
use futures::channel::mpsc;
use futures::future::join_all;
use regex::Regex;
use tokio::{fs, io::AsyncWriteExt, sync::RwLock};

use error::Error;
use model::{Album, Track};
use settings::UserSettings;
use ui::{LogLevel, Message, Progress};

#[macro_use]
extern crate lazy_static;

pub mod core;
mod error;
mod helper;
mod model;
pub mod settings;
pub mod ui;

/// A `Result` alias where the `Err` case is `bandcamp_downloader::Error`.
pub type Result<T> = std::result::Result<T, error::Error>;

lazy_static! {
    /// Band url parsing
    static ref BAND_RE: Regex = Regex::new("band_url = \"(?P<url>.*)\"").unwrap();
    /// album and track url parsing
    static ref ALBUM_RE: Regex = Regex::new("href=\"(?P<album_url>/(album|track)/.*?)\"").unwrap();
}

/// Get text from a url using the reqwest shortcut method
async fn get_url_text(url: &str) -> Result<String> {
    reqwest::get(url).await?.text().await.map_err(From::from)
}

/// Get text from a url using a reqwest Client
async fn client_get_url_text(client: &reqwest::Client, url: &str) -> Result<String> {
    client
        .get(url)
        .send()
        .await?
        .text()
        .await
        .map_err(From::from)
}

/// Get artist "music" bandcamp page (http://artist.bandcamp.com/music)
async fn get_music_page_url(client: &reqwest::Client, url: &str) -> Result<String> {
    // Retrieve URL HTML source code
    let raw_html = match client_get_url_text(client, url).await {
        Ok(res) => res,
        Err(e) => {
            println!("Could not retrieve data for {}", url);
            return Err(e);
        }
    };

    // Get artist "music" bandcamp page (http://artist.bandcamp.com/music)
    let captures = BAND_RE.captures(&raw_html);
    if captures.is_none() {
        println!("No discography could be found on {}. Try to uncheck the \"Download artist discography\" option", url);
        return Err(Error::NoDiscography);
    }

    let music_page_url = captures.unwrap().name("url").unwrap().as_str();
    Ok(format!("{}{}", music_page_url, "/music"))
}

/// Returns the artist's discography from any URL (artist, album, track).
async fn get_disco_urls(client: &reqwest::Client, url: &str) -> Result<Vec<String>> {
    println!("Retrieving artist discography from {}", url);

    // Get artist "music" bandcamp page (http://artist.bandcamp.com/music)
    let music_page_url = match get_music_page_url(client, url).await {
        Ok(res) => res,
        Err(e) => {
            println!("Could not retrieve data for {}", url);
            return Err(e);
        }
    };

    // Retrieve artist "music" page HTML source code
    let raw_html = match client_get_url_text(client, &music_page_url).await {
        Ok(res) => res,
        Err(e) => {
            println!("Could not retrieve data for {}", music_page_url);
            return Err(e);
        }
    };

    let mut albums_urls = Vec::new();
    match helper::get_albums_url(&raw_html) {
        Err(_) => {
            println!("No referred album could be found on {}. Try to uncheck the \"Download artist discography\" option" ,music_page_url);
        }
        Ok(found_albums) => {
            albums_urls.extend(found_albums);
        }
    }

    if albums_urls.is_empty() {
        // This seem to be a one-album artist with no "music" page => URL redirects to the unique album URL
        albums_urls.push(url.to_string());
    }

    Ok(albums_urls)
}

/// Returns all discography lists from a set of URLs (artist, album, track).
async fn get_artist_discography(urls: &HashSet<&str>) -> Vec<String> {
    // TODO: proxy support
    let client = reqwest::Client::new();

    let tasks: Vec<_> = urls
        .iter()
        .map(|url| get_disco_urls(&client, url))
        .collect();

    let results = join_all(tasks).await;
    let albums_urls: HashSet<_> = results.into_iter().flatten().flatten().collect();

    albums_urls.into_iter().collect()
}

/// Returns the albums located at the specified URLs.
async fn get_albums(urls: HashSet<&str>, save_dir: &str) -> Result<Vec<Album>> {
    let client = reqwest::Client::new();

    let tasks = urls.iter().map(|url| {
        let client = &client;
        async move {
            println!("Retrieving album data for {}", url);

            // Retrieve URL HTML source code
            // TODO proxy support
            // TODO cancellation

            let raw_html = match client_get_url_text(&client, url).await {
                Ok(res) => res,
                Err(_) => {
                    println!("Could not retrieve data for {}", url);
                    return None;
                }
            };

            // Get info on album
            let album = match helper::get_album(&raw_html, save_dir) {
                Ok(a) => a,
                Err(_) => {
                    println!("Could not retrieve album info for {}", url);
                    return None;
                }
            };

            if album.tracks.is_empty() {
                println!("No tracks found for {}, album will not be downloaded", url);
                return None;
            }

            Some(album)
        }
    });

    let albums = join_all(tasks).await.into_iter().flatten().collect();

    Ok(albums)
}

/// Barebones http protocol add.
// TODO use regex, more thorough checks
fn prepend_http(url: &str) -> String {
    if !url.starts_with("http") {
        // prepend missing protocol
        format!("http://{}", url)
    } else {
        url.to_string()
    }
}

/// Fetch albums data from the URLs specified when creating this DownloadManager.
pub async fn fetch_urls(urls: &str, discography: bool, save_dir: &str) -> Vec<Album> {
    let retrieve_file_size = false;
    let urls: HashSet<_> = urls.lines().map(prepend_http).collect();
    let urls: HashSet<_> = urls.iter().map(|s| s.as_str()).collect();

    // Get info on albums
    // Get URLs of albums to download
    let albums = if discography {
        println!("collecting discography urls");
        let url_list = get_artist_discography(&urls).await;
        let urls = url_list.iter().map(|s| s.as_str()).collect();

        get_albums(urls, save_dir).await.expect("FIXME")
    } else {
        get_albums(urls, save_dir).await.expect("FIXME")
    };

    albums
}

async fn download_track_stream(
    track: Track,
    allowed_file_size_difference: f32,
    mut sender: mpsc::Sender<Message>,
) -> Result<()> {
    const MAX_TRIES: u32 = 4;

    println!(
        r#"Downloading track "{}" from url: {:?}"#,
        track.title, track.mp3_url
    );
    sender
        .try_send(Message::Log(
            format!(r#"Downloading track "{}""#, track.title,),
            LogLevel::Info,
        ))
        .expect("Failed to send message");

    if Path::new(&track.path).exists() {
        let file_length = fs::metadata(&track.path)
            .await
            .unwrap_or_else(|_| panic!("Unable to stat file {}", track.path))
            .len();

        println!("file already exists {}. Size: {}", &track.path, file_length);
        sender
            .try_send(Message::Log(
                format!("file already exists {}. Size: {}", &track.path, file_length),
                LogLevel::Info,
            ))
            .expect("Failed to send message");
        return Err(Error::Io(String::from("File already exists")));
    }

    let mut tries = 0u32;
    while tries < MAX_TRIES {
        // TODO cancellation
        // Start download
        let response = reqwest::get(&track.mp3_url).await;
        if let Err(e) = response {
            if e.is_status() {
                eprintln!("http error status {}", e.status().unwrap());
                tries += 1;
                continue;
            } else if e.is_timeout() {
                tries += 1;
                continue;
            } else {
                eprintln!("download error {}", e);
                return Err(Error::Download);
            }
        }
        let mut response = response.unwrap();

        let dir = Path::new(&track.path).parent();
        if let Some(parent_dir) = dir {
            if !parent_dir.exists() {
                sender
                    .try_send(Message::Log(
                        format!("creating dir {}", parent_dir.to_string_lossy()),
                        LogLevel::Info,
                    ))
                    .expect("Failed to send message");

                fs::create_dir_all(parent_dir).await?;
            }
        }

        println!("creating file {}", &track.path);
        let mut destination = fs::File::create(&track.path).await?;
        println!("file created");

        let mut downloaded = 0;
        let total_size = response.content_length().unwrap_or(0);
        while let Some(chunk) = response.chunk().await? {
            destination.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;
            sender
                .try_send(Message::Progress(Progress {
                    path: track.path.to_string(),
                    complete: downloaded,
                    total: total_size,
                }))
                .expect("Failed to send message");

            let percent = (downloaded as f32 / total_size as f32) * 100.0;
            println!(
                "{}",
                format!(
                    "{} downloaded: {} of {} ({:.2}%)",
                    &track.title, downloaded, total_size, percent
                ),
            );
        }

        println!(
            "Downloaded track \"{}\" ",
            Path::new(&track.path)
                .file_name()
                .unwrap()
                .to_string_lossy(),
        );
        sender
            .try_send(Message::Log(
                format!(
                    "Downloaded track \"{}\" ",
                    Path::new(&track.path)
                        .file_name()
                        .unwrap()
                        .to_string_lossy(),
                ),
                LogLevel::Info,
            ))
            .expect("Failed to send message");

        return Ok(());
    }

    Err(Error::Download)
}

/// Apply id3 tag to a track in the supplied Album
fn tag_track(
    album: Arc<Album>,
    track_index: usize,
    mut sender: mpsc::Sender<Message>,
) -> Result<()> {
    let track = album
        .tracks
        .get(track_index)
        .ok_or(Error::Io(String::from("Bad track index")))?;
    if !Path::new(&track.path).exists() {
        return Err(Error::Io(String::from("File does not exist")));
    }

    // Don't overwrite existing tag
    if let Ok(_) = id3::Tag::read_from_path(&track.path) {
        sender
            .try_send(Message::Log(
                format!(r#"Track already tagged, skipping "{}""#, track.title,),
                LogLevel::Info,
            ))
            .expect("Failed to send message");
        return Ok(());
    }

    let mut tag = id3::Tag::new();
    println!(r#"Tagging track "{}" "#, track.title,);
    sender
        .try_send(Message::Log(
            format!(r#"Tagging track "{}" "#, track.title,),
            LogLevel::Info,
        ))
        .expect("Failed to send message");

    tag.set_album(&album.title);
    tag.set_artist(&album.artist);
    tag.set_title(&track.title);
    tag.set_track(track.number);
    tag.set_total_tracks(album.tracks.len() as u32);
    if let Some(lyrics) = &track.lyrics {
        tag.add_lyrics(id3::frame::Lyrics {
            lang: String::default(),
            description: String::default(),
            text: lyrics.to_string(),
        })
    }

    let year = album.release_date.year();
    let month = u8::try_from(album.release_date.month()).ok();
    let day = u8::try_from(album.release_date.day()).ok();
    tag.set_date_released(id3::Timestamp {
        year,
        month,
        day,
        hour: None,
        minute: None,
        second: None,
    });
    tag.set_year(year);

    tag.add_comment(id3::frame::Comment {
        lang: "eng".to_string(),
        description: "".to_string(),
        text: "Support the artists you enjoy.".to_string(),
    });

    tag.write_to_path(&track.path, id3::Version::Id3v24)
        .map_err(|e| Error::Io(e.description.to_string()))
}

/// Downloads an album, delivering status updates to a channel via the `sender`
async fn download_album(
    album: Album,
    sender: mpsc::Sender<Message>,
    settings: Arc<RwLock<UserSettings>>,
) {
    let UserSettings {
        allowed_file_size_difference,
        save_cover_art_in_folder,
        save_cover_art_in_tags,
        ..
    } = *settings.read().await;

    // TODO cancellation

    // Create directory to place track files
    if let Err(e) = fs::create_dir_all(&album.path).await {
        eprintln!("{}", e);
        println!("An error occured when creating the album folder. Make sure you have the rights to write files in the folder you chose");
        return;
    }

    // TODO Download artwork

    // Download tracks
    let mut download_tasks = Vec::with_capacity(album.tracks.len());
    for track in &album.tracks {
        download_tasks.push(tokio::spawn(download_track_stream(
            track.clone(),
            allowed_file_size_difference,
            sender.clone(),
        )));
    }
    let tracks_downloaded = join_all(download_tasks).await;

    // Tag tracks if they do not already have a tag
    let mut tag_tasks = Vec::with_capacity(album.tracks.len());
    let album = Arc::new(album);
    for i in 0..album.tracks.len() {
        let album = Arc::clone(&album);
        let sender = sender.clone();
        tag_tasks.push(tokio::spawn(async move { tag_track(album, i, sender) }));
    }
    join_all(tag_tasks).await;

    // TODO Create playlist file
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};

    #[tokio::test]
    async fn get_html_text() {
        let msg = "Gets the content at url in utf8 text form";
        let expected = "abcdefghijklmnopqrstuvwxyz";
        let actual = get_url_text("http://httpbin.org/range/26").await.unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[tokio::test]
    async fn client_get_html_text() {
        let client = reqwest::Client::new();
        let msg = "Gets the content at url in utf8 text form";
        let expected = "abcdefghijklmnopqrstuvwxyz";
        let actual = client_get_url_text(&client, "http://httpbin.org/range/26")
            .await
            .unwrap();

        assert_eq!(actual, expected, "{}", msg);
    }

    #[tokio::test]
    async fn gets_discography() {
        let urls: HashSet<_> = vec![
            "https://moter.bandcamp.com/album/wave-transmission",
            "https://theracers.bandcamp.com/",
        ]
        .into_iter()
        .collect();

        let mut expected: Vec<_> = vec![
            "http://moter.bandcamp.com/album/moter-ep",
            "http://moter.bandcamp.com/album/last-train-to-synthville",
            "http://theracers.bandcamp.com/track/final-lap",
            "http://moter.bandcamp.com/album/wave-transmission",
            "http://moter.bandcamp.com/album/omegadriver",
        ];
        let mut actual = get_artist_discography(&urls).await;
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected,);
    }
}
