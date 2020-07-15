//! DownloadService public contract
use std::sync::Arc;

use futures::channel::mpsc;
use futures::future::join_all;
use tokio::sync::RwLock;

use crate::{settings::UserSettings, ui::Message};

type Settings = Arc<RwLock<UserSettings>>;

/// DownloadService public contract
#[derive(Debug)]
pub struct DownloadService {
    /// User configurable application settings (paths, behavior, etc)
    pub settings: Settings,
}

impl DownloadService {
    /// Create a new instance of this struct
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Start downloads
    pub async fn start_downloads(self: Arc<Self>, urls: String, sender: mpsc::Sender<Message>) {
        let settings = self.settings.read().await;

        let albums = crate::fetch_urls(
            &urls,
            settings.download_artist_discography,
            &settings.downloads_path.to_string_lossy(),
        )
        .await;

        // TODO cancellation

        if settings.download_one_album_at_a_time {
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
