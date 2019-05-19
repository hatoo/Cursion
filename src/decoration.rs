//! Character decoration

use enumset::{EnumSet, EnumSetType};
use std::fmt;
use termion::style;

#[derive(Debug, EnumSetType)]
/// Character decoration
pub enum Decoration {
    /// Blinking text (not widely supported)
    Blink,
    /// Bold text
    Bold,
    /// Fainted text (not widely supported)
    Faint,
    /// Framed text (not widely supported)
    Framed,
    /// Inverted colors (negative mode)
    Invert,
    /// Italic text
    Italic,
    /// Underlined text
    Underline,
}

impl fmt::Display for Decoration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decoration::Blink => style::Blink.fmt(f),
            Decoration::Bold => style::Bold.fmt(f),
            Decoration::Faint => style::Faint.fmt(f),
            Decoration::Framed => style::Framed.fmt(f),
            Decoration::Invert => style::Invert.fmt(f),
            Decoration::Italic => style::Italic.fmt(f),
            Decoration::Underline => style::Underline.fmt(f),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
/// Set of character decorations
pub struct Decorations(EnumSet<Decoration>);

impl Decorations {
    /// With a Decoration
    pub fn with(mut self, decoration: Decoration) -> Self {
        self.0.insert(decoration);
        self
    }

    /// insert a Decoration
    pub fn insert(&mut self, decoration: Decoration) {
        self.0.insert(decoration);
    }
}

impl fmt::Display for Decorations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.0.iter() {
            d.fmt(f)?;
        }
        Ok(())
    }
}
