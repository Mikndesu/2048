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

    pub fn serialise(&self, tiles: &Tiles) {
        self.write(&self.progress_file, tiles);
    }

    pub fn desirialise(&self) -> Option<Tiles> {
        self.read(&self.progress_file)
    }

    fn write(&self, path: &PathBuf, tiles: &Tiles) {
        let file = match File::create(path) {
            Err(why) => panic!("{why}"),
            Ok(file) => file,
        };
        let mut buf_writer = BufWriter::new(file);
        tiles.as_array().iter().for_each(|y| -> () {
            y.iter().for_each(|&x| -> () {
                match buf_writer.write(x.clone().to_ne_bytes().as_slice()) {
                    Ok(_) => (),
                    Err(why) => panic!("{}", why),
                }
            })
        });
    }

    fn read(&self, path: &PathBuf) -> Option<Tiles> {
        let file = match File::open(path) {
            Err(_) => return None,
            Ok(file) => file,
        };
        let mut array = [[0; 4]; 4];
        let mut buf_reader = BufReader::new(file);
        array.iter_mut().for_each(|y| -> () {
            y.iter_mut().for_each(|x| -> () {
                let mut buffer = mem::MaybeUninit::<[u8; 4]>::uninit();
                let buffer = unsafe { &mut *buffer.as_mut_ptr() };
                buf_reader.read_exact(buffer).unwrap();
                *x = i32::from_ne_bytes(
                    <[u8; 4]>::try_from(*buffer).expect("Oops, something went wrong..."),
                );
            })
        });
        return if array != [[0; 4]; 4] {
            Some(Tiles::new(array))
        } else {
            None
        };
    }
}

#[test]
fn test_read() {
    use std::env;
    let data = ProgressData::new();
    let tile = Tiles::new([[1, 2, 3, 4]; 4]);
    let current_dir = env::current_dir().unwrap();
    let tile1 = data.read(&current_dir.join("test/test_read.dat"));
    assert_eq!(tile1.unwrap().as_array(), tile.as_array());
}

#[test]
fn test_write() {
    use std::env;
    let current_dir = env::current_dir().unwrap();
    let data = ProgressData::new();
    let tile = Tiles::new([[1, 2, 3, 4]; 4]);
    let tile1 = Tiles::new([[5; 4]; 4]);
    data.write(&current_dir.join("test/test_write.dat"), &tile);
    data.write(&current_dir.join("test/wrong.dat"), &tile1);
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
