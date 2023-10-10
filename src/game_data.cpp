#include "headers/game_data.hpp"

#include <ncurses.h>

#include <cstdlib>
#include <iostream>
#include <string>

GameData::GameData() {
    namespace fs = std::filesystem;
#ifdef _WIN64
    std::string home_dir = Utils::get_enviroment("USERPROFILE");
#else
    std::string home_dir = std::getenv("HOME");
#endif
    std::string config_dir = home_dir + "/.config/2048";
    this->progress_file_path = fs::path(config_dir).append("progress.dat");
    this->score_file_path = fs::path(config_dir).append("score.dat");
    try {
        fs::create_directories(config_dir);
    } catch(fs::filesystem_error& e) {
    }
    progress.open(this->progress_file_path, std::ios::in | std::ios::out | std::ios::binary);
    score.open(this->score_file_path, std::ios::in | std::ios::out | std::ios::binary);
    if(!progress) {
        std::ofstream(this->progress_file_path, std::ios::out | std::ios::binary);
    } else if(!score) {
        std::ofstream(this->score_file_path, std::ios::out | std::ios::binary);
    }
}

// write down game state to the progress file
void GameData::serialise(std::array<std::array<int, 4>, 4>& array, int& score) {
    progress.seekg(0);
    this->score.seekg(0);
    for(auto& arr : array) {
        for(auto& i : arr) {
            progress.write(reinterpret_cast<char*>(&i), sizeof(i));
        }
    }
    progress.write(reinterpret_cast<char*>(&score), sizeof(score));
}

// read the progress file and overwrite game state
void GameData::deserialise(std::array<std::array<int, 4>, 4>& array, int& score) {
    progress.seekg(0);
    this->score.seekg(0);
    for(auto& arr : array) {
        for(auto& i : arr) {
            progress.read(reinterpret_cast<char*>(&i), sizeof(i));
        }
    }
    progress.read(reinterpret_cast<char*>(&score), sizeof(score));
}