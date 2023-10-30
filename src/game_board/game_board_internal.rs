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

use super::window_ext::WindowExt;
use super::GameBoard;
use pancurses::{
    ACS_BTEE, ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER, ACS_LTEE, ACS_PLUS, ACS_RTEE, ACS_TTEE,
    ACS_ULCORNER, ACS_URCORNER, ACS_VLINE, COLOR_PAIR,
};

impl GameBoard {
    pub(crate) fn reflect_game_board_state(&self) {
        let state = &self.game_board_state;
        let margin_y = |i: i32| -> i32 {
            return if i != 1 { (i / 2) + 1 } else { 1 };
        };
        let margin_x = |i: i32, n: i32| -> i32 {
            let addtional = if n >= 128 { -1 } else { 0 };
            return addtional + if i != 1 { (i / 2) + 1 } else { 1 };
        };
        for i in 0..self.to_be_rendered {
            for j in 0..self.to_be_rendered {
                let n = state.get_state()[i as usize][j as usize];
                let colour = matching_colour(n);
                self.window.attron(COLOR_PAIR(colour.into()));
                self.window.mvaddstr(
                    self.y + margin_y(self.height) + j * (self.height + 1),
                    self.x + margin_x(self.width, n) + i * (self.width + 1),
                    if n != 0 {
                        n.to_string()
                    } else {
                        "".to_string()
                    },
                );
                self.window.attroff(COLOR_PAIR(colour.into()));
            }
        }
    }

    pub(crate) fn game_over(&self) {
        use crate::colour::Colour;
        const A: &str = r#"8""""8                       8"""88                    "#;
        const B: &str = r#"8    " eeeee eeeeeee eeee    8    8 ee   e eeee eeeee  "#;
        const C: &str = r#"8e     8   8 8  8  8 8       8    8 88   8 8    8   8  "#;
        const D: &str = r#"88  ee 8eee8 8e 8  8 8eee    8    8 88  e8 8eee 8eee8e "#;
        const E: &str = r#"88   8 88  8 88 8  8 88      8    8  8  8  88   88   8 "#;
        const F: &str = r#"88eee8 88  8 88 8  8 88ee    8eeee8  8ee8  88ee 88   8 "#;
        let desc = vec![A, B, C, D, E, F];
        self.window.attron(COLOR_PAIR((Colour::Red as u32).into()));
        desc.iter().enumerate().for_each(|(i, msg)| {
            let str = msg.to_string();
            self.window.mvaddstr(self.y + 5 + i as i32, self.x, str);
        });
        self.window.attroff(COLOR_PAIR((Colour::Red as u32).into()));
    }

    pub(crate) fn render_background_grid(&self) {
        for i in 0..=self.to_be_rendered {
            self.window.mvvline(
                0,
                (self.width + 1) * i,
                ACS_VLINE(),
                self.vertical_side_length,
            );
            self.window.mvhline(
                (self.height + 1) * i,
                0,
                ACS_HLINE(),
                self.horizontal_side_length,
            );
        }
        for i in 1..=3 {
            self.window
                .mvaddch(self.y + (self.height + 1) * i, self.x, ACS_LTEE());
            self.window.mvaddch(
                self.y + (self.height + 1) * i,
                self.x + self.horizontal_side_length,
                ACS_RTEE(),
            );
            self.window
                .mvaddch(self.y, self.x + (self.width + 1) * i, ACS_TTEE());
            self.window.mvaddch(
                self.y + self.vertical_side_length,
                self.x + (self.width + 1) * i,
                ACS_BTEE(),
            );
        }
        for i in 1..=3 {
            for j in 1..=3 {
                self.window.mvaddch(
                    self.y + (self.height + 1) * i,
                    self.x + (self.width + 1) * j,
                    ACS_PLUS(),
                );
            }
        }
        self.window.mvaddch(self.y, self.x, ACS_ULCORNER());
        self.window
            .mvaddch(self.y + self.vertical_side_length, self.x, ACS_LLCORNER());
        self.window
            .mvaddch(self.y, self.x + self.horizontal_side_length, ACS_URCORNER());
        self.window.mvaddch(
            self.y + self.vertical_side_length,
            self.horizontal_side_length,
            ACS_LRCORNER(),
        );
    }

    pub(crate) fn render_description(&self) {
        let origin_y_pos = 3;
        let origin_x_pos = self.horizontal_side_length + 4;
        const ARROW: &str = "Use arrow keys to move tiles.";
        const SAVE: &str = "Press 's' to save the current progress.";
        const RESTORE: &str = "Press 'r' to restore your previous progress.";
        const NEW_GAME: &str = "Press 'a' to start a new game.";
        const QUIT: &str = "Press 'q' to quit.";
        let desc = vec![ARROW, SAVE, RESTORE, NEW_GAME, QUIT];
        desc.iter().enumerate().for_each(|(i, x)| {
            let str = x.to_string();
            self.window
                .mvaddstr(origin_y_pos + i as i32, origin_x_pos, str);
        })
    }

    pub(crate) fn render_score(&self) {
        let origin_y_pos = 1;
        let origin_x_pos = self.horizontal_side_length + 4;
        self.window.mvaddstr(
            origin_y_pos,
            origin_x_pos,
            format!("Current Score: {}", self.score),
        );
    }
}

fn matching_colour(i: i32) -> u32 {
    use crate::colour::Colour;
    return match i {
        2 => Colour::White,
        4 => Colour::Green,
        8 => Colour::LightGreen,
        16 => Colour::Magenta,
        32 => Colour::Cyan,
        64 => Colour::Red,
        _ => Colour::Yellow,
    } as u32;
}
