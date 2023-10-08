#pragma once
#include <array>
#include <filesystem>
#include <fstream>

class GameData {
   public:
    GameData();
    void serialise(std::array<std::array<int, 4>, 4>& array, int& score);
    void deserialise(std::array<std::array<int, 4>, 4>& array, int& score);

   private:
    std::filesystem::path progress_file_path;
    std::filesystem::path score_file_path;
    std::fstream progress;
    std::fstream score;
};