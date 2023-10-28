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
        use paste::paste;
        let mut is_move_successful = false;
        let mut score_increase = 0;
        for i in 0..4 {
            let flag: bool;
            let j: i32;
            macro_rules! moving {
                (vec_type => $vec_type:ident, is_reverse_move => $is_reverse_move:expr) => {{
                    paste! {
                        let mut arr = self.game_state.[<get_ $vec_type>](i);
                        (arr, flag, j) = self.move_tiles_internal(arr, $is_reverse_move);
                        self.game_state = self.game_state.[<set_ $vec_type>](i, arr);
                    }
                }};
            }
            match direction {
                Direction::Up => moving!(vec_type=>column,is_reverse_move=>true),
                Direction::Down => moving!(vec_type=>column,is_reverse_move=>false),
                Direction::Right => moving!(vec_type=>row,is_reverse_move=>false),
                Direction::Left => moving!(vec_type=>row,is_reverse_move=>true),
            }
            is_move_successful = is_move_successful || flag;
            score_increase += j;
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
    let (flag, i) = tiles_state.move_tiles(Direction::Right);
    assert_eq!(
        tiles_state.game_state.as_array(),
        [[0; 4], [0; 4], [0; 4], [0, 4, 4, 8]]
    );
    assert_eq!(flag, true);
    assert_eq!(i, 4);
}
