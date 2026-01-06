use std::{path::PathBuf, time::Duration};

pub struct SoundInfo {
    pub title: String,
    pub artist: Option<String>,
    pub path: PathBuf,
    pub album: Option<String>,
    pub duration: Duration,
}
