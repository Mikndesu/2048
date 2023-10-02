#include "headers/game.hpp"

#include <ncurses.h>

#include "headers/direction.hpp"

Game::Game() {
    this->gameBoard = std::make_unique<GameBoard>(0, 0, 3, 7);
}

Game::~Game() {}

void Game::start() {
    using namespace Utility2048;
    while(true) {
        this->gameBoard->render();
        switch(int ch = getch(); ch) {
            case KEY_UP:
                this->gameBoard->move(Direction::UP);
                break;
            case KEY_DOWN:
                this->gameBoard->move(Direction::DOWN);
                break;
            case KEY_RIGHT:
                this->gameBoard->move(Direction::RIGHT);
                break;
            case KEY_LEFT:
                this->gameBoard->move(Direction::LEFT);
                break;
            case 'q':
                return;
        }
    }
}
