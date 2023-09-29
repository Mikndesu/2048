#pragma once
#include "game_board_state.hpp"

class GameBoard {
   public:
    GameBoard(const int y = 0, const int x = 0, const int width = 4, const int height = 1);
    ~GameBoard();
    void render();

   private:
    int x = 0;
    int y = 0;
    const int width;
    const int height;
    const int TO_BE_RENDERED = 4;
    const int VERTICAL_SIDE_LENGTH;
    const int HORIZONTAL_SIDE_LENGTH;
    std::unique_ptr<GameBoardState> game_board_state = nullptr;
    void RenderBackGroundGrid();
    void ReflectGameBoardState();
    int VerticalSideLengthOfSmallerBox(int i);
};