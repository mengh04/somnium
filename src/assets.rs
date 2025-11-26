use anyhow::anyhow;
use gpui::*;
use gpui_component::IconNamed;
use rust_embed::RustEmbed;
use std::borrow::Cow;

/// An asset source that loads assets from the `./assets` folder.
#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

pub enum IconName {
    Play,
    Pause,
    SkipForward,
    SkipBack,
}

impl IconNamed for IconName {
    fn path(self) -> SharedString {
        match self {
            IconName::Play => "icons/play.svg",
            IconName::Pause => "icons/pause.svg",
            IconName::SkipForward => "icons/skip_forward.svg",
            IconName::SkipBack => "icons/skip_back.svg",
        }
        .into()
    }
}
