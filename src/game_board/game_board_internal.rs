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
                self.window.attron(COLOR_PAIR(colour));
                self.window.mvaddstr(
                    self.y + margin_y(self.height) + j * (self.height + 1),
                    self.x + margin_x(self.width, n) + i * (self.width + 1),
                    if n != 0 {
                        n.to_string()
                    } else {
                        "".to_string()
                    },
                );
                self.window.attroff(COLOR_PAIR(colour));
            }
        }
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
