/*
Copyright (c) 2023 Mikndesu

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

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
            game_state: Box::new([[0; 4]; 4].into()),
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

    pub fn has_empty_tile(&self) -> bool {
        (*self.game_state)
            .as_ref()
            .iter()
            .any(|y| y.iter().any(|x| *x == 0))
    }

    // this function should not be called except when there are not any empty tiles.
    pub fn is_movable(&self) -> bool {
        let mut flag: bool = false;
        #[inline]
        fn same_pair_exists(arr: [i32; 4]) -> bool {
            let mut flag: bool = false;
            let mut it = arr.iter().peekable();
            while let Some(x) = it.next() {
                if let Some(next) = it.peek() {
                    flag = flag || *x == **next;
                };
            }
            flag
        }
        for i in 0..4 {
            let column = self.game_state.get_column(i);
            let row = self.game_state.get_row(i);
            flag = flag || same_pair_exists(column) || same_pair_exists(row);
        }
        flag
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
fn test_has_empty_tiles() {
    let mut tiles_state = TilesState::new();
    tiles_state.game_state = Box::new([[2; 4]; 4].into());
    assert_eq!(tiles_state.has_empty_tile(), false);
    tiles_state.game_state[2][1] = 0;
    assert_eq!(tiles_state.has_empty_tile(), true);
}

#[test]
fn test_is_movable() {
    let mut tiles_state = TilesState::new();
    tiles_state.game_state = Box::new([[2; 4]; 4].into());
    assert_eq!(tiles_state.is_movable(), true);
    tiles_state.game_state = Box::new(Tiles::from([
        [1, 2, 3, 4],
        [4, 3, 2, 1],
        [1, 2, 3, 4],
        [4, 3, 2, 1],
    ]));
    assert_eq!(tiles_state.is_movable(), false);
}

#[test]
fn test_move_tiles() {
    let mut tiles_state = TilesState::new();
    tiles_state.game_state = Box::new([[0; 4]; 4].into());
    tiles_state.game_state[2][1] = 2;
    tiles_state.game_state[3][1] = 2;
    tiles_state.game_state[1][2] = 4;
    tiles_state.game_state[0][3] = 8;
    println!("{:?}", tiles_state.game_state);
    let (flag, i) = tiles_state.move_tiles(Direction::Right);
    assert_eq!(
        *(*tiles_state.game_state).as_ref(),
        [[0; 4], [0; 4], [0; 4], [0, 4, 4, 8]]
    );
    assert_eq!(flag, true);
    assert_eq!(i, 4);
}
