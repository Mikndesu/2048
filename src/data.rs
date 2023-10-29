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

use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    mem,
    path::PathBuf,
};

use crate::tiles::Tiles;
pub struct ProgressData {
    progress_file: PathBuf,
}

pub struct ScoreData {
    score_file: File,
}

impl ScoreData {}

impl ProgressData {
    pub fn new() -> Self {
        let home_dir = match home::home_dir() {
            Some(path) => path,
            None => panic!("Impossible to get Home Directory!"),
        };
        let parent_dir = home_dir.join(".config").join("2048");
        if let Err(why) = std::fs::create_dir_all(&parent_dir) {
            panic!("{why}");
        }
        Self {
            progress_file: parent_dir.join("progress.dat"),
        }
    }

    pub fn serialise(&self, tiles: &Tiles, score: i32) {
        self.write(&self.progress_file, tiles, score);
    }

    pub fn desirialise(&self) -> Option<(Tiles, i32)> {
        self.read(&self.progress_file)
    }

    fn write(&self, path: &PathBuf, tiles: &Tiles, score: i32) {
        let file = match File::create(path) {
            Err(why) => panic!("{} {why}", path.display()),
            Ok(file) => file,
        };
        let mut buf_writer = BufWriter::new(file);
        tiles.as_array().iter().for_each(|y| {
            y.iter()
                .for_each(|&x| match buf_writer.write(x.to_ne_bytes().as_slice()) {
                    Ok(_) => (),
                    Err(why) => panic!("{}", why),
                })
        });
        match buf_writer.write(score.to_ne_bytes().as_slice()) {
            Ok(_) => (),
            Err(why) => panic!("{}", why),
        }
    }

    fn read(&self, path: &PathBuf) -> Option<(Tiles, i32)> {
        let file = match File::open(path) {
            Err(_) => return None,
            Ok(file) => file,
        };
        let mut buf_reader = BufReader::new(file);
        #[inline]
        fn read_int_from_bin(buf_reader: &mut BufReader<File>) -> i32 {
            let mut buffer = mem::MaybeUninit::<[u8; 4]>::uninit();
            let buffer = unsafe { &mut *buffer.as_mut_ptr() };
            buf_reader.read_exact(buffer).unwrap();
            i32::from_ne_bytes(*buffer)
        }
        let mut array = [[0; 4]; 4];
        array.iter_mut().for_each(|y| {
            y.iter_mut().for_each(|x| {
                *x = read_int_from_bin(&mut buf_reader);
            })
        });
        let score = read_int_from_bin(&mut buf_reader);
        if array != [[0; 4]; 4] {
            Some((Tiles::new(array), score))
        } else {
            None
        }
    }
}

#[test]
fn test_read() {
    use std::env;
    let data = ProgressData::new();
    let tile = Tiles::new([[1, 2, 3, 4]; 4]);
    let current_dir = env::current_dir().unwrap();
    let tile1 = data.read(&current_dir.join("test/test_read.dat"));
    assert_eq!(tile1.as_ref().unwrap().0.as_array(), tile.as_array());
    assert_eq!(tile1.as_ref().unwrap().1, 4097);
}

#[test]
fn test_write() {
    use std::env;
    let current_dir = env::current_dir().unwrap();
    let data = ProgressData::new();
    let tile = Tiles::new([[1, 2, 3, 4]; 4]);
    let tile1 = Tiles::new([[5; 4]; 4]);
    data.write(&current_dir.join("test/test_write.dat"), &tile, 4097);
    data.write(&current_dir.join("test/wrong.dat"), &tile1, 1000);
    let mut expected = match File::open(current_dir.join("test/test_write_expected.dat")) {
        Ok(f) => f,
        Err(why) => panic!("{}", why),
    };
    let mut write_into = match File::open(current_dir.join("test/test_write.dat")) {
        Ok(f) => f,
        Err(why) => panic!("{}", why),
    };
    let mut wrong = match File::open(current_dir.join("test/wrong.dat")) {
        Ok(f) => f,
        Err(why) => panic!("{}", why),
    };
    assert!(file_diff::diff_files(&mut expected, &mut write_into));
    assert!(!file_diff::diff_files(&mut expected, &mut wrong));
}
