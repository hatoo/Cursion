//! Cursion is a pure Rust, cursor optimization library to avoid flickering in terminal like Curses.
#![allow(clippy::write_with_newline, clippy::write_literal)]
#![warn(missing_docs)]
pub mod color;
pub mod cursor;
pub mod decoration;
pub mod double_buffer;
pub mod style;
pub mod term;
