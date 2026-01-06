use gpui::App;
use gpui::prelude::*;
use gpui_component::Side;
use gpui_component::sidebar::{
    Sidebar as GCSidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};

use crate::ui::IconName;

#[derive(IntoElement)]
pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut gpui::Window, _cx: &mut App) -> impl gpui::IntoElement {
        GCSidebar::new(Side::Left)
            .header(SidebarHeader::new().child("Somnium"))
            .child(
                SidebarGroup::new("Navigation").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("专辑")
                                .icon(IconName::Album)
                                .on_click(|_, _, _| println!("Albums clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("歌曲")
                                .icon(IconName::Music)
                                .on_click(|_, _, _| println!("Songs clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("艺术家")
                                .icon(IconName::Artist)
                                .on_click(|_, _, _| println!("Artists clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("文件夹")
                                .icon(IconName::Folder)
                                .on_click(|_, _, _| println!()),
                        ),
                ),
            )
    }
}
