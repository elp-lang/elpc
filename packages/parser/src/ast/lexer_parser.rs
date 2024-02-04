use crate::ast::lexer::{self, TokenType};

use super::{
    parsers::{
        self, enums::parse_enum_declaration, funcs::parse_fn, string_literals::parse_string_literal,
    },
    AstNode, Expression, Literal, Trie,
};

pub struct Parser {
    position: usize,
    tokens: Vec<lexer::Token>,
    pub current_token: Option<lexer::Token>,
}

impl Parser {
    /// Create a new instance of Parser, takes a Vec<lexer::Token> from a prior run of
    /// Lexer::consume_all_tokens. This function doesn't perform any parsing or mutations.
    pub fn new(tokens: Vec<lexer::Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            position: 0,
            current_token: tokens.get(1).cloned(),
        }
    }

    /// Peek at the next token without consuming it (leave the cursor where it is.) Skips over whitespace, will return an error if you try to peek at a character
    /// but we run out of tokens first.
    pub fn peek(&self) -> Result<lexer::Token, &'static str> {
        let mut iters = 1;

        loop {
            if let Some(token) = self.tokens.get(self.position + iters) {
                match &token.token_type {
                    TokenType::Whitespace(..) => {
                        iters += 1;
                        continue;
                    }
                    TokenType::EOF => {
                        return Err("ran out of tokens");
                    }
                    _ => {
                        return Ok(token.clone());
                    }
                }
            }
        }
    }

    /// Consume N number of tokens (skipping any whitespace entirely)
    pub fn consume_n(&mut self, n: i32) -> Result<Vec<lexer::Token>, &'static str> {
        let mut results = vec![];
        let mut consumed = 0;

        while consumed < n {
            self.position += 1;
            consumed += 1;
            if let Some(token) = self.tokens.get(self.position) {
                match token.token_type {
                    TokenType::Whitespace(..) => {
                        consumed -= 1;
                    }
                    _ => {
                        results.push(token.to_owned());
                    }
                }
            } else {
                return Err("Not enough tokens to consume");
            };
        }

        Ok(results)
    }

    /// Consume the next token, doesn't skip any kind of token.
    pub fn consume(&mut self) -> Option<lexer::Token> {
        self.position += 1;

        self.current_token = self.tokens.get(self.position).map(|token| token.to_owned());

        self.current_token.clone()
    }

    /// Push the cursor back one and update the current_token to the previous character.
    pub fn unconsume(&mut self) -> Option<lexer::Token> {
        self.position -= 1;

        self.current_token = self.tokens.get(self.position).map(|token| token.to_owned());

        self.current_token.clone()
    }

    pub fn parse(&mut self) -> Trie {
        self.position = 0;

        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = &self.current_token {
            let node = match token.token_type {
                TokenType::Keyword(lexer::Keyword::Import) => {
                    match parsers::import::parse_import(self) {
                        Ok(import) => Ok(AstNode::Import(import)),
                        Err(error) => Err(error),
                    }
                }
                TokenType::Keyword(lexer::Keyword::Interface) => {
                    match parsers::interface::parse_interface_declaration(self) {
                        Ok(new_interface) => Ok(AstNode::InterfaceDeclaration(new_interface)),
                        Err(error) => Err(error),
                    }
                }
                TokenType::Keyword(lexer::Keyword::Fn) => match parse_fn(self, false) {
                    Ok(new_fn) => Ok(AstNode::FunctionDeclaration(new_fn)),
                    Err(error) => Err(error),
                },
                TokenType::Keyword(lexer::Keyword::Var) => todo!(),
                TokenType::Keyword(lexer::Keyword::Enum) => match parse_enum_declaration(self) {
                    Ok(new_enum) => Ok(AstNode::EnumDeclaration(new_enum)),
                    Err(error) => Err(error),
                },
                TokenType::Keyword(lexer::Keyword::Match) => todo!(),
                TokenType::Keyword(lexer::Keyword::If) => todo!(),
                TokenType::SOI => continue,
                TokenType::EOF => break,
                TokenType::IntegerLiteral(value) => {
                    Ok(AstNode::LiteralNumber(Literal::Number(value)))
                }
                TokenType::FloatLiteral(f) => Ok(AstNode::LiteralFloat(Literal::Float(f))),
                TokenType::Symbol(lexer::Symbol::DoubleSpeechMark) => {
                    match parse_string_literal(self, lexer::Symbol::DoubleSpeechMark) {
                        Ok(literal) => Ok(AstNode::Expression(Expression::Literal(literal))),
                        Err(error) => Err(error),
                    }
                }
                TokenType::Symbol(lexer::Symbol::SingleSpeechMark) => {
                    match parse_string_literal(self, lexer::Symbol::SingleSpeechMark) {
                        Ok(literal) => Ok(AstNode::Expression(Expression::Literal(literal))),
                        Err(error) => Err(error),
                    }
                }
                TokenType::Symbol(lexer::Symbol::OpenBlock) => todo!(),
                TokenType::Symbol(lexer::Symbol::CloseBlock) => todo!(),
                TokenType::Whitespace(_) => continue,
                TokenType::Void => continue,
                _ => Err(super::syntax_error::SyntaxError::UnexpectedToken(
                    token.clone(),
                )),
            };

            match node {
                Ok(node) => {
                    tree.nodes.push(node);
                }
                Err(e) => panic!("{}", e),
            };

            self.consume();
        }

        tree
    }
}
