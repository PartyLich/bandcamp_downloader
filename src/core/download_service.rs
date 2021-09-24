//! DownloadService public contract
use std::sync::Arc;

use futures::channel::mpsc;
use futures::future::join_all;

use crate::{settings::UserSettings, ui::Message};

/// DownloadService public contract
#[derive(Debug)]
pub struct DownloadService {}

impl DownloadService {
    /// Create a new instance of this struct
    pub fn new() -> Self {
        Self {}
    }

    /// Start downloads

    /// Start downloads
    pub async fn start_downloads(
        self: Arc<Self>,
        urls: String,
        sender: mpsc::Sender<Message>,
        settings: UserSettings,
    ) {
        let albums = crate::fetch_urls(
            &urls,
            settings.download_artist_discography,
            &settings.downloads_path.to_string_lossy(),
            &settings.file_name_format,
        )
        .await;

        // TODO cancellation
        // maybe using a select and a channel to signal?

        let settings = Arc::new(settings);

        if settings.download_one_album_at_a_time {
            // Download one album at a time
            for album in albums {
                crate::download_album(album, sender.clone(), settings.clone()).await;
            }
        } else {
            // Concurrent download
            let download_tasks: Vec<_> = albums
                .into_iter()
                .map(|album| {
                    tokio::spawn(crate::download_album(
                        album,
                        sender.clone(),
                        settings.clone(),
                    ))
                })
                .collect();
            join_all(download_tasks).await;
        }
    }

    // Cancel all downloads
    pub fn cancel_downloads() {
        todo!();
    }
}
