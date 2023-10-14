use pancurses::{ToChtype, Window};

pub trait WindowExt {
    fn mvhline<T: ToChtype>(&self, y: i32, x: i32, ch: T, n: i32) -> i32;
    fn mvvline<T: ToChtype>(&self, y: i32, x: i32, ch: T, n: i32) -> i32;
}

impl WindowExt for Window {
    fn mvvline<T: ToChtype>(&self, y: i32, x: i32, ch: T, n: i32) -> i32 {
        self.mv(y, x);
        self.vline(ch, n)
    }

    fn mvhline<T: ToChtype>(&self, y: i32, x: i32, ch: T, n: i32) -> i32 {
        self.mv(y, x);
        self.hline(ch, n)
    }
}
