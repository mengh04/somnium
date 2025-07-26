//
// Created by mengh04 on 25-7-25.
//
#pragma once

#include "fileref.h"

#include <string>
#include <filesystem>
#include <vector>
#include <optional>

struct MusicInfo {
    std::string title;
    std::string artist;
    std::string album;
    std::string duration;
    std::string path;
};


class FileBrowser {
public:
    FileBrowser() = default;
    FileBrowser(std::filesystem::path );

    bool navigate_to(const std::filesystem::path& path);
    bool go_up_directory();

    [[nodiscard]] std::string get_current_path_str() const;
    [[nodiscard]] std::vector<MusicInfo> get_current_music() const;

    static MusicInfo get_music_info(const std::filesystem::path& file_path);



private:
    std::filesystem::path m_current_path;

    static bool is_music_file(const std::filesystem::path& file_path);
};


