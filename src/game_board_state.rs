use rand::rngs::ThreadRng;

mod game_board_state_internal;

pub struct GameBoardState {
    game_state: [[i32; 4]; 4],
    randomiser: ThreadRng,
}

impl GameBoardState {
    pub fn new() -> Self {
        let mut instance = Self {
            game_state: [[0; 4]; 4],
            randomiser: rand::thread_rng(),
        };
        for _ in 0..=1 {
            instance.initialise_tile();
        }
        instance
    }
    pub fn get_state(&self) -> [[i32; 4]; 4] {
        self.game_state
    }
}
