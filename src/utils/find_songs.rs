use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn find_songs(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let song_extensions = ["mp3", "flac", "wav", "aac", "ogg", "m4a"];
    let songs = fs::read_dir(dir.as_ref())?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| {
                        song_extensions.iter().any(|e| ext.eq_ignore_ascii_case(e))
                    })
        })
        .collect();
    Ok(songs)
}
