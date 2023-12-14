use crate::ast::{
    lexer::{self, Symbol, Token, TokenType},
    syntax_error::SyntaxError,
};

use super::parsers;

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Import(ImportStatement),
    InterfaceDeclaration(InterfaceDeclaration),
    EnumDeclaration(Identifier, Vec<EnumVariant>),
    VariableDeclaration(Identifier, Option<Type>, Option<Expression>),
    FunctionDeclaration(Identifier, Vec<Parameter>, Option<Type>, Option<Block>),
    ExpressionStatement(Expression),
    IfStatement(Expression, Option<Block>, Option<IfStatement>),
    MatchStatement(Expression, Vec<MatchCase>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportStatement {
    pub members: Vec<Identifier>,
    pub source_path: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterfaceDeclaration {
    pub name: Identifier,
    pub members: Vec<InterfaceProperty>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterfaceProperty {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub variant_type: Option<EnumVariantType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumVariantType {
    Option,
    Action(Vec<Parameter>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    TypeName(Identifier),
    FunctionType(Vec<Type>, Box<Type>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    FunctionCall(Identifier, Vec<Argument>),
    Block(Vec<AstNode>),
    // Other expression types can be added based on your language's syntax.
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<AstNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Option<Block>,
    pub else_statement: Option<Box<AstNode>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub body: AstNode,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pattern {
    MemberAccess(Identifier),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Eq)]
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
    pub fn new(tokens: Vec<lexer::Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            position: 0,
            current_token: tokens.get(1).cloned(),
        }
    }

    pub fn consume_n(&mut self, n: i32) -> Option<vec![lexer::Token]> {
        self.position += 1;

        self.current_token = if let Some(token) = self.tokens.get(self.position) {
            Some(token.to_owned())
        } else {
            None
        };

        self.current_token.clone()
    }

    pub fn consume(&mut self) -> Option<lexer::Token> {
        self.position += 1;

        self.current_token = if let Some(token) = self.tokens.get(self.position) {
            Some(token.to_owned())
        } else {
            None
        };

        self.current_token.clone()
    }

    pub fn parse(&mut self) -> Trie {
        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = &self.current_token {
            let node = match token.token_type {
                lexer::TokenType::Keyword(lexer::Keyword::Import) => {
                    parsers::import::parse_import(self)
                }
                lexer::TokenType::Keyword(lexer::Keyword::Interface) => {
                    parsers::interface::parse_interface_declaration(self)
                }
                lexer::TokenType::Keyword(lexer::Keyword::Fn) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::Var) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::From) => continue,
                lexer::TokenType::Keyword(lexer::Keyword::Enum) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::Match) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::If) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::ElseIf) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::Else) => todo!(),
                lexer::TokenType::SOI => continue,
                lexer::TokenType::EOF => break,
                lexer::TokenType::LiteralBoolean(_) => todo!(),
                lexer::TokenType::Symbol(lexer::Symbol::DoubleSpeechMark) => todo!(),
                lexer::TokenType::Symbol(lexer::Symbol::SingleSpeechMark) => todo!(),
                lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => todo!(),
                lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => todo!(),
                lexer::TokenType::ReturnType => todo!(),
                lexer::TokenType::Ident(_) => todo!(),
                lexer::TokenType::Symbol(_) => todo!(),
                lexer::TokenType::Whitespace(_) => continue,
                lexer::TokenType::AccessModifier(_) => todo!(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_import() {
        let input = "import { Thing } from \"elp\"".to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!(AstNode::Import(ImportStatement {
                    members: vec!(Identifier {
                        name: "Thing".to_string(),
                        immutable: true,
                        access_modifier: lexer::AccessModifier::Pub,
                    }),
                    source_path: "elp".to_string(),
                }))
            }
        );
    }

    #[test]
    fn test_parse_interface() {
        let input = "interface MyInterface {
            .property: string
        }"
        .to_string();
        let mut lexer = Lexer::new(input.clone());
        let tokens = lexer.consume_all_tokens();
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse(),
            Trie {
                nodes: vec!(AstNode::InterfaceDeclaration(InterfaceDeclaration {
                    name: Identifier {
                        immutable: true,
                        access_modifier: lexer::AccessModifier::Pub,
                        name: "MyInterface".to_string(),
                    },
                    members: vec!(InterfaceProperty {
                        name: Identifier {
                            immutable: true,
                            access_modifier: lexer::AccessModifier::Pub,
                            name: "property".to_string()
                        },
                        r#type: Type::TypeName(Identifier {
                            immutable: true,
                            access_modifier: lexer::AccessModifier::Pub,
                            name: "string".to_string()
                        })
                    }),
                }))
            }
        );
    }
}
