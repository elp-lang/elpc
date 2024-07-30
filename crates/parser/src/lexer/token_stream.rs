use crate::tokens::Token;

pub struct TokenStream {
    tokens: Vec<Token>,
    cursor: usize,
}

impl<'a> TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, cursor: 0 }
    }

    pub fn is_empty(&'a self) -> bool {
        self.tokens.len() == 0
    }

    pub fn len(&'a self) -> usize {
        self.tokens.len()
    }

    pub fn consume(&'a mut self) -> &mut Self {
        self.cursor += 1;
        self
    }

    pub fn prev(&'a self) -> Option<&Token> {
        if self.is_empty() || self.cursor == 0 {
            return None;
        }

        self.tokens.get(self.cursor - 1)
    }

    pub fn next(&'a self) -> Option<&'a Token> {
        if self.is_empty() || self.cursor + 1 > self.len() {
            return None;
        }

        self.tokens.get(self.cursor)
    }
}
