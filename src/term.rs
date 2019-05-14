use crate::cursor::Cursor;
use crate::style::Style;

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

impl Term {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            buf: vec![vec![Tile::Empty; width]; height],
            cursor: None,
        }
    }
}
