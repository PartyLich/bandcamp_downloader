//! Audio playlist functions
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    model::{Album, Track},
    settings::PlaylistFormat,
    Result,
};

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

// Transform a Track ref to a pls::PlaylistElement
impl From<&Track> for pls::PlaylistElement {
    fn from(track: &Track) -> Self {
        pls::PlaylistElement {
            path: track.path.clone(),
            title: Some(track.title.clone()),
            len: pls::ElementLength::Seconds(track.duration.floor() as u64),
        }
    }
}

/// Write an album playlist to disk in pls format
fn write_pls<P>(album: &Album, file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::create(file_path)?;
    let playlist: Vec<_> = album.tracks.iter().map(From::from).collect();

    pls::write(&playlist, &mut file)?;

    Ok(())
}

/// Write an album playlist to disk in the specified format
pub fn write_playlist(format: PlaylistFormat, album: &Album, mut file_path: PathBuf) -> Result<()> {
    file_path.set_extension(format.value());
    match format {
        PlaylistFormat::M3U => write_m3u(album, file_path),
        PlaylistFormat::PLS => write_pls(album, file_path),
    }
}
