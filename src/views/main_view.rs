use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, Window, div,
    rgb,
};
use gpui_component::button::Button;
use gpui_component::list::{List, ListState};
use gpui_component::sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem};
use gpui_component::slider::{Slider, SliderState};
use gpui_component::{IconName, Side, StyledExt, v_flex};

use crate::components::song_list::SongListDelegate;
use crate::constants;
use crate::models::song_info::SongInfo;
use crate::utils::find_songs::find_songs;

pub struct MainWindow {
    song_list_state: Entity<ListState<SongListDelegate>>,
    slider_state: Entity<SliderState>,
}

impl Render for MainWindow {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        div().h_flex().size_full().bg(gpui::white()).child(
            div()
                .flex_1()
                .h_full()
                .v_flex()
                .bg(rgb(0x24273a))
                .child(
                    div()
                        .id("songs_container")
                        .flex_1()
                        .w_full()
                        .child(List::new(&self.song_list_state)),
                )
                .child(
                    div()
                        .border_t_1()
                        .border_color(rgb(0x363a4f))
                        .h_20()
                        .w_full()
                        .bg(rgb(0x181926))
                        .v_flex()
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .w_full()
                                .child(Slider::new(&self.slider_state)),
                        )
                        .child(
                            div()
                                .h_flex()
                                .w_full()
                                .flex_1()
                                .border_t_1()
                                .border_color(rgb(0x363a4f))
                                // .bg(gpui::red())
                                .justify_center()
                                .gap_8()
                                .child(Button::new("Previous").label("Previous"))
                                .child(Button::new("Play/Pause").label("Play/Pause"))
                                .child(Button::new("Next").label("Next")),
                        ),
                ),
        )
    }
}

impl MainWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let song_infos = find_songs(constants::MUSIC_DIR)
            .unwrap()
            .into_iter()
            .map(|song_path| SongInfo::new(song_path))
            .collect();

        let song_list_delegate = SongListDelegate::new(song_infos);

        let song_list_state =
            cx.new(|cx| ListState::new(song_list_delegate, window, cx).searchable(true));

        let slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .step(1.)
                .default_value(0.)
        });

        Self {
            song_list_state,
            slider_state,
        }
    }
}
