use std::path::PathBuf;

pub fn get_song_paths(music_dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let song_paths = music_dir
        .read_dir()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| {
                        matches!(
                            ext.to_lowercase().as_str(),
                            "mp3" | "flac" | "wav" | "aac" | "ogg" | "m4a"
                        )
                    })
        })
        .collect();

    Ok(song_paths)
}
