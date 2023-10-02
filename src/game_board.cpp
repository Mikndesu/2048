#include "headers/game_board.hpp"

#include <ncurses.h>

#include <cmath>
#include <memory>
#include <string>

GameBoard::GameBoard(const int y, const int x, const int height, const int width)
    : y(y),
      x(x),
      width(width),
      height(height),
      HORIZONTAL_SIDE_LENGTH((this->width + 1) * TO_BE_RENDERED),
      VERTICAL_SIDE_LENGTH((this->height + 1) * TO_BE_RENDERED),
      game_board_state(std::make_unique<GameBoardState>()) {
    initscr();
    noecho();
    keypad(stdscr, TRUE);
    curs_set(0);
}

GameBoard::~GameBoard() {
    endwin();
}

void GameBoard::render() {
    clear();
    this->RenderBackGroundGrid();
    this->ReflectGameBoardState();
    refresh();
}

void GameBoard::RenderBackGroundGrid() {
    // render vertical lines and horizontal ones.
    for(int i = 0; i <= TO_BE_RENDERED; ++i) {
        mvhline(y + (this->height + 1) * i, x, ACS_HLINE, this->width * 4 + 4);
        mvvline(y, x + (this->width + 1) * i, ACS_VLINE, VERTICAL_SIDE_LENGTH);
    }

    // render special characters
    for(int i = 1; i <= 3; ++i) {
        mvaddch(y + (this->height + 1) * i, x, ACS_LTEE);
        mvaddch(y + (this->height + 1) * i, x + HORIZONTAL_SIDE_LENGTH, ACS_RTEE);
        mvaddch(y, x + (this->width + 1) * i, ACS_TTEE);
        mvaddch(y + VERTICAL_SIDE_LENGTH, x + (this->width + 1) * i, ACS_BTEE);
    }

    for(int i = 1; i <= 3; ++i) {
        for(int j = 1; j <= 3; ++j) {
            mvaddch(y + (this->height + 1) * i, x + (this->width + 1) * j, ACS_PLUS);
        }
    }

    mvaddch(y, x, ACS_ULCORNER);
    mvaddch(y + VERTICAL_SIDE_LENGTH, x, ACS_LLCORNER);
    mvaddch(y, x + HORIZONTAL_SIDE_LENGTH, ACS_URCORNER);
    mvaddch(y + VERTICAL_SIDE_LENGTH, HORIZONTAL_SIDE_LENGTH, ACS_LRCORNER);
}

void GameBoard::move(const Utility2048::Direction direction) {
    this->game_board_state->move(direction);
}

void GameBoard::ReflectGameBoardState() {
    const auto state = this->game_board_state->getState();
    constexpr auto margin = [](int n) {
        return n == 1 ? 1 : static_cast<int>(std::floor(n / 2)) + 1;
    };
    for(int i = 0; i < TO_BE_RENDERED; ++i) {
        for(int j = 0; j < TO_BE_RENDERED; ++j) {
            const auto str = std::to_string(state.at(i).at(j));
            mvaddstr(y + margin(this->height) + j * (this->height + 1), x + margin(this->width) + (this->width + 1) * i, str.c_str());
        }
    }
}