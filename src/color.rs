use std::fmt;
use termion::color;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TermColor {
    AnsiValue(u8),
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    Reset,
    Rgb(u8, u8, u8),
    White,
    Yellow,
}

impl termion::color::Color for TermColor {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermColor::AnsiValue(v) => color::AnsiValue(*v).write_fg(f),
            TermColor::Black => color::Black.write_fg(f),
            TermColor::Blue => color::Blue.write_fg(f),
            TermColor::Cyan => color::Cyan.write_fg(f),
            TermColor::Green => color::Green.write_fg(f),
            TermColor::LightBlack => color::LightBlack.write_fg(f),
            TermColor::LightBlue => color::LightBlue.write_fg(f),
            TermColor::LightCyan => color::LightCyan.write_fg(f),
            TermColor::LightGreen => color::LightGreen.write_fg(f),
            TermColor::LightMagenta => color::LightMagenta.write_fg(f),
            TermColor::LightRed => color::LightRed.write_fg(f),
            TermColor::LightWhite => color::LightWhite.write_fg(f),
            TermColor::LightYellow => color::LightYellow.write_fg(f),
            TermColor::Magenta => color::Magenta.write_fg(f),
            TermColor::Red => color::Red.write_fg(f),
            TermColor::Reset => color::Reset.write_fg(f),
            TermColor::Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_fg(f),
            TermColor::White => color::White.write_fg(f),
            TermColor::Yellow => color::Yellow.write_fg(f),
        }
    }
    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TermColor::AnsiValue(v) => color::AnsiValue(*v).write_bg(f),
            TermColor::Black => color::Black.write_bg(f),
            TermColor::Blue => color::Blue.write_bg(f),
            TermColor::Cyan => color::Cyan.write_bg(f),
            TermColor::Green => color::Green.write_bg(f),
            TermColor::LightBlack => color::LightBlack.write_bg(f),
            TermColor::LightBlue => color::LightBlue.write_bg(f),
            TermColor::LightCyan => color::LightCyan.write_bg(f),
            TermColor::LightGreen => color::LightGreen.write_bg(f),
            TermColor::LightMagenta => color::LightMagenta.write_bg(f),
            TermColor::LightRed => color::LightRed.write_bg(f),
            TermColor::LightWhite => color::LightWhite.write_bg(f),
            TermColor::LightYellow => color::LightYellow.write_bg(f),
            TermColor::Magenta => color::Magenta.write_bg(f),
            TermColor::Red => color::Red.write_bg(f),
            TermColor::Reset => color::Reset.write_bg(f),
            TermColor::Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_bg(f),
            TermColor::White => color::White.write_bg(f),
            TermColor::Yellow => color::Yellow.write_bg(f),
        }
    }
}
