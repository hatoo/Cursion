use cursion::double_buffer::DoubleBuffer;
use cursion::style::Style;
use cursion::term::TermWriter;
use std::io::stdout;
use std::thread;
use std::time;

fn main() {
    let mut stdout = stdout();
    let mut double_buffer = DoubleBuffer::with_terminal_size().unwrap();

    for i in 0..100 {
        double_buffer.back.clear_with_terminal_size().unwrap();
        let mut w = TermWriter::new(&mut double_buffer.back);
        for j in 0..100 {
            w.write_str(&format!("{}", i + j), Style::default());
        }
        double_buffer.present(&mut stdout).unwrap();
        thread::sleep(time::Duration::from_secs(1));
    }
}
