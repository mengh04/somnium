use gpui::{
    AppContext, Application, InteractiveElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, WindowOptions, div, rgba,
};
use gpui_component::{StyledExt, label::Label};
use somnium::components::song_card::SongCard;
use somnium::utils::find_songs::find_songs;

struct Somnium;

impl Render for Somnium {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let songs = find_songs("D:/Music").unwrap();

        div()
            .id("main_container")
            .size_full()
            .bg(rgba(0x282c34ff))
            .items_center()
            .overflow_y_scroll()
            .children(
                songs
                    .iter()
                    .map(|song_path| cx.new(|cx| SongCard::new(song_path, cx))),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(|cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| cx.new(|_| Somnium))
                .unwrap();
        })
        .detach();
    })
}
