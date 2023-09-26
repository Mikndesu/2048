#include <ncurses.h>
#include "headers/game.hpp"


Game::Game() {
    this->gameBoard = std::make_unique<GameBoard>();
}

Game::~Game() {
}

void Game::start() {
    while(true) {
        this->gameBoard->render(0,0,4);
        int ch = getch();
        if (ch == 'q') break;
    }
}

