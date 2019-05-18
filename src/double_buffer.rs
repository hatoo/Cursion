use crate::style::{DiffStyle, Style};
use crate::term::{Term, Tile};
use std::io;
use unicode_width::UnicodeWidthChar;

#[derive(Debug, Clone)]
pub struct DoubleBuffer {
    front: Term,
    pub back: Term,
}

impl DoubleBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            front: Term::new(height, width),
            back: Term::new(height, width),
        }
    }

    pub fn with_terminal_size() -> io::Result<Self> {
        Ok(Self {
            front: Term::with_terminal_size()?,
            back: Term::with_terminal_size()?,
        })
    }

    pub fn present<T: io::Write>(&mut self, out: &mut T) -> io::Result<()> {
        if (self.front.height(), self.front.width()) != (self.back.height(), self.back.width()) {
            self.back.draw(out)?;
        } else {
            write!(out, "{}", termion::cursor::Hide)?;
            let mut current_style = Style::default();
            write!(out, "{}", current_style)?;

            for (row, (f, b)) in self
                .front
                .buf()
                .iter()
                .zip(self.back.buf().iter())
                .enumerate()
            {
                let mut current_col = None;
                for (col, (f, b)) in f.iter().zip(b.iter()).enumerate() {
                    match b {
                        Tile::Char(ch, style) => {
                            if b != f {
                                let w = ch.width().unwrap_or_default();
                                if current_col != Some(col) {
                                    write!(
                                        out,
                                        "{}",
                                        termion::cursor::Goto(col as u16 + 1, row as u16 + 1)
                                    )?;
                                }
                                write!(
                                    out,
                                    "{}{}",
                                    DiffStyle {
                                        from: current_style,
                                        to: *style
                                    },
                                    ch
                                )?;

                                current_style = *style;
                                current_col = Some(col + w);
                            }
                        }
                        Tile::Empty => {
                            if current_col != Some(col) {
                                write!(
                                    out,
                                    "{}",
                                    termion::cursor::Goto(col as u16 + 1, row as u16 + 1)
                                )?;
                            }
                            write!(
                                out,
                                "{}{}",
                                DiffStyle {
                                    from: current_style,
                                    to: Style::default()
                                },
                                ' '
                            )?;

                            current_style = Style::default();
                            current_col = Some(col + 1);
                        }
                        Tile::Occupied => {}
                    }
                }
            }
        }

        if let Some(cursor) = self.back.cursor {
            write!(
                out,
                "{}{}{}",
                termion::cursor::Goto(cursor.col as u16 + 1, cursor.row as u16 + 1),
                cursor.shape,
                termion::cursor::Show
            )?;
        } else {
            write!(out, "{}", termion::cursor::Hide)?;
        }
        std::mem::swap(&mut self.front, &mut self.back);
        Ok(())
    }
}
