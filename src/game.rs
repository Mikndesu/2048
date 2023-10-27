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
            self.game_baord.render();
            let ch = self.game_baord.get_input();
            match ch.unwrap() {
                Input::Character('q') => break,
                Input::Character('r') => self.restore_progress(),
                Input::Character('s') => self.save_progress(),
                Input::Character('a') => self.start_new(),
                Input::KeyUp => self.game_baord.move_tiles(Direction::UP),
                Input::KeyDown => self.game_baord.move_tiles(Direction::DOWN),
                Input::KeyRight => self.game_baord.move_tiles(Direction::RIGHT),
                Input::KeyLeft => self.game_baord.move_tiles(Direction::LEFT),
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
