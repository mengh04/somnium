use gpui::{Entity, prelude::*};
use gpui::{Render, Styled, div};
use gpui_component::{ActiveTheme, StyledExt};

use crate::ui::components::sidebar::Sidebar;
use crate::ui::views::song_list_view::SongListView;

pub struct MainView {
    song_list_view: Entity<SongListView>,
}

impl MainView {
    pub fn new(window: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> Self {
        let song_list_view = cx.new(|cx| SongListView::new(window, cx));
        Self { song_list_view }
    }
}

impl Render for MainView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .h_flex()
            .h_full()
            .bg(cx.theme().background)
            .child(Sidebar::new())
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .p_4()
                    .justify_start()
                    .child(self.song_list_view.clone()),
            )
    }
}
