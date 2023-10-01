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

void GameBoardState::move(const Utility2048::Direction direction) {
    using namespace Utility2048;
    bool isMoveSuccessful = false;
    switch (direction) {
            case Direction::UP:
            for (int i = 0; i < 4; ++i) {
                auto column = array_helper->getCertainColumn(i, this->game_state);
                std::reverse(column.begin(), column.end());
                this->moveInternal(column, isMoveSuccessful);
                std::reverse(column.begin(), column.end());
                array_helper->setCertainColumn(i, column, this->game_state);
            }
            break;
        case Direction::DOWN:
            for (int i = 0; i < 4; ++i) {
                auto column = array_helper->getCertainColumn(i, this->game_state);
                this->moveInternal(column, isMoveSuccessful);
                array_helper->setCertainColumn(i, column, this->game_state);
            }
            break;
        case Direction::RIGHT:
            for (int i = 0; i < 4; ++i) {
                auto row = array_helper->getCertainRow(i, this->game_state);
                this->moveInternal(row, isMoveSuccessful);
                array_helper->setCertainRow(i, row, this->game_state);
            }
            break;
        case Direction::LEFT:
            for (int i = 0; i < 4; ++i) {
                auto row = array_helper->getCertainRow(i, this->game_state);
                std::reverse(row.begin(), row.end());
                this->moveInternal(row, isMoveSuccessful);
                std::reverse(row.begin(), row.end());
                array_helper->setCertainRow(i, row, this->game_state);
            }
            break;
    }
    if (isMoveSuccessful)
                this->initialiseCertainBox(r2(random), r2(random));
}

void GameBoardState::moveInternal(std::array<int, 4>& arr, bool& isMoveSuccessful) {
    auto find_matching_pair = [&](int from) {
        for (int to = from + 1; to < 4; ++to) {
            if (arr[from] == 0) {
                break;
            } else if (arr[to] != 0) {
                if (arr[to] == arr[from]) {
                    return to;
                }
                break;
            }
        }
        return -1;
    };
    auto merge_pair = [&](int from, int to) mutable {
        if (arr[to] == arr[from]) {
            arr[to] = arr[to] * 2;
            arr[from] = 0;
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
        if (arr[j + 1] == 0) {
            arr[j + 1] = arr[j];
            arr[j] = 0;
            isMoveSuccessful = true;
        }
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