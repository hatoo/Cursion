use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Style {
    pub fg: crate::color::TermColor,
    pub bg: crate::color::TermColor,
    pub decorations: crate::decoration::Decorations,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DiffStyle<'a, 'b> {
    pub from: &'a Style,
    pub to: &'b Style,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        termion::color::Fg(self.fg).fmt(f)?;
        termion::color::Bg(self.bg).fmt(f)?;
        self.decorations.fmt(f)?;

        Ok(())
    }
}

impl<'a, 'b> fmt::Display for DiffStyle<'a, 'b> {
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
