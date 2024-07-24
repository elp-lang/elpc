use crate::tokens::Token;

pub struct TokenStream {
    tokens: Vec<Token>,
    cursor: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, cursor: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.len() == 0
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume(&mut self) -> &mut Self {
        self.cursor += 1;
        self
    }

    pub fn next(&self) -> Option<&Token> {
        if self.is_empty() || self.cursor + 1 > self.len() {
            return None;
        }

        self.tokens.get(self.cursor)
    }
}
