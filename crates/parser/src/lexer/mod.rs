pub mod token_stream;

use std::char;

use crate::{
    parsing_error::ParsingError,
    span::Span,
    tokens::{AccessModifier, Keyword, Source, Symbol, Token, TokenType, WhiteSpace},
};

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
                source: Source::default(),
            }],
        }
    }

    pub fn new_str(input: &str) -> Self {
        Lexer::new(input.to_string())
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

    fn consume_ident_to_string(&mut self) -> String {
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

        value
    }

    fn consume_ident_into_token(&mut self) -> Token {
        let starting_cursor = self.position;
        let value = self.consume_ident_to_string();

        Token {
            source: Source {
                span: Span {
                    start: starting_cursor,
                    end: self.position - 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            token_type: match value.clone() {
                s if s == "as" => TokenType::Keyword(Keyword::As),
                s if s == "component" => TokenType::Keyword(Keyword::Component),
                s if s == "const" => TokenType::Keyword(Keyword::Const),
                s if s == "else" => TokenType::Keyword(Keyword::Else),
                s if s == "elseif" => TokenType::Keyword(Keyword::ElseIf),
                s if s == "enum" => TokenType::Keyword(Keyword::Enum),
                s if s == "export" => TokenType::Keyword(Keyword::Export),
                s if s == "false" => TokenType::BooleanLiteral(false),
                s if s == "fn" => TokenType::Keyword(Keyword::Fn),
                s if s == "from" => TokenType::Keyword(Keyword::From),
                s if s == "if" => TokenType::Keyword(Keyword::If),
                s if s == "import" => TokenType::Keyword(Keyword::Import),
                s if s == "interface" => TokenType::Keyword(Keyword::Interface),
                s if s == "match" => TokenType::Keyword(Keyword::Match),
                s if s == "nil" => TokenType::Nil,
                s if s == "object" => TokenType::Keyword(Keyword::Object),
                s if s == "private" => TokenType::AccessModifier(AccessModifier::Private),
                s if s == "public" => TokenType::AccessModifier(AccessModifier::Public),
                s if s == "true" => TokenType::BooleanLiteral(true),
                s if s == "type" => TokenType::Keyword(Keyword::Type),
                s if s == "var" => TokenType::Keyword(Keyword::Var),
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
            source: Source {
                span: Span {
                    start: starting_cursor,
                    end: self.position - 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            token_type: TokenType::WhiteSpace(match value.clone() {
                s if s == " " => WhiteSpace::Space,
                s if s == "\n" => WhiteSpace::NewLine,
                s if s == "\t" => WhiteSpace::Tab,
                _ => WhiteSpace::Other(value.clone().to_string()),
            }),
        }
    }

    fn consume_comment_into_token(&mut self) -> Result<Token, Box<ParsingError>> {
        let mut comment_body = String::new();
        let cursor_start = self.position;
        let is_block = match self.next() {
            Some(ch) => ch == '*',
            None => false,
        };
        self.consume();

        loop {
            let next_ch = self.next();
            self.consume();

            match next_ch {
                Some(ch) => match ch {
                    '\n' => {
                        if is_block {
                            comment_body.push(ch);
                        } else {
                            break;
                        }
                    }
                    '*' if is_block => {
                        if let Some(ch) = self.next() {
                            if ch == '/' {
                                self.consume();
                                break;
                            }
                        }
                    }
                    _ => {
                        comment_body.push(ch);
                    }
                },
                None => break,
            }
        }

        Ok(Token {
            token_type: match is_block {
                false => TokenType::CommentLine(comment_body),
                true => TokenType::CommentBlock(comment_body),
            },
            source: Source {
                span: Span {
                    start: cursor_start,
                    end: self.position,
                    ..Default::default()
                },
                ..Default::default()
            },
        })
    }

    fn consume_symbol_into_token(&mut self) -> Result<Token, Box<ParsingError>> {
        let starting_cursor = self.position;
        let ch = self.is_symbol(self.next()).unwrap();

        self.consume();

        let mut token = Token {
            source: Source {
                span: Span {
                    start: starting_cursor,
                    end: self.position - 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            token_type: TokenType::Nil,
        };

        token.token_type = match ch {
            ':' => TokenType::Symbol(Symbol::Colon),
            '{' => TokenType::Symbol(Symbol::OpenBlock),
            '}' => TokenType::Symbol(Symbol::CloseBlock),
            '(' => TokenType::Symbol(Symbol::OpenParen),
            ')' => TokenType::Symbol(Symbol::CloseParen),
            '.' => TokenType::Symbol(Symbol::Dot),
            ',' => TokenType::Symbol(Symbol::Comma),
            '&' => match self.is_symbol(self.next()) {
                Some('&') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::DoubleAmpersand)
                }
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::BitwiseAndAssign)
                }
                _ => TokenType::Symbol(Symbol::Ampersand),
            },
            '%' => TokenType::Symbol(Symbol::Modulo),
            '|' => match self.is_symbol(self.next()) {
                Some('|') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::DoublePipe)
                }
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::BitwiseOrAssign)
                }
                _ => TokenType::Symbol(Symbol::Pipe),
            },
            '^' => match self.is_symbol(self.next()) {
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::BitwiseXorAssign)
                }
                _ => TokenType::Symbol(Symbol::Caret),
            },
            '"' | '\'' => {
                let is_single_quote = ch == '\'';
                let mut value = String::new();
                let mut escaped = false;
                let mut is_open = true;

                while let Some(ch) = self.next() {
                    self.consume();
                    if escaped {
                        match ch {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            '\\' => value.push('\\'),
                            '\'' if is_single_quote => value.push('\''),
                            '"' if !is_single_quote => value.push('"'),
                            _ => {
                                return Err(Box::new(ParsingError::InvalidEscapeSequence(
                                    ch,
                                    token.source,
                                )))
                            }
                        }
                        escaped = false;
                    } else if ch == '\\' {
                        escaped = true;
                    } else if ch == (if is_single_quote { '\'' } else { '"' }) {
                        is_open = false;
                        break;
                    } else {
                        value.push(ch);
                    }
                }

                if escaped || is_open {
                    return Err(Box::new(ParsingError::UnterminatedStringLiteral(
                        token.source,
                    )));
                }

                token.source.span.end = self.position;

                TokenType::StringLiteral(value)
            }
            '\\' => TokenType::Symbol(Symbol::BackSlash),
            '@' => match self.next() {
                Some(ch) => {
                    if ch.is_alphabetic() {
                        let ident = self.consume_ident_to_string();

                        token.source.span.end = token.source.span.start + ident.len();
                        TokenType::MacroCall(ident)
                    } else {
                        TokenType::Symbol(Symbol::Other(ch.into()))
                    }
                }
                None => TokenType::Symbol(Symbol::Other(ch.into())),
            },
            '/' => match self.is_symbol(self.next()) {
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::SlashAssign)
                }
                Some('/') | Some('*') => {
                    let comment = self.consume_comment_into_token()?;
                    token.source = comment.source;
                    comment.token_type
                }
                _ => TokenType::Symbol(Symbol::Slash),
            },
            '<' => match self.is_symbol(self.next()) {
                Some('<') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::BitwiseLeftShift)
                }
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::LessEqual)
                }
                _ => TokenType::Symbol(Symbol::Less),
            },
            '>' => match self.is_symbol(self.next()) {
                Some('>') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::BitwiseRightShift)
                }
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::GreaterEqual)
                }
                _ => TokenType::Symbol(Symbol::Greater),
            },
            '=' => match self.is_symbol(self.next()) {
                Some('=') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::DoubleEqual)
                }
                _ => TokenType::Symbol(Symbol::SingleEqual),
            },
            '-' => match self.is_symbol(self.next()) {
                Some('>') => {
                    self.consume();
                    token.source.span.end += 1;
                    TokenType::Symbol(Symbol::Arrow)
                }
                _ => TokenType::Symbol(Symbol::Hyphen),
            },
            _ => TokenType::Symbol(Symbol::Other(ch.into())),
        };

        Ok(token)
    }

    fn consume_numerics_into_token(&mut self) -> Result<Token, Box<ParsingError>> {
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
        let source = Source {
            span: Span {
                start: starting_cursor,
                end: self.position - 1,
                ..Default::default()
            },
            ..Default::default()
        };

        if probably_int {
            match value.parse::<i64>() {
                Ok(int) => {
                    token_type = TokenType::IntegerLiteral(int);
                }
                Err(err) => return Err(Box::new(ParsingError::InvalidInt(err, source))),
            }

            return Ok(Token { token_type, source });
        }

        match value.parse::<f64>() {
            Ok(f) => token_type = TokenType::FloatLiteral(f),
            Err(err) => return Err(Box::new(ParsingError::InvalidFloat(err, source))),
        }

        Ok(Token { token_type, source })
    }

    fn consume_next_token(&mut self) -> Result<Token, Box<ParsingError>> {
        let source = Source {
            span: Span {
                start: self.position,
                end: self.position,
                ..Default::default()
            },
            ..Default::default()
        };

        if self.next().is_none() {
            return Ok(Token {
                token_type: TokenType::EOF,
                source,
            });
        }

        // Peek at the next char to get what function to call then we can advance the cursor.
        let ch = self.next();
        if self.could_be_ident(ch).is_some() {
            Ok(self.consume_ident_into_token())
        } else if self.is_whitespace(ch).is_some() {
            Ok(self.consume_whitespace_into_token())
        } else if self.is_symbol(ch).is_some() {
            self.consume_symbol_into_token()
        } else if ch.unwrap().is_numeric() {
            self.consume_numerics_into_token()
        } else {
            Err(Box::new(ParsingError::UnknownChar(ch.unwrap(), source)))
        }
    }

    pub fn consume_all_tokens(&mut self) -> Result<Vec<Token>, Box<ParsingError>> {
        loop {
            let next_token_result = self.consume_next_token();

            match next_token_result {
                Ok(next_token) => {
                    self.tokens.push(next_token.to_owned());

                    if next_token.token_type == TokenType::EOF {
                        break;
                    }
                }
                Err(err) => return Err(err),
            }
        }

        Ok(self.tokens.to_owned())
    }
}
