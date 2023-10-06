#pragma once
#include <array>
#include <filesystem>
#include <fstream>

class GameData {
   public:
    GameData();
    void serialise(std::array<std::array<int, 4>, 4>& array);
    void deserialise(std::array<std::array<int, 4>, 4>& array);

   private:
    std::filesystem::path config_path;
    std::fstream fstream;
};