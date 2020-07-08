use std::collections::HashSet;

use futures::future::join_all;
use regex::Regex;

use error::Error;

#[macro_use]
extern crate lazy_static;

mod error;
mod helper;
mod model;

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
