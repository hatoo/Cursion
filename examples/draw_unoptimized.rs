use cursion::style::Style;
use cursion::term::{Term, TermWriter};
use std::io::{stdout, Write};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = MouseTerminal::from(stdout()).into_raw_mode().unwrap();
    let mut t = Term::with_terminal_size().unwrap();
    {
        let mut w = TermWriter::new(&mut t);
        for i in 0..100 {
            if i % 25 == 0 {
                w.newline();
            }
            w.write('a', Style::default()).unwrap();
        }
    }
    t.draw(&mut stdout).unwrap();
    stdout.flush().unwrap();
}
