use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub lines: Vec<usize>,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}-{}", self.lines, self.start + 1, self.end + 1)
    }
}
