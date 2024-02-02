use crate::ast::lexer::{self, TokenType};

use super::parsers::{
    self, enums::parse_enum_declaration, funcs::parse_fn, string_literals::parse_string_literal,
};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Import(ImportStatement),
    InterfaceDeclaration(InterfaceDeclaration),
    EnumDeclaration(EnumDeclaration),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(Fn),
    LiteralNumber(Literal),
    LiteralFloat(Literal),
    Expression(Expression),
    IfStatement(Expression, Option<Block>, Option<IfStatement>),
    MatchStatement(Expression, Vec<MatchCase>),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub r#type: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Trie {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub struct Fn {
    pub name: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub returns: Box<Type>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ImportStatementMember {
    pub name: Identifier,
    pub alias: Option<String>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ImportStatement {
    pub members: Vec<ImportStatementMember>,
    pub source_path: String,
}

#[derive(Default, Debug, PartialEq)]
pub struct ObjectDeclaration {
    pub name: Identifier,
    pub implements: Vec<*const InterfaceDeclaration>,
    pub members: Vec<*const InterfaceProperty>,
}

#[derive(Default, Debug, PartialEq)]
pub struct InterfaceDeclaration {
    pub name: Identifier,
    pub members: Vec<InterfaceProperty>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceProperty {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Default, Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub r#type: Option<EnumVariantType>,
}

#[derive(Debug, PartialEq)]
pub struct EnumDeclaration {
    pub name: Option<Identifier>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Default, Debug, PartialEq)]
pub enum EnumVariantType {
    #[default]
    Option,
    Action(Vec<Parameter>),
}

#[derive(Default, Debug, PartialEq)]
pub struct Parameter {
    pub name: Option<Identifier>,
    pub r#type: Type,
}

#[derive(Default, Debug, PartialEq)]
pub enum Type {
    TypeName(Identifier),
    FnType(Fn),
    Literal(Literal),
    InterfaceType(InterfaceDeclaration),
    ObjectType(ObjectDeclaration),
    EnumType(EnumDeclaration),
    Undefined,
    #[default]
    Void,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    FunctionCall(Identifier, Vec<Argument>),
    IfStatement(Box<IfStatement>),
    Block(Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Option<Block>,
    pub branches: Option<Vec<Box<AstNode>>>,
}

#[derive(Debug, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub body: AstNode,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    MemberAccess(Identifier),
    Boolean(bool),
}

#[derive(Default, Debug, PartialEq)]
pub struct Identifier {
    pub immutable: bool,
    pub access_modifier: lexer::AccessModifier,
    pub name: String,
}

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
