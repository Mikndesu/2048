#include "headers/game.hpp"
#include "headers/direction.hpp"

#include <ncurses.h>

Game::Game() { this->gameBoard = std::make_unique<GameBoard>(0, 0, 3, 7); }

Game::~Game() {}

void Game::start() {
    using namespace Utility2048;
    while (true) {
        this->gameBoard->render();
        int ch = getch();
        if (ch == 'q') break;
        if(ch == KEY_UP) break;
        if(ch == KEY_DOWN) break;
        if(ch == KEY_RIGHT) this->gameBoard->move(Direction::RIGHT);
        if(ch == KEY_LEFT) break;
    }
}
