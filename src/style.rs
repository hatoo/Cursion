#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Style {
    fg: crate::color::TermColor,
    bg: crate::color::TermColor,
    decorations: crate::decoration::Decorations,
}
