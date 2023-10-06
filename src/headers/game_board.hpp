#pragma once
#include <memory>

#include "direction.hpp"
#include "game_board_state.hpp"
#include "game_data.hpp"

class GameBoard {
   public:
    GameBoard(const int y = 0, const int x = 0, const int width = 4, const int height = 1);
    ~GameBoard();
    void clear();
    void render();
    void moveTile(const Utility2048::Direction direction);
    void saveGameProgress();
    void restoreGameProgress();

   private:
    int x = 0;
    int y = 0;
    const int width;
    const int height;
    const int TO_BE_RENDERED = 4;
    const int VERTICAL_SIDE_LENGTH;
    const int HORIZONTAL_SIDE_LENGTH;
    int getMatchingColour(int i);
    std::unique_ptr<GameBoardState> game_board_state = nullptr;
    std::unique_ptr<GameData> game_data = nullptr;
    void RenderBackGroundGrid();
    void ReflectGameBoardState();
    void putColouredStr(int i);
    void initialiseColourPairs();
};