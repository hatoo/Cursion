use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CursorShape {
    Default,
    BlinkingBlock,
    SteadyBlock,
    BlinkingUnderline,
    SteadyUnderline,
    BlinkingBar,
    SteadyBar,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    pub shape: CursorShape,
}

impl fmt::Display for CursorShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CursorShape::Default => "\x1b[\x30 q".fmt(f),
            CursorShape::BlinkingBlock => "\x1b[\x31 q".fmt(f),
            CursorShape::SteadyBlock => "\x1b[\x32 q".fmt(f),
            CursorShape::BlinkingUnderline => "\x1b[\x33 q".fmt(f),
            CursorShape::SteadyUnderline => "\x1b[\x34 q".fmt(f),
            CursorShape::BlinkingBar => "\x1b[\x35 q".fmt(f),
            CursorShape::SteadyBar => "\x1b[\x36 q".fmt(f),
        }
    }
}

impl Default for CursorShape {
    fn default() -> Self {
        CursorShape::Default
    }
}
