#include "headers/game_data.hpp"

#include <cstdlib>
#include <string>

GameData::GameData() {
    namespace fs = std::filesystem;
#ifdef _WIN64
    std::string home_dir = Utils::get_enviroment("USERPROFILE");
#else
    std::string home_dir = std::getenv("HOME");
#endif
    std::string config_dir = home_dir + "/.config/2048";
    this->config_path = fs::path(config_dir).append("game.progress");
    try {
        fs::create_directories(config_dir);
    } catch(fs::filesystem_error e) {
    }
    ifs.open(this->config_path, std::ios::in | std::ios::binary);
    ofs.open(this->config_path, std::ios::out | std::ios::binary);
}

void GameData::serialise(std::array<std::array<int, 4>, 4>& array) {
    for(const auto& arr : array) {
        for(const auto& i : arr) {
            ofs.write((const char*)&i, sizeof(i));
        }
    }
}

void GameData::deserialise(std::array<std::array<int, 4>, 4>& array) {
    for(const auto& arr : array) {
        for(const auto& i : arr) {
            ifs.read((char*)&i, sizeof(i));
        }
    }
}