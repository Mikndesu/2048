use crate::{direction::Direction, tiles::Tiles};
use rand::rngs::ThreadRng;

mod tiles_state_internal;

pub struct TilesState {
    game_state: Tiles,
    randomiser: ThreadRng,
}

impl TilesState {
    pub fn new() -> Self {
        let mut instance = Self {
            game_state: Tiles::new([[0; 4]; 4]),
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

    pub fn move_tiles(&mut self, direction: Direction) -> bool {
        let mut is_move_successful = false;
        match direction {
            Direction::UP => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_column(i);
                    slice.reverse();
                    (slice, is_move_successful) = self.move_tiles_internal(slice);
                    slice.reverse();
                    self.game_state = self.game_state.set_column(i, slice);
                }
            }
            Direction::DOWN => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_column(i);
                    (slice, is_move_successful) = self.move_tiles_internal(slice);
                    self.game_state = self.game_state.set_column(i, slice);
                }
            }
            Direction::RIGHT => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_row(i);
                    (slice, is_move_successful) = self.move_tiles_internal(slice);
                    self.game_state = self.game_state.set_row(i, slice);
                }
            }
            Direction::LEFT => {
                for i in 0..4 {
                    let mut slice = self.game_state.get_row(i);
                    slice.reverse();
                    (slice, is_move_successful) = self.move_tiles_internal(slice);
                    slice.reverse();
                    self.game_state = self.game_state.set_row(i, slice);
                }
            }
        }
        is_move_successful
    }
}

#[test]
fn test_move_tiles() {
    let mut tiles_state = TilesState::new();
    tiles_state.game_state = Tiles::new([[0; 4]; 4]);
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
