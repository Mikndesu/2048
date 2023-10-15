extern crate pancurses;

mod game;
mod game_board;
mod game_board_state;

fn main() {
    let game = game::Game::new();
    game.start()
}
