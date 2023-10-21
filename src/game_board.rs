use crate::{direction::Direction, tiles_state::TilesState};
use pancurses::{curs_set, endwin, initscr, noecho, Input, Window};
mod game_board_internal;
pub(crate) mod window_ext;

pub struct GameBoard {
    window: Window,
    game_board_state: TilesState,
    y: i32,
    x: i32,
    height: i32,
    width: i32,
    to_be_rendered: i32,
    vertical_side_length: i32,
    horizontal_side_length: i32,
}

impl GameBoard {
    pub fn new(y: i32, x: i32, height: i32, width: i32) -> Self {
        let window = initscr();
        window.keypad(true);
        let game_board_state = TilesState::new();
        let to_be_rendered = 4;
        let vertical_side_length = (height + 1) * to_be_rendered;
        let horizontal_side_length = (width + 1) * to_be_rendered;
        noecho();
        curs_set(0);
        Self {
            window,
            game_board_state,
            y,
            x,
            height,
            width,
            to_be_rendered,
            vertical_side_length,
            horizontal_side_length,
        }
    }

    pub fn get_input(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn render(&self) {
        self.window.erase();
        self.render_background_grid();
        self.reflect_game_board_state();
        self.window.refresh();
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        if self.game_board_state.move_tiles(direction) {
            self.game_board_state.initialise_tile();
        }
    }
}

impl Drop for GameBoard {
    fn drop(&mut self) {
        endwin();
    }
}
