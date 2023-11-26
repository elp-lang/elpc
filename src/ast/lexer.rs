#[derive(Debug)]
pub enum Error {
    UnknownToken(String), // Add more error variants as needed
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    SOI,
    EOF,
    LiteralBoolean(bool),
    Keyword(String),
    OpenBlock,
    CloseBlock,
    ReturnType,
    Ident(String),
    Symbol(String),
    Whitespace(String),
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::SOI => write!(f, "SOI"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::LiteralBoolean(value) => write!(f, "Boolean: {}", value),
            TokenType::Keyword(value) => write!(f, "Keyword: {}", value),
            TokenType::OpenBlock => write!(f, "Open block"),
            TokenType::CloseBlock => write!(f, "Close block"),
            TokenType::ReturnType => write!(f, "Return type '->'"),
            TokenType::Ident(value) => write!(f, "Ident: {}", value),
            TokenType::Whitespace(value) => write!(f, "Whitespace: '{:#?}'", value),
            TokenType::Symbol(value) => write!(f, "Symbol: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: (usize, usize),
    pub value: String,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    current_position: usize,
    reader_position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            current_position: 0,
            reader_position: 0,
            tokens: vec![],
        }
    }

    fn could_be_ident(&mut self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_'
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        loop {
            if let Some(ch) = self.input.chars().nth(self.reader_position) {
                if self.could_be_ident(ch) {
                    value.push(ch);
                } else {
                    break;
                }
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position),
            token_type: match value.clone() {
                s if s == "true" => TokenType::LiteralBoolean(true),
                s if s == "false" => TokenType::LiteralBoolean(false),
                _ => TokenType::Ident(value.clone()),
            },
        }
    }

    fn consume_whitespace_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        loop {
            if let Some(ch) = self.input.chars().nth(self.reader_position) {
                if ch.is_whitespace() {
                    value.push(ch);
                } else {
                    break;
                }
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position),
            token_type: TokenType::Whitespace(value.clone()),
        }
    }

    fn consume_symbol_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        loop {
            if let Some(ch) = self.input.chars().nth(self.reader_position) {
                if !ch.is_whitespace() && !self.could_be_ident(ch) {
                    value.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position),
            token_type: match value.clone() {
                s if s == "{" => TokenType::OpenBlock,
                s if s == "}" => TokenType::CloseBlock,
                s if s == "->" => TokenType::ReturnType,
                _ => TokenType::Symbol(value),
            },
        }
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        let mut next_token: Result<Token, Error> = Ok(Token {
            token_type: TokenType::SOI,
            value: "".to_string(),
            span: (0, 0),
        });

        if &self.reader_position + 1 > self.input.len() {
            return Ok(Token {
                token_type: TokenType::EOF,
                span: (self.current_position, self.reader_position),
                value: "".to_string(),
            });
        }

        // Peek at the next char to get what function to call then we can advance the cursor.
        if let Some(ch) = self.input.chars().nth(self.reader_position) {
            next_token = if self.could_be_ident(ch) {
                Ok(self.consume_ident_into_token())
            } else if ch.is_whitespace() {
                Ok(self.consume_whitespace_into_token())
            } else if !ch.is_whitespace() && !self.could_be_ident(ch) {
                Ok(self.consume_symbol_into_token())
            } else {
                Err(Error::UnknownToken("Unknown token.".to_string()))
            }
        } else {
            next_token = Err(Error::UnknownToken("Unknown token.".to_string()));
        }

        next_token
    }

    pub fn consume_all_tokens(&mut self) -> Result<&Vec<Token>, Error> {
        loop {
            let next_token = self.consume_next_token();
            print!("\n{}, {}\n", self.current_position, self.reader_position);
            if self.current_position == self.reader_position {
                panic!("Cursor was not advanced!")
            }
            self.current_position = self.reader_position;

            if let Err(err) = next_token {
                return Err(err);
            }

            if let Ok(token) = next_token {
                self.tokens.push(token.clone());
                print!("\n'{}'\n{}\n", token.value, token.token_type);
                if token.token_type == TokenType::EOF {
                    break;
                }
            } else {
                self.tokens.push(next_token.unwrap());
            }
        }

        Ok(&self.tokens)
    }
}
