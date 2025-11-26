use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};
use std::path::PathBuf;

pub fn init(cx: &mut App) {
    let theme_name = SharedString::from("Catppuccin Macchiato");
    // Load and watch themes from ./themes directory
    let theme_path = PathBuf::from("./themes");
    if let Err(err) = ThemeRegistry::watch_dir(theme_path, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }
}
