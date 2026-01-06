use crate::{ui, ui::Assets, ui::views::MainView};
use gpui::{AppContext, Application, WindowBounds, WindowOptions, px, size};
use gpui_component::Root;

pub fn run() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        ui::theme::init(cx);

        let window_bounds = WindowBounds::centered(size(px(1200.), px(800.)), cx);
        cx.spawn(async move |cx| -> anyhow::Result<()> {
            let window_options = WindowOptions {
                window_bounds: Some(window_bounds),
                ..Default::default()
            };
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| MainView);
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok(())
        })
        .detach();
    });
}
