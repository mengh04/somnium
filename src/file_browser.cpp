//
// Created by mengh04 on 25-7-25.
//

#include "file_browser.h"

#include <format>
#include <utility>

FileBrowser::FileBrowser(std::filesystem::path  path) : m_current_path(std::move(path)) {}

std::vector<MusicInfo> FileBrowser::get_current_music() const {
    std::vector<MusicInfo> entries;

    for (const auto& entry : std::filesystem::directory_iterator(m_current_path)) {
        if (!entry.is_directory() && is_music_file(entry)) {
            entries.emplace_back(get_music_info(entry.path().string()));
        }
    }

    return entries;
}

std::string FileBrowser::get_current_path_str() const {
    return m_current_path.string();
}
MusicInfo FileBrowser::get_music_info(const std::filesystem::path& file_path) {
    TagLib::FileRef f(file_path.string().c_str());
    std::string title = f.tag()->title().to8Bit(true);
    std::string artist = f.tag()->artist().to8Bit(true);
    std::string album = f.tag()->album().to8Bit(true);

    const int seconds = f.audioProperties()->lengthInSeconds();
    int minutes = seconds / 60;
    int secs = seconds % 60;
    std::string duration = std::format("{:02}:{:02}", minutes, secs);

    f.save();
    return MusicInfo{
        .title = std::move(title),
        .artist = std::move(artist),
        .album = std::move(album),
        .duration = std::move(duration),
        .path = file_path.string()
    };
}

bool FileBrowser::is_music_file(const std::filesystem::path& file_path) {
    return file_path.extension() == ".mp3" || file_path.extension() == ".flac" ||
           file_path.extension() == ".ogg" || file_path.extension() == ".wav";
}