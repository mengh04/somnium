use gpui::{AppContext, Application, WindowBounds, WindowOptions, px, size};

use gpui_component::Root;
use somnium::assets::Assets;
use somnium::services::playback::service::PlayerService;
use somnium::themes;
use somnium::views::main_view::MainWindow;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(|cx| {
        gpui_component::init(cx);
        themes::init(cx);
        PlayerService::init();

        let width_pixels = 1000.;
        let height_pixels = width_pixels / 16. * 9.;
        let window_bounds = WindowBounds::centered(size(px(width_pixels), px(height_pixels)), cx);
        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(window_bounds),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| MainWindow::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .unwrap();
        })
        .detach();
    })
}
