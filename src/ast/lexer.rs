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
            tokens: vec![Token {
                token_type: TokenType::SOI,
                value: "".to_string(),
                span: (0, 0),
            }],
        }
    }

    fn could_be_ident(&mut self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_'
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        while let Some(ch) = self.input.chars().nth(self.reader_position) {
            if self.could_be_ident(ch) {
                value.push(ch);
            } else {
                break;
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position - 1),
            token_type: match value.clone() {
                s if s == "true" => TokenType::LiteralBoolean(true),
                s if s == "false" => TokenType::LiteralBoolean(false),
                _ => TokenType::Ident(value.clone()),
            },
        }
    }

    fn consume_whitespace_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        while let Some(ch) = self.input.chars().nth(self.reader_position) {
            if ch.is_whitespace() {
                value.push(ch);
            } else {
                break;
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position - 1),
            token_type: TokenType::Whitespace(value.clone()),
        }
    }

    fn consume_symbol_into_token(&mut self) -> Token {
        let mut value: String = "".to_string();

        while let Some(ch) = self.input.chars().nth(self.reader_position) {
            if !ch.is_whitespace() && !self.could_be_ident(ch) {
                value.push(ch);
            } else {
                break;
            }

            self.reader_position += 1;
        }

        Token {
            value: value.clone(),
            span: (self.current_position, self.reader_position - 1),
            token_type: match value.clone() {
                s if s == "{" => TokenType::OpenBlock,
                s if s == "}" => TokenType::CloseBlock,
                s if s == "->" => TokenType::ReturnType,
                _ => TokenType::Symbol(value),
            },
        }
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        let next_token: Result<Token, Error>;

        if self.reader_position + 1 >= self.input.len() - 1 {
            self.current_position = self.input.len() - 1;
            return Ok(Token {
                token_type: TokenType::EOF,
                span: (self.reader_position, self.reader_position),
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

    pub fn consume_all_tokens(&mut self) -> &Vec<Token> {
        while let Ok(next_token) = self.consume_next_token() {
            if self.current_position == self.reader_position
                && next_token.token_type != TokenType::EOF
            {
                print!("Parsed token {:#?}\n", next_token);
                panic!("Cursor was not advanced!")
            }
            self.current_position = self.reader_position;

            self.tokens.push(next_token.clone());
            if next_token.token_type == TokenType::EOF {
                break;
            }
        }

        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! results {
    ($str:expr, $($middle:expr),*,) => {
        {
            let soi = Token {
                token_type: TokenType::SOI,
                span: (0, 0),
                value: "".to_string(),
            };
            let eof = Token {
                token_type: TokenType::EOF,
                span: ($str.len()-1, $str.len()-1),
                value: "".to_string(),
            };
            let mut v = Vec::new();
            v.push(soi);
            $(v.push($middle);)*
            v.push(eof);
            v
        }
    };
}

    macro_rules! whitespace {
        ($x:expr, $value:expr) => {
            Token {
                token_type: TokenType::Whitespace($value.to_string()),
                span: ($x, $x),
                value: $value.to_string(),
            }
        };
    }

    #[test]
    fn test_lexer_sanity() {
        let input = "import { Thing } from \"elp\"".to_string();
        for (index, character) in input.chars().enumerate() {
            println!("{}: {}", index, character);
        }
        let mut lexer = Lexer::new(input.clone());

        assert_eq!(
            *lexer.consume_all_tokens(),
            results!(
                input.clone(),
                Token {
                    token_type: TokenType::Ident("import".to_string()),
                    span: (0, 5),
                    value: "import".to_string(),
                },
                whitespace!(6, " "),
                Token {
                    token_type: TokenType::OpenBlock,
                    span: (8, 9),
                    value: "{".to_string(),
                },
                whitespace!(9, " "),
                Token {
                    token_type: TokenType::Ident("Thing".to_string()),
                    span: (11, 16),
                    value: "Thing".to_string(),
                },
                whitespace!(17, " "),
                Token {
                    token_type: TokenType::CloseBlock,
                    span: (19, 20),
                    value: "}".to_string(),
                },
                whitespace!(21, " "),
                Token {
                    token_type: TokenType::Ident("from".to_string()),
                    span: (22, 26),
                    value: "from".to_string(),
                },
                whitespace!(26, " "),
            )
        );
    }
}
