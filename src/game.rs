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

    pub fn start(&self) {
        loop {
            self.game_baord.render();
            let ch = self.game_baord.get_input();
            if ch.unwrap() == pancurses::Input::Character('q') {
                break;
            }
        }
    }
}
