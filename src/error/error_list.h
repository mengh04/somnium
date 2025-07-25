#pragma once

namespace Error {
    enum Audio {
        OK,
        ERROR,
        FILE_LOAD_FAILED,
        DEVICE_START_FAILED,
        NO_FILE_LOADED,
        DEVICE_INIT_FAILED,
        SEEK_FAILED
    };
}