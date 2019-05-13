#[derive(Debug, Eq, PartialEq)]
pub struct Style {
    fg: crate::color::TermColor,
    bg: crate::color::TermColor,
    decorations: crate::decoration::Decorations,
}
