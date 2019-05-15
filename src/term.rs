use crate::cursor::Cursor;
use crate::style::{DiffStyle, Style};
use std::fmt;
use std::io;
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Occupied,
    Char(char, Style),
}

#[derive(Debug)]
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

    pub fn with_terminal_size() -> io::Result<Self> {
        termion::terminal_size().map(|(col, row)| Self::new(row as usize, col as usize))
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

            if col + w > self.width {
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

    pub fn draw<T: io::Write>(&self, w: &mut T) -> io::Result<()> {
        let mut current_style = Style::default();
        write!(
            w,
            "{}{}{}",
            // 1-indexed
            termion::cursor::Goto(1, 1),
            current_style,
            termion::clear::All
        )?;

        for (current_row, line) in self.buf.iter().enumerate() {
            let mut current_col = 0;
            for (col, t) in line.iter().enumerate() {
                match t {
                    Tile::Char(ch, style) => {
                        if current_col != col {
                            current_col = col;
                            write!(
                                w,
                                "{}",
                                termion::cursor::Goto(
                                    current_col as u16 + 1,
                                    current_row as u16 + 1
                                )
                            )?;
                        }
                        if current_style != *style {
                            write!(
                                w,
                                "{}",
                                DiffStyle {
                                    from: current_style,
                                    to: *style
                                }
                            )?;
                            current_style = *style;
                        }
                        write!(w, "{}", ch)?;
                        current_col += 1;
                    }
                    Tile::Occupied => {
                        current_col += 1;
                    }
                    Tile::Empty => {}
                }
            }
            // Do not print newline in the last line.
            if current_row + 1 < self.height {
                write!(w, "\r\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct TermWriter<'a> {
    term: &'a mut Term,
    row: usize,
    col: usize,
}

impl<'a> TermWriter<'a> {
    pub fn new(term: &'a mut Term) -> Self {
        Self {
            term,
            row: 0,
            col: 0,
        }
    }

    pub fn is_writable(&self) -> bool {
        self.row < self.term.height
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.col = 0;
    }

    pub fn cause_newline(&self, ch: char) -> bool {
        if let Some(w) = ch.width() {
            self.col + w > self.term.width
        } else {
            false
        }
    }

    pub fn write(&mut self, ch: char, style: Style) -> Option<(usize, usize)> {
        if !self.is_writable() {
            return None;
        }

        let w = ch.width()?;
        if w == 0 {
            return None;
        }

        if w > self.term.width {
            return None;
        }

        if self.col + w > self.term.width {
            self.row += 1;
            self.col = 0;
        }

        if !self.is_writable() {
            return None;
        }

        let res = (self.row, self.col);
        self.term
            .set_char_at(self.row, self.col, ch, style)
            .unwrap();

        self.col += w;
        if self.col >= self.term.width {
            self.col = 0;
            self.row += 1;
        }

        Some(res)
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

#[test]
fn test_term_writer() {
    let mut t = Term::new(2, 3);
    {
        let mut w = TermWriter::new(&mut t);
        assert_eq!(w.write('a', Style::default()), Some((0, 0)));
        assert_eq!(w.write('あ', Style::default()), Some((0, 1)));
        assert_eq!(w.write('い', Style::default()), Some((1, 0)));
        assert_eq!(w.write('b', Style::default()), Some((1, 2)));
    }
    assert_eq!(
        t.buf,
        vec![
            vec![
                Tile::Char('a', Style::default()),
                Tile::Char('あ', Style::default()),
                Tile::Occupied,
            ],
            vec![
                Tile::Char('い', Style::default()),
                Tile::Occupied,
                Tile::Char('b', Style::default()),
            ]
        ]
    );
    let mut t = Term::new(2, 3);
    {
        let mut w = TermWriter::new(&mut t);
        assert_eq!(w.write('a', Style::default()), Some((0, 0)));
        w.newline();
        assert_eq!(w.write('あ', Style::default()), Some((1, 0)));
    }
    assert_eq!(
        t.buf,
        vec![
            vec![Tile::Char('a', Style::default()), Tile::Empty, Tile::Empty,],
            vec![
                Tile::Char('あ', Style::default()),
                Tile::Occupied,
                Tile::Empty,
            ]
        ]
    );
}
