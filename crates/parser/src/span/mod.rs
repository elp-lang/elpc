use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub lines: Vec<usize>,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}-{}", self.lines, self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_display() {
        let span = Span::default();

        assert_eq!("[]:0-0", format!("{}", span));
    }
}
