#pragma once

#include <memory>

#include "game_board.hpp"

class Game {
   public:
    Game();
    ~Game();
    void start();

   private:
    std::unique_ptr<GameBoard> gameBoard = nullptr;
};