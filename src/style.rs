use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Style {
    pub fg: crate::color::TermColor,
    pub bg: crate::color::TermColor,
    pub decorations: crate::decoration::Decorations,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DiffStyle {
    pub from: Style,
    pub to: Style,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        termion::color::Fg(self.fg).fmt(f)?;
        termion::color::Bg(self.bg).fmt(f)?;
        self.decorations.fmt(f)?;

        Ok(())
    }
}

impl fmt::Display for DiffStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.from.fg != self.to.fg {
            termion::color::Fg(self.to.fg).fmt(f)?;
        }

        if self.from.bg != self.to.bg {
            &termion::color::Bg(self.to.bg).fmt(f)?;
        }

        if self.from.decorations != self.to.decorations {
            termion::style::Reset.fmt(f)?;
            self.to.decorations.fmt(f)?;
        }

        Ok(())
    }
}
