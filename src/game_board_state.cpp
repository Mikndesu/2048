#include "headers/game_board_state.hpp"
#include <ctime>

GameBoardState::GameBoardState() {
    random.seed(std::time(NULL));
    // initialise Game Board State
    this->game_state.fill(std::array<int, 4> {0,0,0,0});
    this->initialiseCertainBox(r2(random), r2(random));
    this->initialiseCertainBox(r2(random), r2(random));
}

GameBoardState::~GameBoardState() {

}

void GameBoardState::updateCertainBox(const int y, const int x) {

}

void GameBoardState::move(const GameBoardState::Direction direction ) {

}

void GameBoardState::initialiseCertainBox(const int y, const int x) {
    if(this->isInitialised(y,x)) {
        this->initialiseCertainBox(r2(random), r2(random));
        return;
    }
    updateCertainBoxInternal(y, x, this->newTileValue());
}

void GameBoardState::updateCertainBoxInternal(const int y, const int x, const int value) {
    this->game_state[y][x] = value;
}

int GameBoardState::newTileValue() {
    // New Tile should be 4 10 % probability
    return r1(random) == 1 ? 4 : 2;
}

bool GameBoardState::isInitialised(const int y, const int x) {
    return this->game_state[y][x] != 0;
}

std::array<std::array<int, 4>, 4>& GameBoardState::getState() {
    return this->game_state;
}