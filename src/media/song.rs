use std::path::PathBuf;

pub struct SoundInfo {
    pub title: String,
    pub artist: Option<String>,
    pub path: PathBuf,
}
