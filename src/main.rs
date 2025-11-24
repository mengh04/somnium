use gpui::{AppContext, Application, WindowOptions};

use gpui_component::Root;
use somnium::views::main_view::MainWindow;

fn main() {
    let app = Application::new();

    app.run(|cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| MainWindow::new(cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();
        })
        .detach();
    })
}
