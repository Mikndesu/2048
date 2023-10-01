#pragma once

#include <iostream>
#include <array>
#include <numeric>
#include <random>
#include <memory>
#include <ncurses.h>

#include "direction.hpp"
#include "array_helper.hpp"

class GameBoardState {
   public:
    GameBoardState();
    ~GameBoardState();
    std::array<std::array<int, 4>, 4>& getState();
    void updateCertainBox(const int y, const int x);
    void move(const Utility2048::Direction direction);

   private:
    std::array<std::array<int, 4>, 4> game_state;
    std::unique_ptr<ArrayHelper> array_helper = nullptr;
    std::mt19937 random;
    // Randomiser used for the appearance of tile 4
    std::uniform_int_distribution<int> r1{1, 10};
    // Randomiser used for choosing certain box, or lines or rows at random.
    std::uniform_int_distribution<int> r2{0, 3};
    void updateCertainBoxInternal(const int y, const int x, const int value);
    void initialiseCertainBox(const int y, const int x);
    int newTileValue();
    void moveInternal(std::array<int, 4>& arr, bool& isMoveSuccessful);
    bool isInitialised(const int y, const int x);
    bool isInitialised(const std::array<int,4>& array, const int index);
};