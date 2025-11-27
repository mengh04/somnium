use gpui::{ParentElement, Task};
use gpui_component::{
    IndexPath,
    label::Label,
    list::{ListDelegate, ListItem, ListState},
};

use crate::models::song_info::SongInfo;
use crate::services::playback::{player::PlayerCommand, service::PlayerService};

pub struct SongListDelegate {
    song_infos: Vec<SongInfo>,
    filtered_song_infos: Vec<SongInfo>,
    seletected_index: Option<IndexPath>,
}

impl ListDelegate for SongListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &gpui::App) -> usize {
        self.filtered_song_infos.len()
    }

    fn render_item(
        &self,
        ix: IndexPath,
        _window: &mut gpui::Window,
        _cx: &mut gpui::App,
    ) -> Option<Self::Item> {
        self.filtered_song_infos.get(ix.row).map(|song_info| {
            let title = song_info.title.clone().unwrap_or("未知歌曲".to_string());
            ListItem::new(ix).child(Label::new(&title)).on_click({
                let path = song_info.path.clone();
                move |_, _, _| {
                    let player = PlayerService::get();
                    let _ = player
                        .command_sender
                        .send(PlayerCommand::Play(path.clone()));
                }
            })
        })
    }

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<ListState<Self>>,
    ) -> gpui::Task<()> {
        self.filtered_song_infos = self
            .song_infos
            .iter()
            .filter(|song_info| {
                song_info.title.as_ref().map_or(false, |title| {
                    title.to_lowercase().contains(&query.to_lowercase())
                })
            })
            .cloned()
            .collect();
        Task::ready(())
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<ListState<Self>>,
    ) {
        self.seletected_index = ix;
        cx.notify();
    }
}

impl SongListDelegate {
    pub fn new(song_infos: Vec<SongInfo>) -> Self {
        Self {
            song_infos: song_infos.clone(),
            filtered_song_infos: song_infos,
            seletected_index: None,
        }
    }
}
