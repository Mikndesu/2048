mod game_board_state_internal;

pub struct GameBoardState {
    game_state: [[i32; 4]; 4],
}

impl GameBoardState {
    pub fn new() -> Self {
        Self {
            game_state: [[0; 4]; 4],
        }
    }
    pub fn get_state(&self) -> [[i32; 4]; 4] {
        self.game_state
    }
}
