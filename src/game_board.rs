/*
Copyright (c) 2023 Mikndesu

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use crate::{data::ProgressData, direction::Direction, tiles_state::TilesState};
use pancurses::{
    curs_set, endwin, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK,
    COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
mod game_board_internal;
pub(crate) mod window_ext;

pub struct GameBoard {
    window: Window,
    game_board_state: TilesState,
    score: i32,
    is_game_over: bool,
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
        start_color();
        curs_set(0);
        initialise_colour_pairs();
        Self {
            window,
            game_board_state,
            score: 0,
            is_game_over: false,
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

    pub fn check_if_game_is_over(&mut self) {
        self.is_game_over =
            !self.game_board_state.has_empty_tile() && !self.game_board_state.is_movable();
    }

    pub fn render(&self) {
        if self.is_game_over {
            self.reflect_game_board_state();
            self.game_over();
            self.window.refresh();
            return;
        }
        self.window.erase();
        self.render_background_grid();
        self.reflect_game_board_state();
        self.render_description();
        self.render_score();
        self.window.refresh();
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        let (is_move_successful, score_increase) = self.game_board_state.move_tiles(direction);
        if is_move_successful {
            self.game_board_state.initialise_tile();
        }
        self.score += score_increase;
    }

    pub fn clear(&mut self) {
        self.game_board_state.clear();
        self.score = 0;
        self.is_game_over = false;
    }

    pub fn restore_progress(&mut self) {
        // if saved progress is not valid, do nothing.
        let game_data = ProgressData::new();
        if let Some((tiles, score)) = game_data.desirialise() {
            self.game_board_state.game_state = Box::new(tiles);
            self.score = score;
            self.is_game_over = false;
        }
    }

    pub fn save_progress(&self) {
        if self.is_game_over {
            return;
        }
        let game_data = ProgressData::new();
        game_data.serialise(&self.game_board_state.game_state, self.score);
    }
}

impl Drop for GameBoard {
    fn drop(&mut self) {
        endwin();
    }
}

fn initialise_colour_pairs() {
    use crate::colour::Colour;
    init_pair(Colour::White.into(), COLOR_WHITE, COLOR_BLACK);
    init_pair(Colour::Green.into(), COLOR_GREEN, COLOR_BLACK);
    init_pair(Colour::LightGreen.into(), 10, COLOR_BLACK);
    init_pair(Colour::Magenta.into(), COLOR_MAGENTA, COLOR_BLACK);
    init_pair(Colour::Cyan.into(), COLOR_CYAN, COLOR_BLACK);
    init_pair(Colour::Red.into(), COLOR_RED, COLOR_BLACK);
    init_pair(Colour::Yellow.into(), COLOR_YELLOW, COLOR_BLACK);
}
