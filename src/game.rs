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

use crate::game_board::GameBoard;

pub struct Game {
    game_baord: GameBoard,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_baord: GameBoard::new(0, 0, 3, 7),
        }
    }

    pub fn start(&mut self) {
        use crate::direction::Direction;
        use pancurses::Input;
        loop {
            self.game_baord.check_if_game_is_over();
            self.game_baord.render();
            let ch = self.game_baord.get_input();
            match ch.unwrap() {
                Input::Character('q') => break,
                Input::Character('r') => self.restore_progress(),
                Input::Character('s') => self.save_progress(),
                Input::Character('a') => self.start_new(),
                Input::KeyUp => self.game_baord.move_tiles(Direction::Up),
                Input::KeyDown => self.game_baord.move_tiles(Direction::Down),
                Input::KeyRight => self.game_baord.move_tiles(Direction::Right),
                Input::KeyLeft => self.game_baord.move_tiles(Direction::Left),
                _ => (),
            }
        }
    }

    fn start_new(&mut self) {
        self.game_baord.clear();
    }

    fn save_progress(&self) {
        self.game_baord.save_progress();
    }

    fn restore_progress(&mut self) {
        self.game_baord.restore_progress();
    }
}
