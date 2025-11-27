use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, SharedString, Styled,
    Window, div,
};
use gpui_component::StyledExt;
use gpui_component::button::Button;
use gpui_component::list::{List, ListState};
use gpui_component::slider::{Slider, SliderState};

use crate::assets::IconName;
use crate::components::song_list::SongListDelegate;
use crate::constants;
use crate::models::song_info::SongInfo;
use crate::services::playback::{player::PlayerCommand, service::PlayerService};
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
        let create_player_button =
            |id: SharedString, icon_name: IconName, command: PlayerCommand| {
                Button::new(id).icon(icon_name).on_click({
                    let command = command.clone();
                    move |_, _, _cx| {
                        let player = PlayerService::get();
                        let _ = player.command_sender.send(command.clone());
                    }
                })
            };

        div().h_flex().size_full().child(
            div()
                .flex_1()
                .h_full()
                .v_flex()
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
                        .h_24()
                        .w_full()
                        .v_flex()
                        .child(div().px_3().w_full().child(Slider::new(&self.slider_state)))
                        .child(
                            div()
                                .h_flex()
                                .w_full()
                                .flex_1()
                                .border_t_1()
                                .justify_center()
                                .gap_8()
                                .child(create_player_button(
                                    "skip_back".into(),
                                    IconName::SkipBack,
                                    PlayerCommand::SkipBack,
                                ))
                                .child(create_player_button(
                                    "play_pause".into(),
                                    IconName::Play,
                                    PlayerCommand::Resume,
                                ))
                                .child(create_player_button(
                                    "skip_forward".into(),
                                    IconName::SkipForward,
                                    PlayerCommand::SkipForward,
                                )),
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
