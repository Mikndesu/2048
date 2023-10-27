use crate::{direction::Direction, tiles::Tiles};
use rand::rngs::ThreadRng;

mod tiles_state_internal;

pub struct TilesState {
    pub game_state: Box<Tiles>,
    randomiser: ThreadRng,
}

impl TilesState {
    pub fn new() -> Self {
        let mut instance = Self {
            game_state: Box::new(Tiles::new([[0; 4]; 4])),
            randomiser: rand::thread_rng(),
        };
        for _ in 0..=1 {
            instance.initialise_tile();
        }
        instance
    }

    pub fn get_state(&self) -> &Tiles {
        &self.game_state
    }

    pub fn move_tiles(&mut self, direction: Direction) -> (bool, i32) {
        let mut is_move_successful = false;
        let mut score_increase = 0;
        match direction {
            Direction::UP => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_column(i);
                    slice.reverse();
                    let (mut slice, flag, j) = self.move_tiles_internal(slice);
                    slice.reverse();
                    self.game_state = self.game_state.set_column(i, slice);
                    is_move_successful = is_move_successful || flag;
                    score_increase += j;
                }
            }
            Direction::DOWN => {
                for i in 0..4 {
                    let slice = self.game_state.get_column(i);
                    let (slice, flag, j) = self.move_tiles_internal(slice);
                    self.game_state = self.game_state.set_column(i, slice);
                    is_move_successful = is_move_successful || flag;
                    score_increase += j;
                }
            }
            Direction::RIGHT => {
                for i in 0..4 {
                    let slice = self.game_state.get_row(i);
                    let (slice, flag, j) = self.move_tiles_internal(slice);
                    self.game_state = self.game_state.set_row(i, slice);
                    is_move_successful = is_move_successful || flag;
                    score_increase += j;
                }
            }
            Direction::LEFT => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_row(i);
                    slice.reverse();
                    let (mut slice, flag, j) = self.move_tiles_internal(slice);
                    slice.reverse();
                    self.game_state = self.game_state.set_row(i, slice);
                    is_move_successful = is_move_successful || flag;
                    score_increase += j;
                }
            }
        }
        (is_move_successful, score_increase)
    }

    pub fn clear(&mut self) {
        self.game_state.clear_all();
        for _ in 0..=1 {
            self.initialise_tile();
        }
    }
}

#[test]
fn test_move_tiles() {
    let mut tiles_state = TilesState::new();
    tiles_state.game_state = Box::new(Tiles::new([[0; 4]; 4]));
    tiles_state.game_state[2][1] = 2;
    tiles_state.game_state[3][1] = 2;
    tiles_state.game_state[1][2] = 4;
    tiles_state.game_state[0][3] = 8;
    println!("{:?}", tiles_state.game_state);
    tiles_state.move_tiles(Direction::RIGHT);
    assert_eq!(
        tiles_state.game_state.as_array(),
        [[0; 4], [0; 4], [0; 4], [0, 4, 4, 8]]
    );
}
