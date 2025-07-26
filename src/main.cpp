#include "audio_player.h"
#include "src/file_browser.h"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/dom/elements.hpp"
#include <ranges>
#include <iostream>


int main() {
    using namespace ftxui;
    auto file_browser = FileBrowser(R"(C:\Music)");
    auto music_infos = file_browser.get_current_music();

    auto audio_player = AudioPlayer();

    auto screen = ScreenInteractive::Fullscreen();

    const std::vector<std::string> music_titles = music_infos | std::views::transform(&MusicInfo::title) | std::ranges::to<std::vector>();

    int menu_selected = 0;
    int old_menu_selected = 0;

    screen.Loop();

    return 0;
}