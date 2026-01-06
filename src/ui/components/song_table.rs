use gpui_component::table::{Column, TableDelegate, TableState};

use gpui::{App, Context, IntoElement, Window};

use crate::media::sound_info::SoundInfo;

struct MyData {
    id: usize,
    name: String,
    age: u32,
    email: String,
}

pub struct SongTableDelegate {
    data: Vec<SoundInfo>,
    columns: Vec<Column>,
}

impl SongTableDelegate {
    pub fn new(songs: Vec<SoundInfo>) -> Self {
        Self {
            data: songs,
            columns: vec![
                Column::new("title", "标题").width(200.).sortable(),
                Column::new("artist", "艺术家").width(150.).sortable(),
                Column::new("album", "专辑").width(159.).sortable(),
                Column::new("duration", "时长").width(80.),
            ],
        }
    }
}

impl TableDelegate for SongTableDelegate {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.data.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let song = &self.data[row_ix];
        let col = &self.columns[col_ix];

        match col.key.as_ref() {
            "title" => song.title.clone(),
            "artist" => song.artist.as_ref().unwrap_or(&"-".to_string()).to_string(),
            "album" => song.album.as_ref().unwrap_or(&"-".to_string()).to_string(),
            "duration" => {
                song.duration.as_secs();
                format!(
                    "{}:{:02}",
                    song.duration.as_secs() / 60,
                    song.duration.as_secs() % 60
                )
            }
            _ => "".to_string(),
        }
    }
}
