use cursion::double_buffer::DoubleBuffer;
use cursion::style::Style;
use cursion::term::TermWriter;
use std::io::{stdout, Write};
use std::thread;
use std::time;
use cursion::color::TermColor;
use cursion::decoration::{Decoration, Decorations};

fn main() {
    let mut stdout = stdout();
    let mut double_buffer = DoubleBuffer::with_terminal_size().unwrap();

    for i in 0..240 {
        double_buffer.back.clear_with_terminal_size().unwrap();
        let mut cursor = None;
        let mut w = TermWriter::new(&mut double_buffer.back);
        w.write_str("You don't notice flickering", Style::default());
        w.newline();
        for j in 0..800 {
            if j % 40 == 0 {
                w.newline();
            }
            if i == j {
                cursor = w.write('b', Style {
                    bg: TermColor::Blue,
                    fg: TermColor::White,
                    decorations: Decorations::default().with(Decoration::Bold),
                });
            } else {
                w.write('a', Style {
                    bg: TermColor::Red,
                    fg: TermColor::White,
                    decorations: Decorations::default().with(Decoration::Invert),
                });
            }
        }
        double_buffer.back.cursor = cursor.map(|(row, col)| cursion::cursor::Cursor {
            row,
            col,
            shape: Default::default(),
        });
        double_buffer.present(&mut stdout).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_secs(1) / 120);
    }

    write!(stdout, "{}", Style::default()).unwrap();
}
