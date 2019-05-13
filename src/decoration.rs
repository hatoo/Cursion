use std::collections::btree_set::BTreeSet;
use std::fmt;
use termion::style;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum Decoration {
    Blink,
    Bold,
    Faint,
    Framed,
    Invert,
    Italic,
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

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct Decorations(BTreeSet<Decoration>);

impl Decorations {
    pub fn with(mut self, decoration: Decoration) -> Self {
        self.0.insert(decoration);
        self
    }
}

impl fmt::Display for Decorations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in &self.0 {
            d.fmt(f)?;
        }
        Ok(())
    }
}
