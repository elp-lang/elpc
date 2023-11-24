#[derive(Debug)]
enum Error {
    LexicalError(String),
    // Add more error variants as needed
}

#[derive(Debug)]
enum TokenType {
    SOI,
    EOF,
    UNKNOWN,
    LITERAL_BOOLEAN(bool),
}

#[derive(Debug)]
struct Token {
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

    fn consume_chars_into_token(&mut self, value: String) -> Token {
        return match value {
            s if s == "true" => Token {
                value: value.clone(),
                span: (self.current_position, self.reader_position),
                token_type: TokenType::LITERAL_BOOLEAN(true),
            },
            _ => Token {
                value: value.clone(),
                span: (self.current_position, self.reader_position),
                token_type: TokenType::LITERAL_BOOLEAN(true),
            },
        };
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        let mut value: String = "".to_string();

        if &self.reader_position + 1 > self.input.len() {
            return Ok(Token {
                span: (self.input.len(), self.input.len()),
                token_type: TokenType::EOF,
                value: "".to_string(),
            });
        } else {
            for ch in self.input.chars() {
                if ch.is_whitespace() {
                    if value != "" {
                        self.tokens.push(self.consume_chars_into_token(value));
                        value = "".to_string();
                    }
                    break;
                }

                value.push(ch);
                self.reader_position += 1;

                print!("{}", ch.to_string());
            }
        }

        if self.current_position == self.reader_position {
            return Err(Error::LexicalError(
                "Made no progress when expecting a new token.".to_string(),
            ));
        }

        self.current_position = self.reader_position;
        return Ok(tok);
    }
}
