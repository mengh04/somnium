use gpui::{
    AppContext, AsyncApp, Context, Entity, InteractiveElement, ParentElement, Render, Styled,
    WeakEntity, Window, div,
};
use gpui_component::button::Button;
use gpui_component::list::{List, ListState};
use gpui_component::slider::{Slider, SliderState};
use gpui_component::{Disableable, StyledExt};

use crate::assets::IconName;
use crate::components::song_list::SongListDelegate;
use crate::constants;
use crate::models::song_info::SongInfo;
use crate::playback::commands::PlayerCommand;
use crate::playback::events::PlayerEvent;
use crate::playback::service::PlayerService;
use crate::playback::state::PlayerState;
use crate::utils::find_songs::find_songs;

pub struct MainWindow {
    song_list_state: Entity<ListState<SongListDelegate>>,
    slider_state: Entity<SliderState>,
    current_state: PlayerState,
}

impl Render for MainWindow {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let view_handle = cx.entity();

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
                                .child(
                                    Button::new("resume/pause")
                                        .icon({
                                            if matches!(self.current_state, PlayerState::Playing) {
                                                IconName::Pause
                                            } else {
                                                IconName::Play
                                            }
                                        })
                                        .on_click({
                                            move |_, _, cx| {
                                                view_handle.update(cx, |this, _cx| {
                                                    let service = PlayerService::get();
                                                    if matches!(
                                                        this.current_state,
                                                        PlayerState::Playing
                                                    ) {
                                                        let _ = service
                                                            .command_sender
                                                            .send(PlayerCommand::Pause);
                                                    } else if matches!(
                                                        this.current_state,
                                                        PlayerState::Paused
                                                    ) {
                                                        let _ = service
                                                            .command_sender
                                                            .send(PlayerCommand::Resume);
                                                    }
                                                })
                                            }
                                        })
                                        .disabled({
                                            if matches!(self.current_state, PlayerState::Stopped) {
                                                true
                                            } else {
                                                false
                                            }
                                        }),
                                ),
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

        cx.spawn(|weak_entity: WeakEntity<MainWindow>, cx: &mut AsyncApp| {
            let mut event_receiver = PlayerService::get().event_receiver.resubscribe();
            let cx = cx.clone();
            async move {
                while let Ok(event) = event_receiver.recv().await {
                    let _ = cx.update(|cx| {
                        if let Some(entity) = weak_entity.upgrade() {
                            entity.update(cx, |this, cx| match event {
                                PlayerEvent::StateChanged(state) => {
                                    this.current_state = state;
                                    cx.notify();
                                }
                                _ => {}
                            });
                        }
                    });
                }
            }
        })
        .detach();

        Self {
            song_list_state,
            slider_state,
            current_state: PlayerState::Stopped,
        }
    }
}
