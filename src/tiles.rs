#[derive(Debug)]
pub struct Tiles {
    tiles: [[i32; 4]; 4],
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
    pub fn new(array: [[i32; 4]; 4]) -> Self {
        Self { tiles: array }
    }

    pub fn as_array(&self) -> [[i32; 4]; 4] {
        return self.tiles;
    }

    pub fn get_column(&self, i: i32) -> [i32; 4] {
        self.tiles[i as usize]
    }

    pub fn get_row(&self, i: i32) -> [i32; 4] {
        let mut arr: [i32; 4] = [0; 4];
        arr.iter_mut()
            .zip(self.tiles.iter())
            .for_each(|(x, y)| *x = y[i as usize]);
        return arr;
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
}

#[test]
fn test_tiles_index_mut() {
    let mut tiles = Tiles::new([[0; 4]; 4]);
    assert_eq!(tiles[0], [0, 0, 0, 0]);
    tiles[0] = [1; 4];
    assert_eq!(tiles[0], [1, 1, 1, 1]);
    tiles[2] = [1, 2, 3, 4];
    assert_eq!(tiles[2], [1, 2, 3, 4])
}

#[test]
fn test_get_certain_column() {
    let tiles = Tiles::new([[1, 2, 3, 4]; 4]);
    assert_eq!(tiles.get_column(2), [1, 2, 3, 4]);
}

#[test]
fn test_get_certain_row() {
    let tiles = Tiles::new([[1, 2, 3, 4]; 4]);
    assert_eq!(tiles.get_row(2), [3; 4]);
}

#[test]
fn test_set_certain_row() {
    let tiles = Tiles::new([[1, 2, 3, 4]; 4]);
    assert_eq!(tiles.set_row(2, [5; 4]).as_array(), [[1, 2, 5, 4]; 4]);
}
