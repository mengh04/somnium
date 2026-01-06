use std::path::Path;

use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::Accessor,
};

use crate::media::sound_info::SoundInfo;

pub fn get_metadata(path: impl AsRef<Path>) -> anyhow::Result<SoundInfo> {
    let path = path.as_ref();
    let tagged_file = Probe::open(path)?.read()?;
    let properties = tagged_file.properties();

    if let Some(tag) = tagged_file.primary_tag() {
        let title = tag.title().map(|s| s.to_string()).unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Title")
                .to_string()
        });
        let artist = tag.artist().map(|s| s.to_string());
        let album = tag.album().map(|s| s.to_string());

        Ok(SoundInfo {
            title,
            artist,
            path: path.to_path_buf(),
            duration: properties.duration(),
            album,
        })
    } else {
        eprintln!("No metadata!");
        Ok(SoundInfo {
            title: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown Title")
                .to_string(),
            artist: None,
            path: path.to_path_buf(),
            album: None,
            duration: properties.duration(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_metadata() {
        let path = "fixtures/sample.flac";
        let metadata = get_metadata(path).unwrap();

        assert_eq!(metadata.title, "Old Memory");
        assert_eq!(metadata.artist, Some("市川淳".to_string()));
        assert_eq!(
            metadata.album,
            Some("『ヨスガノソラ』オリジナルサウンドトラック-New-".to_string())
        );
    }

    #[test]
    fn test_none_title() {
        let path = "fixtures/no_title.flac";
        let metadata = get_metadata(path).unwrap();

        assert_eq!(metadata.title, "no_title");
    }

    #[test]
    fn test_no_stem() {
        let path = "fixtures/.flac";

        assert!(get_metadata(path).is_err());
    }

    #[test]
    fn test_no_metadata() {
        let path = "fixtures/no_metadata.flac";
        let metadata = get_metadata(path).unwrap();

        assert_eq!(metadata.title, "no_metadata");
        assert_eq!(metadata.artist, None);
    }
}
