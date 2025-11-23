use lofty::prelude::*;
use lofty::probe::Probe;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SongInfo {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<u32>, // duration in seconds
}

impl SongInfo {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();

        let tagged_file = Probe::open(&path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            // If the "primary" tag doesn't exist, we just grab the
            // first tag we can find. Realistically, a tag reader would likely
            // iterate through the tags to find a suitable one.
            None => tagged_file.first_tag().expect("ERROR: No tags found!"),
        };

        let title = tag.title().map(|s| s.to_string());
        let artist = tag.artist().map(|s| s.to_string());
        let album = tag.album().map(|s| s.to_string());
        let duration = tagged_file.properties().duration().as_secs() as u32;
        let duration = Some(duration);

        Self {
            path,
            title,
            artist,
            album,
            duration,
        }
    }
}
