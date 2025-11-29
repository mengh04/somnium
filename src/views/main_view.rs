use std::time::Duration;

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
                                                view_handle.update(cx, |this, cx| {
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

        // ==================== 修复的代码开始 ====================
        cx.spawn(|weak_entity: WeakEntity<MainWindow>, cx: &mut AsyncApp| {
            // 1. 关键步骤：在这里克隆！
            // cx 传进来是引用，我们 clone 它得到一个有所有权的变量。
            let cx = cx.clone();

            async move {
                loop {
                    // 2. 异步等待 50ms (这是你的轮询间隔)
                    // 使用 cx.background_executor() 确保时间调度准确且不阻塞
                    cx.background_executor()
                        .timer(Duration::from_millis(50))
                        .await;

                    // 3. 切回主线程更新 UI
                    // cx.update 会调度闭包到 UI 线程执行
                    let _ = cx.update(|cx| {
                        // 尝试获取 Entity (MainWindow)，如果窗口关了，upgrade 会失败
                        if let Some(entity) = weak_entity.upgrade() {
                            // 调用你写好的 poll_events
                            entity.update(cx, |this, cx| {
                                this.poll_events(cx);
                            });
                        }
                    });

                    // 4. 如果 Entity 已经销毁 (窗口关闭)，跳出循环停止任务
                    if weak_entity.upgrade().is_none() {
                        break;
                    }
                }
            }
        })
        .detach();
        // ==================== 修复的代码结束 ====================

        Self {
            song_list_state,
            slider_state,
            current_state: PlayerState::Stopped,
        }
    }

    fn poll_events(&mut self, cx: &mut Context<Self>) {
        let player_service = PlayerService::get();
        let event_receiver = player_service.event_receiver.lock().unwrap();

        while let Ok(event) = event_receiver.try_recv() {
            match event {
                PlayerEvent::StateChanged(new_state) => {
                    self.current_state = new_state;
                    cx.notify();
                }
                _ => (),
            }
        }
    }
}
