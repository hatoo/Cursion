use crate::cursor::Cursor;
use crate::style::Style;
use std::fmt;
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Occupied,
    Char(char, Style),
}

pub struct Term {
    height: usize,
    width: usize,
    buf: Vec<Vec<Tile>>,
    pub cursor: Option<Cursor>,
}

#[derive(Debug)]
pub enum Error {
    NotEnoughSpace,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Not enough space to set".fmt(f)
    }
}
impl std::error::Error for Error {}

impl Term {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            buf: vec![vec![Tile::Empty; width]; height],
            cursor: None,
        }
    }

    pub fn set_char_at(
        &mut self,
        row: usize,
        col: usize,
        ch: char,
        style: Style,
    ) -> Result<(), Error> {
        assert!(row < self.height);
        assert!(col < self.width);

        if let Some(w) = ch.width() {
            if w == 0 {
                return Ok(());
            }

            if col + w >= self.width {
                return Err(Error::NotEnoughSpace);
            }

            let mut l = col;
            let mut r = col + w - 1;

            while l > 0 && self.buf[row][l] == Tile::Occupied {
                l -= 1;
            }

            if let Tile::Char(_, _) = self.buf[row][r] {
                r += 1;
            }

            while r < self.width && self.buf[row][r] == Tile::Occupied {
                r += 1;
            }

            for i in l..r {
                self.buf[row][i] = Tile::Empty;
            }

            self.buf[row][col] = Tile::Char(ch, style);
            for i in col + 1..col + w {
                self.buf[row][i] = Tile::Occupied;
            }
        }
        Ok(())
    }
}

#[test]
fn test_set_char_at() {
    let mut t = Term::new(1, 4);
    t.set_char_at(0, 0, 'あ', Style::default()).unwrap();
    assert_eq!(
        t.buf,
        vec![vec![
            Tile::Char('あ', Style::default()),
            Tile::Occupied,
            Tile::Empty,
            Tile::Empty
        ]]
    );
    t.set_char_at(0, 1, 'い', Style::default()).unwrap();
    assert_eq!(
        t.buf,
        vec![vec![
            Tile::Empty,
            Tile::Char('い', Style::default()),
            Tile::Occupied,
            Tile::Empty
        ]]
    );
    t.set_char_at(0, 0, 'う', Style::default()).unwrap();
    assert_eq!(
        t.buf,
        vec![vec![
            Tile::Char('う', Style::default()),
            Tile::Occupied,
            Tile::Empty,
            Tile::Empty
        ]]
    );
    t.set_char_at(0, 2, 'a', Style::default()).unwrap();
    assert_eq!(
        t.buf,
        vec![vec![
            Tile::Char('う', Style::default()),
            Tile::Occupied,
            Tile::Char('a', Style::default()),
            Tile::Empty
        ]]
    );
    t.set_char_at(0, 1, 'b', Style::default()).unwrap();
    assert_eq!(
        t.buf,
        vec![vec![
            Tile::Empty,
            Tile::Char('b', Style::default()),
            Tile::Char('a', Style::default()),
            Tile::Empty
        ]]
    );
}
