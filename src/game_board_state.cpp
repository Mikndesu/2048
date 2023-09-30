#include "headers/game_board_state.hpp"

#include <ncurses.h>

#include <ctime>

GameBoardState::GameBoardState()
    : array_helper(std::make_unique<ArrayHelper>()) {
    random.seed(std::time(NULL));
    // initialise Game Board State
    this->game_state.fill(std::array<int, 4>{0, 0, 0, 0});
    this->initialiseCertainBox(r2(random), r2(random));
    this->initialiseCertainBox(r2(random), r2(random));
}

GameBoardState::~GameBoardState() {}

void GameBoardState::updateCertainBox(const int y, const int x) {}

void GameBoardState::move(const Utility2048::Direction direction) {
    using namespace Utility2048;
    switch (bool isMoveSuccessful = false; direction) {
        case Direction::UP:
            break;
        case Direction::DOWN:
            break;
        case Direction::RIGHT:
            for (int i = 0; i < 4; ++i) {
                auto row = array_helper->getCertainRow(i, this->game_state);
                // def_prog_mode();
                // endwin();
                // std::cout << row[0] << row[1] << row[2] << row[3] <<
                // std::endl; reset_prog_mode(); refresh();
                auto find_matching_pair = [&](int from) {
                    for(int to = from + 1; to < 4; ++to) {
                        if(row[from] == 0) {
                            break;
                        } else if(row[to] != 0) {
                            if(row[to] == row[from]) {
                                return to;
                            }
                            break;
                        }
                    }
                    return -1;
                };
                auto merge_pair = [&](int from, int to) mutable {
                    if(row[to] == row[from]) {
                        row[to] = row[to] * 2;
                        row[from] = 0;
                        isMoveSuccessful = true;
                    }
                };
                for (int from = 0; from < 4; ++from) {
                    if (int to = find_matching_pair(from); to != -1) {
                        merge_pair(from, to);
                        from += 2;
                    }
                }
                for (int j = 0; j < 3; ++j) {
                    if (row[j + 1] == 0) {
                        row[j + 1] = row[j];
                        row[j] = 0;
                        isMoveSuccessful = true;
                    }
                }
                array_helper->setCertainRow(i, row, this->game_state);
            }
            if(isMoveSuccessful) this->initialiseCertainBox(r2(random), r2(random));
            break;
        case Direction::LEFT:
            break;
    }
}

void GameBoardState::initialiseCertainBox(const int y, const int x) {
    if (this->isInitialised(y, x)) {
        this->initialiseCertainBox(r2(random), r2(random));
        return;
    }
    updateCertainBoxInternal(y, x, this->newTileValue());
}

void GameBoardState::updateCertainBoxInternal(const int y, const int x,
                                              const int value) {
    this->game_state[y][x] = value;
}

int GameBoardState::newTileValue() {
    // New Tile should be 4 10 % probability
    return r1(random) == 1 ? 4 : 2;
}

bool GameBoardState::isInitialised(const int y, const int x) {
    return this->game_state[y][x] != 0;
}

bool GameBoardState::isInitialised(const std::array<int, 4>& array,
                                   const int index) {
    return array[index] != 0;
}

std::array<std::array<int, 4>, 4>& GameBoardState::getState() {
    return this->game_state;
}