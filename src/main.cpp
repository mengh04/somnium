#include "audio_player.h"
#include "src/file_browser.h"
#include "ftxui/component/screen_interactive.hpp"
#include "ftxui/component/component.hpp"
#include "ftxui/dom/elements.hpp"
#include <ranges>
#include <iostream>


int main() {
    freopen("error.log", "w", stderr);
    using namespace ftxui;
    auto file_browser = FileBrowser(R"(D:\Music)");
    auto music_infos = file_browser.get_current_music();

    auto audio_player = AudioPlayer();

    auto screen = ScreenInteractive::Fullscreen();

    const std::vector<std::string> music_titles = music_infos | std::views::transform(&MusicInfo::title) | std::ranges::to<std::vector>();

    int menu_selected = 0;
    int old_menu_selected = 0;

    auto radio_box = Radiobox(music_titles, &menu_selected) | frame | CatchEvent([&] (const Event& event) {
        if (old_menu_selected != menu_selected) {
            if (auto result = audio_player.load(music_infos[menu_selected].path); !result) {
                exit(result.error());
            };
            if (auto result = audio_player.play(); !result) {
                exit(result.error());
            };
            old_menu_selected = menu_selected;
            return true;
        }
        return false;
    });


    screen.Loop(radio_box);

    return 0;
}