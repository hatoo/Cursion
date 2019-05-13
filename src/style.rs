use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Style {
    fg: crate::color::TermColor,
    bg: crate::color::TermColor,
    decorations: crate::decoration::Decorations,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        termion::color::Fg(self.fg).fmt(f)?;
        termion::color::Bg(self.bg).fmt(f)?;
        self.decorations.fmt(f)?;

        Ok(())
    }
}
