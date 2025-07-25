//
// Created by mengh04 on 25-7-25.
//

#include "audio_player.h"
#include <utility> // For std::move and std::exchange

void AudioPlayer::data_callback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount) {
    // The user data is the AudioPlayer instance itself
    auto* player = static_cast<AudioPlayer*>(pDevice->pUserData);
    if (player == nullptr || !player->m_is_initialized) {
        return;
    }
    ma_decoder_read_pcm_frames(&player->m_decoder, pOutput, frameCount, nullptr);
}


AudioPlayer::AudioPlayer() : m_device{}, m_decoder{}, m_is_initialized(false) {}

AudioPlayer::~AudioPlayer() {
    if (m_is_initialized) {
        ma_device_uninit(&m_device);
        ma_decoder_uninit(&m_decoder);
    }
}

AudioPlayer::AudioPlayer(AudioPlayer&& other) noexcept
    : m_device(other.m_device),
      m_decoder(other.m_decoder),
      m_is_initialized(std::exchange(other.m_is_initialized, false)) {
    // After moving, the new object owns the data. Update the user data pointer.
    if (m_is_initialized) {
        m_device.pUserData = this;
    }
    // Zero out the other object to prevent double-free
    other.m_device = {};
    other.m_decoder = {};
}

AudioPlayer& AudioPlayer::operator=(AudioPlayer&& other) noexcept {
    if (this != &other) {
        // Uninitialize existing resources if any
        if (m_is_initialized) {
            ma_device_uninit(&m_device);
            ma_decoder_uninit(&m_decoder);
        }

        // Move resources from the other object
        m_device = other.m_device;
        m_decoder = other.m_decoder;
        m_is_initialized = std::exchange(other.m_is_initialized, false);

        // Update user data and zero out the other object
        if (m_is_initialized) {
            m_device.pUserData = this;
        }
        other.m_device = {};
        other.m_decoder = {};
    }
    return *this;
}

// --- Public Methods ---

std::expected<void, Error::Audio> AudioPlayer::load(const std::filesystem::path& filePath) {
    if (m_is_initialized) {
        ma_device_uninit(&m_device);
        ma_decoder_uninit(&m_decoder);
        m_is_initialized = false;
    }

    // 1. Initialize decoder
    ma_result result = ma_decoder_init_file(filePath.string().c_str(), nullptr, &m_decoder);
    if (result != MA_SUCCESS) {
        return std::unexpected(Error::Audio::FILE_LOAD_FAILED);
    }

    // 2. Initialize device with info from decoder
    ma_device_config deviceConfig = ma_device_config_init(ma_device_type_playback);
    deviceConfig.playback.format   = m_decoder.outputFormat;
    deviceConfig.playback.channels = m_decoder.outputChannels;
    deviceConfig.sampleRate        = m_decoder.outputSampleRate;
    deviceConfig.dataCallback      = data_callback;
    deviceConfig.pUserData         = this; // Pass a pointer to this instance

    result = ma_device_init(nullptr, &deviceConfig, &m_device);
    if (result != MA_SUCCESS) {
        ma_decoder_uninit(&m_decoder); // Clean up decoder if device fails
        return std::unexpected(Error::Audio::DEVICE_INIT_FAILED);
    }

    m_is_initialized = true;
    return {};
}

std::expected<void, Error::Audio> AudioPlayer::play() {
    if (!m_is_initialized) {
        return std::unexpected(Error::Audio::NO_FILE_LOADED);
    }

    // Seek to beginning before playing
    if (ma_decoder_seek_to_pcm_frame(&m_decoder, 0) != MA_SUCCESS) {
        return std::unexpected(Error::Audio::SEEK_FAILED);
    }

    if (ma_device_start(&m_device) != MA_SUCCESS) {
        return std::unexpected(Error::Audio::DEVICE_START_FAILED);
    }

    return {};
}
