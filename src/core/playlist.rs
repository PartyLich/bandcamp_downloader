//! Audio playlist functions
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{model::Album, settings::PlaylistFormat, Result};

/// Write an album playlist to disk in m3u format
fn write_m3u<P>(album: &Album, file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::create(file_path)?;
    let mut writer = m3u::Writer::new(&mut file);

    for track in &album.tracks {
        let entry = m3u::path_entry(&track.path);
        writer.write_entry(&entry)?;
    }

    Ok(())
}

/// Write an album playlist to disk in the specified format
pub fn write_playlist(format: PlaylistFormat, album: &Album, mut file_path: PathBuf) -> Result<()> {
    file_path.set_extension(format.value());
    match format {
        PlaylistFormat::M3U => write_m3u(album, file_path),
    }
}
