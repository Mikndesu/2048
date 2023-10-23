extern crate pancurses;

mod colour;
mod direction;
mod game;
mod game_board;
mod tiles;
mod tiles_state;

fn main() {
    let mut game = game::Game::new();
    game.start()
}
