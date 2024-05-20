use std::char;
use std::fmt::Display;

use super::parsing_error::ParsingError;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum AccessModifier {
    #[default]
    Private,
    Pub,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    As,
    Fn,
    Var,
    Const,
    Import,
    From,
    Interface,
    Object,
    Enum,
    Match,
    If,
    ElseIf,
    Else,
    Nil,
    Empty,
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
    BackSlash,
    SingleEqual,
    DoubleEqual,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Void,
    Unknown,
    SOI,
    EOF,
    BooleanLiteral(bool),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Keyword(Keyword),
    ReturnType,
    Ident(String),
    Symbol(Symbol),
    Whitespace(Whitespace),
    AccessModifier(AccessModifier),
}

impl Display for TokenType {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::Void => "void".into(),
            TokenType::SOI => "SOI (Start Of Input)".into(),
            TokenType::EOF => "EOF (End Of File)".into(),
            TokenType::Keyword(Keyword::As) => "as".into(),
            TokenType::Keyword(Keyword::Interface) => "interface".into(),
            TokenType::Keyword(Keyword::Enum) => "enum".into(),
            TokenType::Keyword(Keyword::Fn) => "fn".into(),
            TokenType::Keyword(Keyword::Var) => "var".into(),
            TokenType::Keyword(Keyword::Const) => "const".into(),
            TokenType::Keyword(Keyword::Import) => "import".into(),
            TokenType::Keyword(Keyword::From) => "from".into(),
            TokenType::Keyword(Keyword::Object) => "object".into(),
            TokenType::Keyword(Keyword::Match) => "match".into(),
            TokenType::Keyword(Keyword::If) => "if".into(),
            TokenType::Keyword(Keyword::ElseIf) => "elseif".into(),
            TokenType::Keyword(Keyword::Else) => "else".into(),
            TokenType::Keyword(Keyword::Nil) => "nil".into(),
            TokenType::Keyword(Keyword::Empty) => "empty".into(),
            TokenType::ReturnType => "return type".into(),
            TokenType::Ident(s) => format!("ident({})", s),
            TokenType::Symbol(Symbol::CloseParen) => ")".into(),
            TokenType::Symbol(Symbol::DoubleSpeechMark) => "\"".into(),
            TokenType::Symbol(Symbol::SingleSpeechMark) => "'".into(),
            TokenType::Symbol(Symbol::SingleEqual) => "=".into(),
            TokenType::Symbol(Symbol::DoubleEqual) => "==".into(),
            TokenType::Symbol(Symbol::OpenParen) => "(".into(),
            TokenType::Symbol(Symbol::Colon) => ":".into(),
            TokenType::Symbol(Symbol::OpenBlock) => "{".into(),
            TokenType::Symbol(Symbol::CloseBlock) => "}".into(),
            TokenType::Symbol(Symbol::Period) => ".".into(),
            TokenType::Symbol(Symbol::Comma) => ",".into(),
            TokenType::Symbol(Symbol::BackSlash) => "\\".into(),
            TokenType::Symbol(Symbol::BitwiseOr) => "|".into(),
            TokenType::Symbol(Symbol::BitwiseAnd) => "&".into(),
            TokenType::Symbol(Symbol::BitwiseXor) => "^".into(),
            TokenType::Symbol(Symbol::BitwiseLeftShift) => "<<".into(),
            TokenType::Symbol(Symbol::BitwiseRightShift) => ">>".into(),
            TokenType::Symbol(Symbol::Other(s)) => s.to_string(),
            TokenType::Whitespace(Whitespace::Tab) => "tab \\t".into(),
            TokenType::Whitespace(Whitespace::Return) => "return \\r".into(),
            TokenType::Whitespace(Whitespace::NewLine) => "new line \\n".into(),
            TokenType::Whitespace(Whitespace::Other(w)) => w.to_string(),
            TokenType::AccessModifier(AccessModifier::Pub) => "pub".into(),
            TokenType::BooleanLiteral(_) => "boolean".into(),
            TokenType::StringLiteral(_) => "string literal".into(),
            TokenType::IntegerLiteral(n) => format!("integer '{}'", n),
            TokenType::FloatLiteral(f) => format!("float '{:e}'", f),
            TokenType::AccessModifier(AccessModifier::Private) => "private".into(),
            TokenType::Unknown => "unknown".into(),
        };
        write!(f1, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]
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
        ch.filter(|&ch| ch.is_ascii_alphabetic() || ch == '_')
    }

    fn is_whitespace(&self, ch: Option<char>) -> Option<char> {
        ch.filter(|&ch| ch.is_whitespace())
    }

    fn is_symbol(&self, ch: Option<char>) -> Option<char> {
        ch.filter(|&ch| {
            !ch.is_whitespace() && self.could_be_ident(Some(ch)).is_none() && !ch.is_numeric()
        })
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let starting_cursor = self.position;
        let mut value: String = "".to_string();

        while let Some(ch) = self.next() {
            if value.is_empty() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    value.push(ch);
                    self.consume();
                } else {
                    break;
                }
            } else if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(ch);
                self.consume();
            } else {
                break;
            }
        }

        Token {
            value: value.clone(),
            span: (starting_cursor, self.position - 1),
            token_type: match value.clone() {
                s if s == "true" => TokenType::BooleanLiteral(true),
                s if s == "false" => TokenType::BooleanLiteral(false),
                s if s == "pub" => TokenType::AccessModifier(AccessModifier::Pub),
                s if s == "private" => TokenType::AccessModifier(AccessModifier::Private),
                s if s == "fn" => TokenType::Keyword(Keyword::Fn),
                s if s == "var" => TokenType::Keyword(Keyword::Var),
                s if s == "const" => TokenType::Keyword(Keyword::Const),
                s if s == "import" => TokenType::Keyword(Keyword::Import),
                s if s == "from" => TokenType::Keyword(Keyword::From),
                s if s == "interface" => TokenType::Keyword(Keyword::Interface),
                s if s == "object" => TokenType::Keyword(Keyword::Object),
                s if s == "enum" => TokenType::Keyword(Keyword::Enum),
                s if s == "match" => TokenType::Keyword(Keyword::Match),
                s if s == "if" => TokenType::Keyword(Keyword::If),
                s if s == "elseif" => TokenType::Keyword(Keyword::ElseIf),
                s if s == "else" => TokenType::Keyword(Keyword::Else),
                s if s == "void" => TokenType::Void,
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

            if ch == '\n' || ch == '\r' || ch == '\t' {
                break;
            }
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

        let ch = self.is_symbol(self.next()).unwrap();
        self.consume();
        let mut token = Token {
            value: ch.into(),
            span: (starting_cursor, self.position - 1),
            token_type: TokenType::Unknown,
        };

        token.token_type = match ch {
            ':' => TokenType::Symbol(Symbol::Colon),
            '{' => TokenType::Symbol(Symbol::OpenBlock),
            '}' => TokenType::Symbol(Symbol::CloseBlock),
            '(' => TokenType::Symbol(Symbol::OpenParen),
            ')' => TokenType::Symbol(Symbol::CloseParen),
            '.' => TokenType::Symbol(Symbol::Period),
            ',' => TokenType::Symbol(Symbol::Comma),
            '&' => TokenType::Symbol(Symbol::BitwiseAnd),
            '|' => TokenType::Symbol(Symbol::BitwiseOr),
            '^' => TokenType::Symbol(Symbol::BitwiseXor),
            '"' => TokenType::Symbol(Symbol::DoubleSpeechMark),
            '\'' => TokenType::Symbol(Symbol::SingleSpeechMark),
            '\\' => TokenType::Symbol(Symbol::BackSlash),
            '<' => match self.is_symbol(self.next()) {
                Some('<') => TokenType::Symbol(Symbol::BitwiseLeftShift),
                _ => TokenType::Symbol(Symbol::Other(ch.to_string())),
            },
            '>' => match self.is_symbol(self.next()) {
                Some('>') => TokenType::Symbol(Symbol::BitwiseRightShift),
                _ => TokenType::Symbol(Symbol::Other(ch.to_string())),
            },
            '=' => match self.is_symbol(self.next()) {
                Some('=') => {
                    self.consume();
                    token.value = "==".into();
                    token.span.1 += 1;
                    TokenType::Symbol(Symbol::DoubleEqual)
                }
                _ => TokenType::Symbol(Symbol::SingleEqual),
            },
            '-' => match self.is_symbol(self.next()) {
                Some('>') => {
                    self.consume();
                    token.value = "->".into();
                    token.span.1 += 1;
                    TokenType::ReturnType
                }
                _ => TokenType::Symbol(Symbol::Other(ch.into())),
            },
            _ => TokenType::Symbol(Symbol::Other(ch.into())),
        };

        token
    }

    fn consume_numerics_into_token(&mut self) -> Result<Token, ParsingError> {
        let mut value: String = "".into();
        let starting_cursor = self.position;
        let mut probably_int = true;

        while let Some(ch) = self.next() {
            if value.is_empty() {
                if !ch.is_numeric() {
                    break;
                }
            } else if !ch.is_numeric() && ch != '_' && ch != '.' && ch != 'e' {
                break;
            }

            if ch == '.' {
                probably_int = false;
            }

            value.push(ch);

            self.consume();
        }

        let token_type: TokenType;

        if probably_int {
            match value.parse::<i64>() {
                Ok(int) => {
                    token_type = TokenType::IntegerLiteral(int);
                }
                Err(err) => return Err(ParsingError::InvalidInt(err)),
            }

            return Ok(Token {
                token_type,
                value: value.to_string(),
                span: (starting_cursor, self.position - 1),
            });
        }

        match value.parse::<f64>() {
            Ok(f) => token_type = TokenType::FloatLiteral(f),
            Err(err) => return Err(ParsingError::InvalidFloat(err)),
        }

        Ok(Token {
            token_type,
            value: value.to_string(),
            span: (starting_cursor, self.position - 1),
        })
    }

    fn consume_next_token(&mut self) -> Result<Token, ParsingError> {
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
        } else if ch.unwrap().is_numeric() {
            self.consume_numerics_into_token()
        } else {
            Err(ParsingError::UnknownChar(ch.unwrap()))
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
    use crate::parser::testing::Test;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lexer_sanity() {
        let space = Whitespace::Other(" ".to_string());
        let tests: Vec<Test<&'static str, Vec<Token>>> = vec![
            Test {
                name: "basic int",
                input: "123",
                expected: vec![
                    Token {
                        token_type: TokenType::SOI,
                        span: (0, 0),
                        value: "".to_string(),
                    },
                    Token {
                        token_type: TokenType::IntegerLiteral(123),
                        value: "123".into(),
                        span: (0, 2),
                    },
                    Token {
                        token_type: TokenType::EOF,
                        span: (3, 3),
                        value: "".to_string(),
                    },
                ],
            },
            Test {
                name: "basic float",
                input: "0.3",
                expected: vec![
                    Token {
                        token_type: TokenType::SOI,
                        span: (0, 0),
                        value: "".to_string(),
                    },
                    Token {
                        token_type: TokenType::FloatLiteral(0.3),
                        value: "0.3".into(),
                        span: (0, 2),
                    },
                    Token {
                        token_type: TokenType::EOF,
                        span: (3, 3),
                        value: "".to_string(),
                    },
                ],
            },
            Test {
                name: "scientific float",
                input: "0.1e3",
                expected: vec![
                    Token {
                        token_type: TokenType::SOI,
                        span: (0, 0),
                        value: "".to_string(),
                    },
                    Token {
                        token_type: TokenType::FloatLiteral(0.1e3),
                        value: "0.1e3".into(),
                        span: (0, 4),
                    },
                    Token {
                        token_type: TokenType::EOF,
                        span: (5, 5),
                        value: "".to_string(),
                    },
                ],
            },
            Test {
                name: "basic import",
                input: "import { Thing } from \"elp\"",
                expected: vec![
                    Token {
                        token_type: TokenType::SOI,
                        span: (0, 0),
                        value: "".to_string(),
                    },
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
                    Token {
                        token_type: TokenType::EOF,
                        span: (27, 27),
                        value: "".to_string(),
                    },
                ],
            },
            Test {
                name: "escaped string",
                input: "\\\"escaped\\\"",
                expected: vec![
                    Token {
                        token_type: TokenType::SOI,
                        span: (0, 0),
                        value: "".to_string(),
                    },
                    Token {
                        token_type: TokenType::Symbol(Symbol::BackSlash),
                        span: (0, 0),
                        value: "\\".into(),
                    },
                    Token {
                        token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                        span: (1, 1),
                        value: "\"".into(),
                    },
                    Token {
                        token_type: TokenType::Ident("escaped".into()),
                        span: (2, 8),
                        value: "escaped".into(),
                    },
                    Token {
                        token_type: TokenType::Symbol(Symbol::BackSlash),
                        span: (9, 9),
                        value: "\\".into(),
                    },
                    Token {
                        token_type: TokenType::Symbol(Symbol::DoubleSpeechMark),
                        span: (10, 10),
                        value: "\"".into(),
                    },
                    Token {
                        token_type: TokenType::EOF,
                        span: (11, 11),
                        value: "".to_string(),
                    },
                ],
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new(test.input.to_string());

            assert_eq!(*lexer.consume_all_tokens(), test.expected);
        }
    }
}
