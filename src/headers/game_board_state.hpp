#pragma once

#include <ncurses.h>

#include <array>
#include <iostream>
#include <memory>
#include <numeric>
#include <random>

#include "array_helper.hpp"
#include "direction.hpp"

class GameBoardState {
   public:
    GameBoardState();
    std::array<std::array<int, 4>, 4>& getState();
    void moveTile(const Utility2048::Direction direction);

   private:
    std::array<std::array<int, 4>, 4> game_state;
    std::unique_ptr<ArrayHelper> array_helper = nullptr;
    std::mt19937 random;
    // Randomiser used for the appearance of tile 4
    std::uniform_int_distribution<int> r1{1, 10};
    // Randomiser used for choosing certain box, or lines or rows at random.
    std::uniform_int_distribution<int> r2{0, 3};
    void updateCertainTileInternal(const int y, const int x, const int value);
    void initialiseCertainTile(const int y, const int x);
    void moveTilesInternal(std::array<int, 4>& arr, bool& isMoveSuccessful);
    int newTileValue();
    bool isTileInitialised(const int y, const int x);
    bool isTileInitialised(const std::array<int, 4>& array, const int index);
};