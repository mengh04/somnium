use gpui::{AppContext, Context, Entity, Window};
use gpui_component::table::{Table, TableState};

use crate::{
    media::{metadata::get_metadata, scanner::get_song_paths},
    ui::components::SongTableDelegate,
};

pub struct SongListView {
    song_table_state: Entity<TableState<SongTableDelegate>>,
}

impl SongListView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let songs = get_song_paths("D:/Music")
            .unwrap_or(vec![])
            .iter()
            .map(|path| get_metadata(path))
            .filter_map(|result| result.ok())
            .collect();
        let song_table_state =
            cx.new(|cx| TableState::new(SongTableDelegate::new(songs), window, cx));
        Self { song_table_state }
    }
}

impl gpui::Render for SongListView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        Table::new(&self.song_table_state)
    }
}
