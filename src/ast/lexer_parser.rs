use crate::ast::lexer::{self, TokenType};

use super::parsers::{self, funcs::parse_fn};

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Import(ImportStatement),
    InterfaceDeclaration(InterfaceDeclaration),
    EnumDeclaration(Identifier, Vec<EnumVariant>),
    VariableDeclaration(Identifier, Option<Type>, Option<Expression>),
    FunctionDeclaration(Fn),
    ExpressionStatement(Expression),
    IfStatement(Expression, Option<Block>, Option<IfStatement>),
    MatchStatement(Expression, Vec<MatchCase>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Trie {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fn {
    pub name: Option<Identifier>,
    pub params: Vec<Parameter>,
    pub returns: Box<Type>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ImportStatement {
    pub members: Vec<Identifier>,
    pub source_path: String,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ObjectDeclaration {
    pub name: Identifier,
    pub implements: Vec<*const InterfaceDeclaration>,
    pub members: Vec<*const InterfaceProperty>,
}

#[derive(Default, Debug, PartialEq, Eq)]
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
pub struct Enum {
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumVariantType {
    Option,
    Action(Vec<Parameter>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Option<Identifier>,
    pub r#type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    TypeName(Identifier),
    FnType(Fn),
    InterfaceType(InterfaceDeclaration),
    ObjectType(ObjectDeclaration),
    EnumType(Enum),
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

#[derive(Default, Debug, PartialEq, Eq)]
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

    // Consume N number of tokens (skipping any whitespace entirely)
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
        self.position = 0;

        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = &self.current_token {
            let node = match token.token_type {
                lexer::TokenType::Keyword(lexer::Keyword::Import) => {
                    match parsers::import::parse_import(self) {
                        Ok(import) => Ok(AstNode::Import(import)),
                        Err(error) => Err(error),
                    }
                }
                lexer::TokenType::Keyword(lexer::Keyword::Interface) => {
                    let interface = parsers::interface::parse_interface_declaration(self);

                    match interface {
                        Ok(new_interface) => Ok(AstNode::InterfaceDeclaration(new_interface)),
                        Err(error) => Err(error),
                    }
                }
                lexer::TokenType::Keyword(lexer::Keyword::Fn) => match parse_fn(self, false) {
                    Ok(new_fn) => Ok(AstNode::FunctionDeclaration(new_fn)),
                    Err(error) => Err(error),
                },
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
                TokenType::SOI => todo!(),
                TokenType::EOF => todo!(),
                TokenType::LiteralBoolean(_) => todo!(),
                TokenType::Keyword(_) => todo!(),
                TokenType::ReturnType => todo!(),
                TokenType::Ident(_) => todo!(),
                TokenType::Symbol(_) => todo!(),
                TokenType::Whitespace(_) => todo!(),
                TokenType::AccessModifier(_) => todo!(),
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
    fn test_parse_basic_interface() {
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

    #[test]
    fn test_parse_interface() {
        let input = "interface MyInterface {
            .property: string
            .property: interface {
                .property: int32
                .property : number
            }
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
                    members: vec!(
                        InterfaceProperty {
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
                        },
                        InterfaceProperty {
                            name: Identifier {
                                immutable: true,
                                access_modifier: lexer::AccessModifier::Pub,
                                name: "property".to_string()
                            },
                            r#type: Type::InterfaceType(InterfaceDeclaration {
                                name: Identifier {
                                    name: "".into(),
                                    immutable: true,
                                    access_modifier: lexer::AccessModifier::Pub,
                                },
                                members: vec!(
                                    InterfaceProperty {
                                        name: Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: "property".to_string()
                                        },
                                        r#type: Type::TypeName(Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: "int32".to_string()
                                        })
                                    },
                                    InterfaceProperty {
                                        name: Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: "property".to_string()
                                        },
                                        r#type: Type::TypeName(Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: "number".to_string()
                                        })
                                    },
                                )
                            })
                        }
                    ),
                }))
            }
        );
    }
}
