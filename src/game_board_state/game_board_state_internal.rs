use super::GameBoardState;
use rand::Rng;

impl GameBoardState {
    pub(crate) fn initialise_tile(&mut self) {
        let y = self.random_coordinate();
        let x = self.random_coordinate();
        if self.is_tile_initialised(y, x) {
            self.initialise_tile()
        }
        let new_value = self.generate_new_tile_value();
        self.update_certain_tile_internal(y, x, new_value);
    }

    pub(crate) fn is_tile_initialised(&self, y: i32, x: i32) -> bool {
        self.game_state[y as usize][x as usize] != 0
    }

    pub(crate) fn generate_new_tile_value(&mut self) -> i32 {
        if self.randomiser.gen_range(1..10) == 1 {
            4
        } else {
            2
        }
    }

    fn update_certain_tile_internal(&mut self, y: i32, x: i32, value: i32) {
        self.game_state[y as usize][x as usize] = value;
    }

    fn random_coordinate(&mut self) -> i32 {
        self.randomiser.gen_range(0..3)
    }
}
