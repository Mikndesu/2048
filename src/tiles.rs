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

#[derive(Debug)]
pub struct Tiles {
    tiles: [[i32; 4]; 4],
}

impl From<[[i32; 4]; 4]> for Tiles {
    fn from(value: [[i32; 4]; 4]) -> Self {
        Tiles { tiles: value }
    }
}

impl From<Tiles> for [[i32; 4]; 4] {
    fn from(value: Tiles) -> Self {
        value.tiles
    }
}

impl AsRef<[[i32; 4]; 4]> for Tiles {
    fn as_ref(&self) -> &[[i32; 4]; 4] {
        &self.tiles
    }
}

impl std::ops::Index<usize> for Tiles {
    type Output = [i32; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl std::ops::IndexMut<usize> for Tiles {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl std::fmt::Display for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tiles)?;
        Ok(())
    }
}

impl Tiles {
    pub fn get_column(&self, i: i32) -> [i32; 4] {
        self.tiles[i as usize]
    }

    pub fn get_row(&self, i: i32) -> [i32; 4] {
        let mut arr: [i32; 4] = [0; 4];
        arr.iter_mut()
            .zip(self.tiles.iter())
            .for_each(|(x, y)| *x = y[i as usize]);
        arr
    }

    pub fn set_column(&self, i: i32, column: [i32; 4]) -> Box<Tiles> {
        let mut tiles = self.tiles;
        tiles[i as usize] = column;
        Box::new(Self { tiles })
    }

    pub fn set_row(&self, i: i32, row: [i32; 4]) -> Box<Tiles> {
        let mut tiles = self.tiles;
        tiles
            .iter_mut()
            .zip(row.iter())
            .for_each(|(x, y)| x[i as usize] = *y);
        Box::new(Self { tiles })
    }

    pub fn clear_all(&mut self) {
        self.tiles = [[0; 4]; 4];
    }
}

#[test]
fn test_tiles_index_mut() {
    let mut tiles: Tiles = [[0; 4]; 4].into();
    assert_eq!(tiles[0], [0, 0, 0, 0]);
    tiles[0] = [1; 4];
    assert_eq!(tiles[0], [1, 1, 1, 1]);
    tiles[2] = [1, 2, 3, 4];
    assert_eq!(tiles[2], [1, 2, 3, 4])
}

#[test]
fn test_get_certain_column() {
    let tiles: Tiles = [[1, 2, 3, 4]; 4].into();
    assert_eq!(tiles.get_column(2), [1, 2, 3, 4]);
}

#[test]
fn test_get_certain_row() {
    let tiles: Tiles = [[1, 2, 3, 4]; 4].into();
    assert_eq!(tiles.get_row(2), [3; 4]);
}

#[test]
fn test_set_certain_row() {
    let tiles: Tiles = [[1, 2, 3, 4]; 4].into();
    let a: [[i32; 4]; 4] = (*tiles.set_row(2, [5; 4])).into();
    assert_eq!(a, [[1, 2, 5, 4]; 4]);
}
