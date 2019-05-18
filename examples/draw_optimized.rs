use cursion::double_buffer::DoubleBuffer;
use cursion::style::Style;
use cursion::term::{Term, TermWriter};
use std::io::{stdout, Write};
use std::thread;
use std::time;

fn main() {
    let mut stdout = stdout();
    let mut double_buffer = DoubleBuffer::with_terminal_size().unwrap();

    for i in 0..240 {
        double_buffer.back.clear_with_terminal_size().unwrap();
        let mut cursor = None;
        {
            let mut w = TermWriter::new(&mut double_buffer.back);
            w.write_str("You may notice flickering", Style::default());
            w.newline();
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
        double_buffer.back.cursor = cursor.map(|(row, col)| cursion::cursor::Cursor {
            row,
            col,
            shape: Default::default(),
        });
        double_buffer.present(&mut stdout).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_secs(1) / 120);
    }
}
