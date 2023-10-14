use crate::game_board::window_ext::WindowExt;
use crate::game_board::GameBoard;
use pancurses::{
    ACS_BTEE, ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER, ACS_LTEE, ACS_PLUS, ACS_RTEE, ACS_TTEE,
    ACS_ULCORNER, ACS_URCORNER, ACS_VLINE,
};

impl GameBoard {
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
}
