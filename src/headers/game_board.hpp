#pragma once

class GameBoard {
   public:
    GameBoard(const int y = 0, const int x = 0, const int length = 4);
    ~GameBoard();
    void render();

   private:
    int x = 0;
    int y = 0;
    int length = 4;
    const int TO_BE_RENDERED = 4;
    const int VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX;
    const int VERTICAL_SIDE_LENGTH;
    const int HORIZONTAL_SIDE_LENGTH;
    void RenderBackGroundGrid();
    int VerticalSideLengthOfSmallerBox(int i);
};