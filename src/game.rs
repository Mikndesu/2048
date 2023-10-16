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
                Input::KeyUp => self.game_baord.move_tiles(Direction::UP),
                Input::KeyDown => self.game_baord.move_tiles(Direction::DOWN),
                Input::KeyRight => self.game_baord.move_tiles(Direction::RIGHT),
                Input::KeyLeft => self.game_baord.move_tiles(Direction::LEFT),
                _ => (),
            }
        }
    }
}
