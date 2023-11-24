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
    Ident(String),
    Whitespace(String),
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

    fn consume_chars_into_token(&mut self, value: &String) -> Token {
        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position),
            token_type: match value {
                s if s == "true" => TokenType::LiteralBoolean(true),
                s if s == "false" => TokenType::LiteralBoolean(false),
                s if s == "import" => TokenType::Keyword(value.clone()),
                s if s == "{" => TokenType::OpenBlock,
                s if s == "}" => TokenType::CloseBlock,
                _ => TokenType::Ident(value.clone()),
            },
        }
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        let mut value: String = "".to_string();
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
        } else {
            for ch in self.input.chars().skip(self.current_position) {
                // Skip white space.
                self.reader_position += 1;

                if ch.is_whitespace() {
                    next_token = Ok(Token{
                    value: ch,
                        token_type: TokenType::Whitespace(ch),
                span: (self.current_position, self.reader_position),
                    })
                }

                    if value != "" {
                        next_token = Ok(self.consume_chars_into_token(&value));

                        value.clear();
                    }
                    break;
                } else {
                    value.push(ch);
                    self.reader_position += 1;
                }
            }
        }

        self.current_position = self.reader_position;

        next_token
    }

    pub fn consume_all_tokens(&mut self) -> Result<&Vec<Token>, Error> {
        loop {
            let next_token = self.consume_next_token();

            if let Err(err) = next_token {
                return Err(err);
            }

            if let Ok(token) = next_token {
                self.tokens.push(token.clone());
                if token.token_type.clone() == TokenType::EOF {
                    break;
                }
            } else {
                self.tokens.push(next_token.unwrap());
                break;
            }
        }

        Ok(&self.tokens)
    }
}
