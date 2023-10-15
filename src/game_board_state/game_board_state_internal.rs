use super::GameBoardState;

impl GameBoardState {
    pub(crate) fn initialise_certain_tile(&mut self, y: i32, x: i32) {
        if self.is_tile_initialised(y, x) {
            self.initialise_certain_tile(y, x)
        }
        self.update_certain_tile_internal(y, x, self.generate_new_tile_value());
    }

    pub(crate) fn is_tile_initialised(&self, y: i32, x: i32) -> bool {
        self.game_state[y as usize][x as usize] != 0
    }

    pub(crate) fn generate_new_tile_value(&self) -> i32 {
        2
    }

    pub(crate) fn update_certain_tile_internal(&mut self, y: i32, x: i32, value: i32) {
        self.game_state[y as usize][x as usize] = value;
    }
}
