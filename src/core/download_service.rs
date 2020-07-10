//! DownloadService public contract
use futures::channel::mpsc;
use futures::future::join_all;
use std::sync::Arc;

use crate::{settings::UserSettings, ui::Message};

/// DownloadService public contract
#[derive(Debug)]
pub struct DownloadService {
    /// User configurable application settings (paths, behavior, etc)
    pub settings: UserSettings,
}

impl DownloadService {
    /// Create a new instance of this struct
    pub fn new(settings: UserSettings) -> Self {
        Self { settings }
    }

    /// Start downloads
    pub async fn start_downloads(self: Arc<Self>, urls: String, sender: mpsc::Sender<Message>) {
        let albums = crate::fetch_urls(&urls, self.settings.download_artist_discography).await;
        // TODO cancellation

        if self.settings.download_one_album_at_a_time {
            // Download one album at a time
            for album in albums {
                crate::download_album(album, sender.clone()).await;
            }
        } else {
            // Concurrent download
            let download_tasks: Vec<_> = albums
                .into_iter()
                .map(|album| tokio::spawn(crate::download_album(album, sender.clone())))
                .collect();
            join_all(download_tasks).await;
        }
    }

    // Cancel all downloads
    pub fn cancel_downloads() {
        todo!();
    }
}
