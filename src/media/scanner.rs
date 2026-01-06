use crate::media::song::Song;
use tokio::fs;

pub async fn get_songs_from_directory(path: &str) -> anyhow::Result<Vec<Song>> {
    let mut songs = Vec::new();
    let mut entries = fs::read_dir(path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "mp3" || ext == "flac" || ext == "wav" {
                    let file_name = path
                        .file_stem()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unknown")
                        .to_string();

                    let full_path = path.to_string_lossy().to_string();
                    songs.push(Song {
                        id: 0,
                        title: file_name,
                        artist: None,
                        path: full_path,
                    });
                }
            }
        }
    }

    Ok(songs)
}
