#[derive(Debug)]
pub enum Error {
    UnknownToken(String), // Add more error variants as needed
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum AccessModifier {
    Const,
    #[default]
    Pub,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    Fn,
    Var,
    Import,
    From,
    Interface,
    Object,
    Enum,
    Match,
    If,
    ElseIf,
    Else,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Whitespace {
    Return,
    NewLine,
    Tab,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    DoubleSpeechMark,
    SingleSpeechMark,
    OpenParen,
    CloseParen,
    Colon,
    OpenBlock,
    CloseBlock,
    Period,
    Comma,
    Other(String),
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    SOI,
    EOF,
    LiteralBoolean(bool),
    Keyword(Keyword),
    ReturnType,
    Ident(String),
    Symbol(Symbol),
    Whitespace(Whitespace),
    AccessModifier(AccessModifier),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: (usize, usize),
    pub value: String,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            position: 0,
            input,
            tokens: vec![Token {
                token_type: TokenType::SOI,
                value: "".to_string(),
                span: (0, 0),
            }],
        }
    }

    fn next(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn consume(&mut self) {
        self.position += 1;
    }

    fn could_be_ident(&self, ch: Option<char>) -> Option<char> {
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

    fn is_whitespace(&self, ch: Option<char>) -> Option<char> {
        match ch {
            None => None,
            Some(ch) => {
                if ch.is_whitespace() {
                    Some(ch)
                } else {
                    None
                }
            }
        }
    }

    fn is_symbol(&self, ch: Option<char>) -> Option<char> {
        match ch {
            None => None,
            Some(ch) => {
                if !ch.is_whitespace() && self.could_be_ident(Some(ch)).is_none() {
                    Some(ch)
                } else {
                    None
                }
            }
        }
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let starting_cursor = self.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.could_be_ident(self.next()) {
            value.push(ch);
            self.consume();
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.position - 1),
            token_type: match value.clone() {
                s if s == "true" => TokenType::LiteralBoolean(true),
                s if s == "false" => TokenType::LiteralBoolean(false),
                s if s == "pub" => TokenType::AccessModifier(AccessModifier::Pub),
                s if s == "const" => TokenType::AccessModifier(AccessModifier::Const),
                s if s == "fn" => TokenType::Keyword(Keyword::Fn),
                s if s == "var" => TokenType::Keyword(Keyword::Var),
                s if s == "import" => TokenType::Keyword(Keyword::Import),
                s if s == "from" => TokenType::Keyword(Keyword::From),
                s if s == "interface" => TokenType::Keyword(Keyword::Interface),
                s if s == "object" => TokenType::Keyword(Keyword::Object),
                s if s == "enum" => TokenType::Keyword(Keyword::Enum),
                s if s == "match" => TokenType::Keyword(Keyword::Match),
                s if s == "if" => TokenType::Keyword(Keyword::If),
                s if s == "elseif" => TokenType::Keyword(Keyword::ElseIf),
                s if s == "else" => TokenType::Keyword(Keyword::Else),
                _ => TokenType::Ident(value.clone()),
            },
        }
    }

    fn consume_whitespace_into_token(&mut self) -> Token {
        let starting_cursor = self.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.is_whitespace(self.next()) {
            value.push(ch);
            self.consume();
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.position - 1),
            token_type: TokenType::Whitespace(match value.clone() {
                s if s == "\r" => Whitespace::Return,
                s if s == "\n" => Whitespace::NewLine,
                s if s == "\t" => Whitespace::Tab,
                _ => Whitespace::Other(value.clone().to_string()),
            }),
        }
    }

    fn consume_symbol_into_token(&mut self) -> Token {
        let starting_cursor = self.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.is_symbol(self.next()) {
            value.push(ch);
            self.consume();
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.position - 1),
            token_type: match value.clone() {
                s if s == ":" => TokenType::Symbol(Symbol::Colon),
                s if s == "{" => TokenType::Symbol(Symbol::OpenBlock),
                s if s == "}" => TokenType::Symbol(Symbol::CloseBlock),
                s if s == "." => TokenType::Symbol(Symbol::Period),
                s if s == "->" => TokenType::ReturnType,
                s if s == "\"" => TokenType::Symbol(Symbol::DoubleSpeechMark),
                s if s == "'" => TokenType::Symbol(Symbol::SingleSpeechMark),
                _ => TokenType::Symbol(Symbol::Other(value)),
            },
        }
    }

    fn consume_next_token(&mut self) -> Result<Token, Error> {
        if self.next().is_none() {
            return Ok(Token {
                token_type: TokenType::EOF,
                span: (self.position, self.position),
                value: "".to_string(),
            });
        }

        // Peek at the next char to get what function to call then we can advance the cursor.
        let ch = self.next();
        if self.could_be_ident(ch).is_some() {
            Ok(self.consume_ident_into_token())
        } else if self.is_whitespace(ch).is_some() {
            Ok(self.consume_whitespace_into_token())
        } else if self.is_symbol(ch).is_some() {
            Ok(self.consume_symbol_into_token())
        } else {
            Err(Error::UnknownToken("Unknown token.".to_string()))
        }
    }

    pub fn consume_all_tokens(&mut self) -> Vec<Token> {
        while let Ok(next_token) = self.consume_next_token() {
            self.tokens.push(next_token.to_owned());

            if next_token.token_type == TokenType::EOF {
                break;
            }
        }

        self.tokens.to_owned()
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
                span: ($str.len(), $str.len()),
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

    #[test]
    fn test_lexer_sanity() {
        let input = "import { Thing } from \"elp\"".to_string();
        let mut lexer = Lexer::new(input.clone());
        let space = Whitespace::Other(" ".to_string());

        assert_eq!(
            results!(
                input.clone(),
                Token {
                    token_type: TokenType::Keyword(Keyword::Import),
                    span: (0, 5),
                    value: "import".to_string(),
                },
                Token {
                    token_type: TokenType::Whitespace(space.clone()),
                    span: (6, 6),
                    value: " ".to_string(),
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    span: (7, 7),
                    value: "{".to_string(),
                },
                Token {
                    token_type: TokenType::Whitespace(space.clone()),
                    span: (8, 8),
                    value: " ".to_string(),
                },
                Token {
                    token_type: TokenType::Ident("Thing".to_string()),
                    span: (9, 13),
                    value: "Thing".to_string(),
                },
                Token {
                    token_type: TokenType::Whitespace(space.clone()),
                    span: (14, 14),
                    value: " ".to_string(),
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    span: (15, 15),
                    value: "}".to_string(),
                },
                Token {
                    token_type: TokenType::Whitespace(space.clone()),
                    span: (16, 16),
                    value: " ".to_string(),
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::From),
                    span: (17, 20),
                    value: "from".to_string(),
                },
                Token {
                    token_type: TokenType::Whitespace(space.clone()),
                    span: (21, 21),
                    value: " ".to_string(),
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                    span: (22, 22),
                    value: "\"".to_string(),
                },
                Token {
                    token_type: TokenType::Ident("elp".to_string()),
                    span: (23, 25),
                    value: "elp".to_string(),
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                    span: (26, 26),
                    value: "\"".to_string(),
                },
            ),
            *lexer.consume_all_tokens(),
        );
    }
}
