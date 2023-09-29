#include "headers/game.hpp"

#include <ncurses.h>

Game::Game() { this->gameBoard = std::make_unique<GameBoard>(0, 0, 3, 7); }

Game::~Game() {}

void Game::start() {
    while (true) {
        this->gameBoard->render();
        int ch = getch();
        if (ch == 'q') break;
    }
}
