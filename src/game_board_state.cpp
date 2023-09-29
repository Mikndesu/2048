#include "headers/game_board_state.hpp"

GameBoardState::GameBoardState() {
    this->game_state.fill(std::array<int, 4> {0,0,0,0});
}

GameBoardState::~GameBoardState() {

}

void GameBoardState::updateCertainBox(const int y, const int x, const int value) {
    
}

std::array<std::array<int, 4>, 4>& GameBoardState::getState() {
    return this->game_state;
}