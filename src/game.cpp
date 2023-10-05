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
                this->gameBoard->moveTile(Direction::UP);
                break;
            case KEY_DOWN:
                this->gameBoard->moveTile(Direction::DOWN);
                break;
            case KEY_RIGHT:
                this->gameBoard->moveTile(Direction::RIGHT);
                break;
            case KEY_LEFT:
                this->gameBoard->moveTile(Direction::LEFT);
                break;
            case 'a':
                this->startNewGame();
                return;
            case 'p':
                this->saveGameProgress();
                return;
            case 'r':
                this->restoreGameProgress();
                return;
            case 'q':
                return;
        }
    }
}

void Game::startNewGame() {
    this->gameBoard->clear();
    this->start();
}

void Game::saveGameProgress() {}

void Game::restoreGameProgress() {}
