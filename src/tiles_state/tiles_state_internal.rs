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

use super::TilesState;
use rand::{seq::SliceRandom, Rng};

macro_rules! reverse_if {
    ($array:expr, $flag:expr) => {
        if $flag {
            $array.reverse();
        }
    };
}

impl TilesState {
    pub(crate) fn initialise_tile(&mut self) {
        let mut vec: Vec<(i32, i32)> = vec![];
        (*self.game_state)
            .as_ref()
            .iter()
            .enumerate()
            .for_each(|(i, y)| {
                y.iter().enumerate().for_each(|(j, _)| {
                    if !self.is_tile_initialised(i as i32, j as i32) {
                        vec.push((i as i32, j as i32));
                    }
                })
            });
        if let Some(&(y, x)) = vec.choose(&mut self.randomiser) {
            let new_value = self.generate_new_tile_value();
            self.update_certain_tile_internal(y, x, new_value);
        }
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

    pub(crate) fn move_tiles_internal(
        &self,
        mut array: [i32; 4],
        is_reverse_move: bool,
    ) -> ([i32; 4], bool, i32) {
        reverse_if!(array, is_reverse_move);
        let (arr, flag1, score_increase) = Self::merge_matching_pair(array);
        let (mut arr2, flag2) = Self::move_after_merge(arr);
        reverse_if!(arr2, is_reverse_move);
        // return true when either of methods return true
        (arr2, flag1 | flag2, score_increase)
    }

    fn merge_matching_pair(mut arr: [i32; 4]) -> ([i32; 4], bool, i32) {
        let mut is_move_successful = false;
        let mut score_increase = 0;
        arr.reverse();
        let mut from = 0;
        while from < 4 {
            let from_value = arr[from];
            if from_value != 0 {
                const NOT_FOUND: usize = usize::MAX;
                let option = arr
                    .iter()
                    .skip(from + 1)
                    .enumerate()
                    .try_fold(NOT_FOUND, |opt, (i, x)| -> Option<usize> {
                        if (opt != NOT_FOUND) || *x == 0 {
                            Some(opt)
                        } else if *x == from_value {
                            Some(from + i + 1)
                        } else {
                            None
                        }
                    })
                    .and_then(|x| -> Option<usize> {
                        return if x != NOT_FOUND { Some(x) } else { None };
                    });
                if let Some(to) = option {
                    arr[from] *= 2;
                    arr[to] = 0;
                    score_increase += arr[from];
                    is_move_successful = true;
                    from += 1;
                }
            }
            from += 1;
        }
        arr.reverse();
        (arr, is_move_successful, score_increase)
    }

    fn move_after_merge(mut arr: [i32; 4]) -> ([i32; 4], bool) {
        let mut is_move_successful = false;
        arr.reverse();
        for i in 0..4 {
            if arr[i] == 0 {
                // iterate new array that skips first 'i' + 1 value
                // i.e. when 'i' = 0 then [2,0,0,4] => [0,0,4]
                // thus the index of the new array is equivalent
                // to what is added 'i+1' to the old one since
                // the index counts start at 0.
                if let Some(j) = arr.iter().skip(i + 1).position(|&x| x != 0) {
                    let equivalent_index = i + j + 1;
                    arr.swap(i, equivalent_index);
                    arr[equivalent_index] = 0;
                    is_move_successful = true;
                }
            }
        }
        arr.reverse();
        (arr, is_move_successful)
    }

    fn update_certain_tile_internal(&mut self, y: i32, x: i32, value: i32) {
        self.game_state[y as usize][x as usize] = value;
    }
}

#[test]
fn test_move_tiles_internal() {
    use crate::tiles_state::TilesState;
    let game_board_state = TilesState::new();
    assert_eq!(
        game_board_state.move_tiles_internal([2, 2, 8, 4], false),
        ([0, 4, 8, 4], true, 4)
    );
    assert_eq!(
        game_board_state.move_tiles_internal([2, 2, 8, 4], true),
        ([4, 8, 4, 0], true, 4)
    );
    assert_eq!(
        game_board_state.move_tiles_internal([2, 2, 2, 2], false),
        ([0, 0, 4, 4], true, 8)
    );
    assert_eq!(
        game_board_state.move_tiles_internal([16, 16, 16, 16], false),
        ([0, 0, 32, 32], true, 64)
    );
}

#[test]
fn test_merge_matching_pair() {
    assert_eq!(
        TilesState::merge_matching_pair([2, 0, 0, 2]),
        ([0, 0, 0, 4], true, 4)
    );
    assert_eq!(
        TilesState::merge_matching_pair([2, 2, 0, 2]),
        ([2, 0, 0, 4], true, 4)
    );
    assert_eq!(
        TilesState::merge_matching_pair([2, 2, 2, 2]),
        ([0, 4, 0, 4], true, 8)
    );
    assert_eq!(
        TilesState::merge_matching_pair([2, 0, 4, 2]),
        ([2, 0, 4, 2], false, 0)
    );
}

#[test]
fn test_move_after_merge() {
    assert_eq!(
        TilesState::move_after_merge([2, 0, 0, 2]),
        ([0, 0, 2, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([4, 0, 0, 2]),
        ([0, 0, 4, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([2, 2, 0, 2]),
        ([0, 2, 2, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([2, 2, 0, 0]),
        ([0, 0, 2, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([0, 2, 0, 2]),
        ([0, 0, 2, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([4, 8, 0, 2]),
        ([0, 4, 8, 2], true)
    );
    assert_eq!(
        TilesState::move_after_merge([2, 0, 0, 0]),
        ([0, 0, 0, 2], true)
    );
    assert_eq!(TilesState::move_after_merge([2; 4]), ([2; 4], false));
}
