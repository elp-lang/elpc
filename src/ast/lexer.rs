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
    DoubleSpeechMark,
    SingleSpeechMark,
    OpenBlock,
    CloseBlock,
    ReturnType,
    Ident(String),
    Symbol(String),
    Whitespace(String),
    AccessModifier(String),
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
            TokenType::DoubleSpeechMark => write!(f, "\""),
            TokenType::SingleSpeechMark => write!(f, "'"),
            TokenType::AccessModifier(value) => write!(f, "Access modifier: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cursor {
    input: String,
    pub position: usize,
    pub prev_char: Option<char>,
    pub current_char: Option<char>,
    pub next_char: Option<char>,
}

impl Cursor {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            prev_char: None,
            current_char: None,
            next_char: None,
        }
    }

    pub fn next(&mut self) -> Self {
        self.prev_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.input.chars().nth(self.position);
        self.to_owned()
    }

    pub fn prev(&mut self) -> Self {
        self.next_char = self.current_char;
        self.current_char = self.prev_char;
        self.prev_char = self.input.chars().nth(self.position - 1);
        self.to_owned()
    }

    pub fn consume(&mut self) {
        self.position += 1;
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
    cursor: Cursor,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            cursor: Cursor::new(input),
            tokens: vec![Token {
                token_type: TokenType::SOI,
                value: "".to_string(),
                span: (0, 0),
            }],
        }
    }

    fn could_be_ident(&mut self, ch: Option<char>) -> Option<char> {
        match ch {
            None => None,
            Some(ch) => {
                if ch.is_ascii_alphabetic() || ch.is_numeric() || ch == '_' {
                    Some(ch)
                } else {
                    None
                }
            }
        }
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let starting_cursor = self.cursor.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.could_be_ident(self.cursor.next().current_char) {
            value.push(ch);
            self.cursor.consume();
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.cursor.position),
            token_type: match value.clone() {
                s if s == "true" => TokenType::LiteralBoolean(true),
                s if s == "false" => TokenType::LiteralBoolean(false),
                _ => TokenType::Ident(value.clone()),
            },
        }
    }

    fn consume_whitespace_into_token(&mut self) -> Token {
        let starting_cursor = self.cursor.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.cursor.next().current_char {
            if ch.is_whitespace() {
                value.push(ch);
                self.cursor.consume();
            } else {
                break;
            }
        }

        return Token {
            value: value.clone(),
            span: (starting_cursor, self.cursor.position),
            token_type: TokenType::Whitespace(value.clone()),
        };
    }

    fn consume_symbol_into_token(&mut self) -> Token {
        let starting_cursor = self.cursor.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.cursor.next().current_char {
            match !ch.is_whitespace() && self.could_be_ident(Some(ch)).is_some() {
                true => {
                    value.push(ch);
                }
                false => break,
            }
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.cursor.position),
            token_type: match value.clone() {
                s if s == "{" => TokenType::OpenBlock,
                s if s == "}" => TokenType::CloseBlock,
                s if s == "->" => TokenType::ReturnType,
                s if s == "\"" => TokenType::DoubleSpeechMark,
                s if s == "'" => TokenType::SingleSpeechMark,
                _ => TokenType::Symbol(value),
            },
        }
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        let next_token: Result<Token, Error>;

        if self.cursor.next_char.is_none() {
            return Ok(Token {
                token_type: TokenType::EOF,
                span: (self.cursor.position, self.cursor.position),
                value: "".to_string(),
            });
        }

        // Peek at the next char to get what function to call then we can advance the cursor.
        if let Some(ch) = self.cursor.next_char {
            next_token = if self.could_be_ident(ch).is_some() {
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
            if self.cursor == self.cursor && next_token.token_type != TokenType::EOF {
                print!("Parsed token but cursor didn't move {:#?}\n", next_token);
            }

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
                    span: (7, 7),
                    value: "{".to_string(),
                },
                whitespace!(8, " "),
                Token {
                    token_type: TokenType::Ident("Thing".to_string()),
                    span: (9, 13),
                    value: "Thing".to_string(),
                },
                whitespace!(14, " "),
                Token {
                    token_type: TokenType::CloseBlock,
                    span: (15, 15),
                    value: "}".to_string(),
                },
                whitespace!(16, " "),
                Token {
                    token_type: TokenType::Ident("from".to_string()),
                    span: (17, 20),
                    value: "from".to_string(),
                },
                whitespace!(21, " "),
                Token {
                    token_type: TokenType::DoubleSpeechMark,
                    span: (22, 22),
                    value: "\"".to_string(),
                },
                Token {
                    token_type: TokenType::Ident("elp".to_string()),
                    span: (23, 25),
                    value: "elp".to_string(),
                },
                Token {
                    token_type: TokenType::DoubleSpeechMark,
                    span: (26, 26),
                    value: "\"".to_string(),
                },
            ),
            *lexer.consume_all_tokens(),
        );
    }
}
