use gpui::prelude::*;
use gpui::{Render, Styled, div};
use gpui_component::label::Label;
use gpui_component::{ActiveTheme, StyledExt};

use crate::ui::components::sidebar::Sidebar;

pub struct MainView;

impl Render for MainView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .h_flex()
            .h_full()
            .bg(cx.theme().background)
            .child(Sidebar::new())
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .p_4()
                    .justify_start()
                    .child(Label::new(
                        "Welcome to the Main Viewdfsd fasdfsdfsadf sdfasdfasdf",
                    )),
            )
    }
}
