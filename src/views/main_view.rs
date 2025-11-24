use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, div, rgba,
};

use crate::components::song_card::SongCard;
use crate::utils::find_songs::find_songs;

pub struct MainWindow {
    song_cards: Vec<Entity<SongCard>>,
}

impl Render for MainWindow {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .id("main_container")
            .size_full()
            .bg(rgba(0x282c34ff))
            .items_center()
            .overflow_y_scroll()
            .children(self.song_cards.clone())
    }
}

impl MainWindow {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let songs = find_songs(crate::constants::MUSIC_DIR).unwrap();
        let song_cards = songs
            .iter()
            .map(|song_path| cx.new(|cx| SongCard::new(song_path, cx)))
            .collect();

        Self { song_cards }
    }
}
