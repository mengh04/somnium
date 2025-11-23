use std::path::Path;

use gpui::{AppContext, Context, Entity, ParentElement, Render, div};
use gpui_component::button::Button;

use crate::models::song_info::SongInfo;

pub struct SongCard {
    song_info: Entity<SongInfo>,
}

impl Render for SongCard {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let title = self
            .song_info
            .read(cx)
            .title
            .as_deref()
            .unwrap_or("未知歌曲");

        div().child(Button::new("song").label(title.to_string()))
    }
}

impl SongCard {
    pub fn new(song_path: impl AsRef<Path>, cx: &mut Context<SongCard>) -> Self {
        Self {
            song_info: cx.new(|_| SongInfo::new(song_path)),
        }
    }
}
