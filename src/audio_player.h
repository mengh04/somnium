//
// Created by mengh04 on 25-7-25.
//
#pragma once

#include <expected>
#include <filesystem>
#include "error/error_list.h"
#include "thirdparty/miniaudio/miniaudio.h"

class AudioPlayer {
public:
    AudioPlayer();
    ~AudioPlayer();

    AudioPlayer(const AudioPlayer&) = delete;
    AudioPlayer& operator=(const AudioPlayer&) = delete;
    AudioPlayer(AudioPlayer&&) noexcept;
    AudioPlayer& operator=(AudioPlayer&&) noexcept;

    std::expected<void, Error::Audio> load(const std::filesystem::path& filePath);
    std::expected<void, Error::Audio> play();

private:
    static void data_callback(ma_device* p_device, void* p_output, const void* p_input, ma_uint32 frame_count);

    ma_device m_device;
    ma_decoder m_decoder;
    bool m_is_initialized = false;
};
