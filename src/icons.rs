use gpui_component::IconNamed;

pub enum IconName {
    Album,
    Setting,
    Artist,
    Folder,
    Music,
}

impl IconNamed for IconName {
    fn path(self) -> gpui::SharedString {
        match self {
            IconName::Album => "icons/album.svg",
            IconName::Setting => "icons/settings.svg",
            IconName::Artist => "icons/artist.svg",
            IconName::Folder => "icons/folder.svg",
            IconName::Music => "icons/music.svg",
        }
        .into()
    }
}
