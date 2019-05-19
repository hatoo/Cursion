//! Style in terminal

use std::fmt;

/// A Style
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Style {
    /// Foreground color
    pub fg: crate::color::TermColor,
    /// Background color
    pub bg: crate::color::TermColor,
    /// Decorations
    pub decorations: crate::decoration::Decorations,
}

/// Previous and next Style
#[derive(Debug, Eq, PartialEq)]
pub struct DiffStyle {
    /// from
    pub from: Style,
    /// to
    pub to: Style,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.decorations.fmt(f)?;
        termion::color::Fg(self.fg).fmt(f)?;
        termion::color::Bg(self.bg).fmt(f)?;

        Ok(())
    }
}

impl fmt::Display for DiffStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.from.decorations != self.to.decorations {
            self.to.decorations.fmt(f)?;
        }

        if self.from.fg != self.to.fg {
            termion::color::Fg(self.to.fg).fmt(f)?;
        }

        if self.from.bg != self.to.bg {
            termion::color::Bg(self.to.bg).fmt(f)?;
        }

        Ok(())
    }
}
