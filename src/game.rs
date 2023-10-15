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
        use pancurses::Input;
        loop {
            self.game_baord.render();
            let ch = self.game_baord.get_input();
            match ch.unwrap() {
                Input::Character('q') => break,
                Input::KeyUp => break,
                Input::KeyDown => break,
                Input::KeyRight => break,
                Input::KeyLeft => break,
                _ => (),
            }
        }
    }
}
