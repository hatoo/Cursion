use cursion::style::Style;
use cursion::term::{Term, TermWriter};
use std::io::{stdout, Write};
use std::thread;
use std::time;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = MouseTerminal::from(stdout()).into_raw_mode().unwrap();

    for i in 0..120 {
        let mut t = Term::with_terminal_size().unwrap();
        let mut cursor = None;
        {
            let mut w = TermWriter::new(&mut t);
            for j in 0..800 {
                if j % 40 == 0 {
                    w.newline();
                }
                if i == j {
                    cursor = w.write('b', Style::default());
                } else {
                    w.write('a', Style::default());
                }
            }
        }
        t.cursor = cursor.map(|(row, col)| cursion::cursor::Cursor {
            row,
            col,
            shape: cursion::cursor::CursorShape::SteadyUnderline,
        });
        t.draw(&mut stdout).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_secs(1) / 120);
    }
}
