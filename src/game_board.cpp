#include "headers/game_board.hpp"

#include <ncurses.h>

#include <cmath>

GameBoard::GameBoard(const int y, const int x, const int length)
    : y(y),
      x(x),
      length(length),
      VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX(
          VerticalSideLengthOfSmallerBox(length)),
      HORIZONTAL_SIDE_LENGTH((this->length + 1) * TO_BE_RENDERED),
      VERTICAL_SIDE_LENGTH((VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX + 1) *
                           TO_BE_RENDERED) {
    initscr();
    noecho();
    curs_set(0);
}

GameBoard::~GameBoard() { endwin(); }

void GameBoard::render() { this->RenderBackGroundGrid(); }

void GameBoard::RenderBackGroundGrid() {
    // render vertical lines and horizontal ones.
    for (int i = 0; i <= TO_BE_RENDERED; ++i) {
        mvhline(y + (VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX + 1) * i, x, ACS_HLINE,
                this->length * 4 + 4);
        mvvline(y, x + (this->length + 1) * i, ACS_VLINE, VERTICAL_SIDE_LENGTH);
    }

    // render special characters
    for (int i = 1; i <= 3; ++i) {
        mvaddch(y + (VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX + 1) * i, x, ACS_LTEE);
        mvaddch(y + (VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX + 1) * i,
                x + HORIZONTAL_SIDE_LENGTH, ACS_RTEE);
        mvaddch(y, x + (this->length + 1) * i, ACS_TTEE);
        mvaddch(y + VERTICAL_SIDE_LENGTH, x + (length + 1) * i, ACS_BTEE);
    }

    for (int i = 1; i <= 3; ++i) {
        for (int j = 1; j <= 3; ++j) {
            mvaddch(y + (VERTICAL_SIDE_LENGTH_OF_SMALLER_BOX + 1) * i,
                    x + (this->length + 1) * j, ACS_PLUS);
        }
    }

    mvaddch(y, x, ACS_ULCORNER);
    mvaddch(y + VERTICAL_SIDE_LENGTH, x, ACS_LLCORNER);
    mvaddch(y, x + HORIZONTAL_SIDE_LENGTH, ACS_URCORNER);
    mvaddch(y + VERTICAL_SIDE_LENGTH, HORIZONTAL_SIDE_LENGTH, ACS_LRCORNER);
}

int GameBoard::VerticalSideLengthOfSmallerBox(int length) {
    return static_cast<int>(std::floor(length * 0.5));
}