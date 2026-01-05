use gpui::{AppContext, Application, WindowOptions};
use gpui_component::Root;
use gpui_component_assets::Assets;

use crate::views::main_view::MainView;

mod theme;
mod views;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::init(cx);

        cx.spawn(async move |cx| -> anyhow::Result<()> {
            let window_options = WindowOptions::default();
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| MainView);
                cx.new(|cx| Root::new(view, window, cx))
            })?;
            Ok(())
        })
        .detach();
    });
}
