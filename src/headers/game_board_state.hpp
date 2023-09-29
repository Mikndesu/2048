#pragma once

#include <array>
#include <numeric>

class GameBoardState {
   public:
    GameBoardState();
    ~GameBoardState();
    std::array<std::array<int, 4>, 4>& getState();
    void updateCertainBox(const int y, const int x, const int value);

   private:
    std::array<std::array<int, 4>, 4> game_state;
};