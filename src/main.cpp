#include "audio_player.h"
#include <iostream>

int main() {
    AudioPlayer player;

    auto load_result = player.load_and_init("assets/music.flac");
    if (!load_result) {
        std::cerr << "Failed to load and initialize audio file." << std::endl;
        // Here you could check the specific error from the std::unexpected
        return -1;
    }

    std::cout << "File loaded. Press Enter to play..." << std::endl;
    std::cin.get();

    auto play_result = player.play();
    if (!play_result) {
        std::cerr << "Failed to play audio." << std::endl;
        return -1;
    }

    std::cout << "Playing... Press Enter to quit." << std::endl;
    std::cin.get();


    return 0;
}