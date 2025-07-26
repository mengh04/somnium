cmake_minimum_required(VERSION 3.31)
project(somnium)

set(CMAKE_CXX_STANDARD 23)

add_executable(somnium src/main.cpp
        src/audio_player.cpp
        src/error/error_list.cpp
        thirdparty/miniaudio/miniaudio.cpp
        src/file_browser.cpp
        src/component/custom_menu.h
        src/component/custom_menu.cpp
)

target_include_directories(somnium PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/
)

find_package(ftxui CONFIG REQUIRED)
find_package(taglib CONFIG REQUIRED)
find_package(ZLIB REQUIRED)

target_link_libraries(somnium PRIVATE
        ftxui::dom ftxui::screen ftxui::component
        TagLib::tag TagLib::tag_c TagLib::TagLib
        ZLIB::ZLIB
)