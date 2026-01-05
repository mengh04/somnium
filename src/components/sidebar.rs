use gpui::App;
use gpui::prelude::*;
use gpui_component::Side;
use gpui_component::sidebar::{
    Sidebar as GCSidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};

use crate::icons::IconName;

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
            .header(SidebarHeader::new().child("My Application"))
            .child(
                SidebarGroup::new("Navigation").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("Albums")
                                .icon(IconName::Album)
                                .on_click(|_, _, _| println!("Albums clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("Songs")
                                .icon(IconName::Music)
                                .on_click(|_, _, _| println!("Songs clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("Artists")
                                .icon(IconName::Artist)
                                .on_click(|_, _, _| println!("Artists clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("Folders")
                                .icon(IconName::Folder)
                                .on_click(|_, _, _| println!()),
                        ),
                ),
            )
            .footer(SidebarFooter::new().child("User Profile"))
    }
}
