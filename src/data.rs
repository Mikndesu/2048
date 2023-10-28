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

impl ProgressData {
    pub fn new() -> Self {
        let home_dir = match home::home_dir() {
            Some(path) => path,
            None => panic!("Impossible to get Home Directory!"),
        };
        Self {
            progress_file: home_dir.join(".config/2048/progress.dat"),
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
            Err(why) => panic!("{why}"),
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
