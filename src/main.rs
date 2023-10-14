extern crate pancurses;

mod game;
mod game_board;

fn main() {
    let game = game::Game::new();
    game.start()
}
