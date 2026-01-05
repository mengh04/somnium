use gpui::{Render, Styled, div};
use gpui_component::ActiveTheme;

pub struct MainView;

impl Render for MainView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().size_full().bg(cx.theme().background)
    }
}
