use regex::Regex;

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
}
