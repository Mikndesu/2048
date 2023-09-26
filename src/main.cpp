#include <iostream>
#include <memory>
#include <cmath>
#include <ncurses.h>
#include "headers/game.hpp"

int main() {
    std::unique_ptr<Game> game = std::make_unique<Game>();
    game->start();
    return 0;
}