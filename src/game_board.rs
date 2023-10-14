use pancurses::{
    curs_set, endwin, initscr, noecho, Window, ACS_BTEE, ACS_HLINE, ACS_LLCORNER, ACS_LRCORNER,
    ACS_LTEE, ACS_PLUS, ACS_RTEE, ACS_TTEE, ACS_ULCORNER, ACS_URCORNER, ACS_VLINE,
};

pub struct GameBoard {
    y: i32,
    x: i32,
    height: i32,
    width: i32,
    window: Window,
}

impl GameBoard {
    pub fn new(y: i32, x: i32, height: i32, width: i32) -> Self {
        let window = initscr();
        noecho();
        curs_set(0);
        Self {
            y,
            x,
            height,
            width,
            window,
        }
    }

    pub fn render(&self) {
        loop {
            let to_be_rendered = 4;
            let vertical_side_length = (self.height + 1) * to_be_rendered;
            let horizontal_side_length = (self.width + 1) * to_be_rendered;
            for i in 0..=to_be_rendered {
                self.window.mv(0, (self.width + 1) * i);
                self.window.vline(ACS_VLINE(), vertical_side_length);
                self.window.mv((self.height + 1) * i, 0);
                self.window.hline(ACS_HLINE(), horizontal_side_length);
            }
            for i in 1..=3 {
                self.window
                    .mvaddch(self.y + (self.height + 1) * i, self.x, ACS_LTEE());
                self.window.mvaddch(
                    self.y + (self.height + 1) * i,
                    self.x + horizontal_side_length,
                    ACS_RTEE(),
                );
                self.window
                    .mvaddch(self.y, self.x + (self.width + 1) * i, ACS_TTEE());
                self.window.mvaddch(
                    self.y + vertical_side_length,
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
                .mvaddch(self.y + vertical_side_length, self.x, ACS_LLCORNER());
            self.window
                .mvaddch(self.y, self.x + horizontal_side_length, ACS_URCORNER());
            self.window.mvaddch(
                self.y + vertical_side_length,
                horizontal_side_length,
                ACS_LRCORNER(),
            );
            let ch = self.window.getch();
            if ch.unwrap() == pancurses::Input::KeyEnter {
                break;
            }
        }
    }
}

impl Drop for GameBoard {
    fn drop(&mut self) {
        endwin();
    }
}
