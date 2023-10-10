#include "headers/game_board.hpp"

#include <ncurses.h>

#include <cmath>
#include <cstring>
#include <memory>
#include <string>

#include "headers/colour.hpp"

GameBoard::GameBoard(const int y, const int x, const int height, const int width)
    : y(y),
      x(x),
      width(width),
      height(height),
      HORIZONTAL_SIDE_LENGTH((this->width + 1) * TO_BE_RENDERED),
      VERTICAL_SIDE_LENGTH((this->height + 1) * TO_BE_RENDERED),
      game_board_state(std::make_unique<GameBoardState>()),
      game_data(std::make_unique<GameData>()) {
    initscr();
    noecho();
    // to detect special keys like Enter key.
    keypad(stdscr, TRUE);
    // to render coloured character.
    start_color();
    curs_set(0);
    this->initialiseColourPairs();
}

GameBoard::~GameBoard() {
    this->saveGameProgress();
    endwin();
}

void GameBoard::clear() {
    this->game_board_state->clearProgress();
}

void GameBoard::render() {
    erase();
    this->RenderBackGroundGrid();
    this->ReflectGameBoardState();
    this->reflectGameScore();
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

void GameBoard::moveTile(const Utility2048::Direction direction) {
    this->game_board_state->moveTile(direction);
}

void GameBoard::ReflectGameBoardState() {
    const auto state = this->game_board_state->getState();
    constexpr auto margin_y = [](int i) {
        return i == 1 ? 1 : static_cast<int>(std::floor(i / 2)) + 1;
    };
    auto margin_x = [](int i, int n) {
        int addtional_margin = (n >= 128) ? -1 : 0;
        return addtional_margin + (i == 1 ? 1 : static_cast<int>(std::floor(i / 2)) + 1);
    };
    for(int i = 0; i < TO_BE_RENDERED; ++i) {
        for(int j = 0; j < TO_BE_RENDERED; ++j) {
            int n = state.at(i).at(j);
            auto colour = getMatchingColour(n);
            auto char_to_be_rendered = [&]() {
                std::string str = (n != 0) ? std::to_string(n) : "";
                return str;
            };
            attron(COLOR_PAIR(colour));
            mvaddstr(y + margin_y(this->height) + j * (this->height + 1), x + margin_x(this->width, n) + (this->width + 1) * i,
                     char_to_be_rendered().c_str());
            attroff(COLOR_PAIR(colour));
        }
    }
}

void GameBoard::showDescription() {
    const int origin_y_pos = this->HORIZONTAL_SIDE_LENGTH + 5;
    const int origin_x_pos = 4;
}

void GameBoard::reflectGameScore() {
    const int origin_y_pos = 0;
    const int origin_x_pos = this->HORIZONTAL_SIDE_LENGTH + 5;
    constexpr auto message = "Current Score: ";
    mvaddstr(origin_y_pos, origin_x_pos, message);
    mvaddstr(origin_y_pos, origin_x_pos + std::strlen(message), std::to_string(this->game_board_state->getCurrentScore()).c_str());
}

void GameBoard::saveGameProgress() {
    this->game_data->serialise(this->game_board_state->getState(), this->game_board_state->getCurrentScore());
}

void GameBoard::restoreGameProgress() {
    this->game_data->deserialise(this->game_board_state->getState(), this->game_board_state->getCurrentScore());
}

void GameBoard::initialiseColourPairs() {
    using namespace Utility2048;
    using enum Colour;
    // for tile 2 and gameboard
    init_pair(getIntValue(WHITE), COLOR_WHITE, COLOR_BLACK);
    // for tile 4
    init_pair(getIntValue(GREEN), COLOR_GREEN, COLOR_BLACK);
    // for tile 8
    init_pair(getIntValue(LIGHT_GREEN), 10, COLOR_BLACK);
    // for tile 16
    init_pair(getIntValue(MAGENTA), COLOR_MAGENTA, COLOR_BLACK);
    // for tile 32
    init_pair(getIntValue(CYAN), COLOR_CYAN, COLOR_BLACK);
    // for tile 64
    init_pair(getIntValue(RED), COLOR_RED, COLOR_BLACK);
    // for tile 128 and more
    init_pair(getIntValue(YELLOW), COLOR_YELLOW, COLOR_BLACK);
}

int GameBoard::getMatchingColour(int i) {
    using namespace Utility2048;
    using enum Colour;
    switch(i) {
        case 2:
            return getIntValue(WHITE);
        case 4:
            return getIntValue(GREEN);
        case 8:
            return getIntValue(LIGHT_GREEN);
        case 16:
            return getIntValue(MAGENTA);
        case 32:
            return getIntValue(CYAN);
        case 64:
            return getIntValue(RED);
        default:
            return getIntValue(YELLOW);
    }
}