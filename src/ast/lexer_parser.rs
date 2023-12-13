use crate::ast::{
    lexer::{self, Symbol, TokenType},
    syntax_error::SyntaxError,
};

#[derive(Debug, PartialEq, Eq)]
pub enum AstNode {
    Import(ImportStatement),
    InterfaceDeclaration(Identifier, Vec<InterfaceProperty>),
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
pub struct Interface {
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
    current_token: Option<lexer::Token>,
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            position: 0,
            current_token: tokens.get(1).cloned(),
        }
    }

    fn consume(&mut self) -> Option<lexer::Token> {
        self.position += 1;

        self.current_token = if let Some(token) = self.tokens.get(self.position) {
            Some(token.to_owned())
        } else {
            None
        };

        self.current_token.clone()
    }

    fn parse_import(&mut self) -> Result<AstNode, SyntaxError> {
        println!("parsing import");
        match &self.current_token.clone() {
            None => return Err(SyntaxError::MissingToken("import")),
            Some(token) => {
                if token.token_type != lexer::TokenType::Keyword(lexer::Keyword::Import) {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        lexer::TokenType::Keyword(lexer::Keyword::Import),
                        token.clone(),
                    ));
                } else {
                    let mut import_statement = ImportStatement {
                        members: vec![],
                        source_path: "".to_string(),
                    };

                    // Collect all tokens until we get to lexer::TokenType::CloseBlock
                    // this will include Ident, Comma and Whitespace and quotation marks.
                    // We skip most of these characters.
                    let mut found_opening_brace = false;
                    let mut found_closing_brace = false;
                    let mut imports: Vec<lexer::Token> = vec![];
                    while self.consume().is_some() {
                        if let Some(token) = &self.current_token {
                            match &token.token_type {
                                // Skip whitespace and the opening brace but mark it as found.
                                lexer::TokenType::Symbol(lexer::Symbol::OpenBlock) => {
                                    if found_opening_brace {
                                        return Err::<AstNode, SyntaxError>(
                                            SyntaxError::UnexpectedToken(token.clone()),
                                        );
                                    } else {
                                        found_opening_brace = true;
                                    }
                                }
                                lexer::TokenType::Symbol(lexer::Symbol::CloseBlock) => {
                                    if !found_opening_brace {
                                        return Err(SyntaxError::UnexpectedTokenButGot(
                                            TokenType::Symbol(lexer::Symbol::OpenBlock),
                                            token.clone(),
                                        ));
                                    } else {
                                        found_closing_brace = true;
                                    }
                                }
                                lexer::TokenType::Keyword(lexer::Keyword::Import) => {
                                    continue;
                                }
                                lexer::TokenType::Keyword(lexer::Keyword::From) => {
                                    if !found_closing_brace {
                                        return Err(SyntaxError::UnexpectedTokenButGot(
                                            TokenType::Symbol(lexer::Symbol::CloseBlock),
                                            token.clone(),
                                        ));
                                    }
                                }
                                lexer::TokenType::Symbol(..) => continue,
                                lexer::TokenType::Whitespace(..) => continue,
                                lexer::TokenType::Ident(..) => {
                                    if found_closing_brace {
                                        import_statement.source_path = token.value.clone();
                                    } else {
                                        imports.push(token.clone());
                                    }
                                }
                                lexer::TokenType::EOF => break,
                                _ => {
                                    return Err(SyntaxError::UnexpectedToken(
                                        self.current_token.clone().unwrap(),
                                    ));
                                }
                            };
                        }
                    }

                    if !found_closing_brace {
                        return Err(SyntaxError::MissingToken("}"));
                    }

                    import_statement.members = imports
                        .iter()
                        .map(|token| Identifier {
                            name: token.value.clone(),
                            immutable: true,
                            access_modifier: lexer::AccessModifier::Pub,
                        })
                        .collect();

                    return Ok(AstNode::Import(import_statement));
                }
            }
        }
    }

    fn parse_interface_declaration(&mut self) -> Result<AstNode, SyntaxError> {
        println!("parsing interface");
        match &self.current_token.clone() {
            None => return Err(SyntaxError::MissingToken("interface")),
            Some(token) => {
                let mut found_opening_brace = false;
                let mut found_closing_brace = false;
                let mut interface: Interface = Interface {
                    name: Identifier {
                        immutable: true,
                        access_modifier: lexer::AccessModifier::Pub,
                        name: "".to_string(),
                    },
                    members: vec![],
                };

                while self.consume().is_some() {
                    if let Some(token) = &self.current_token {
                        match &token.token_type {
                            TokenType::Keyword(lexer::Keyword::Interface) => continue,
                            TokenType::Ident(name) => {
                                if found_opening_brace && interface.members.len() == 0 {
                                    interface.name.name = name.to_string();
                                } else {
                                    // We want to look ahead two tokens to get the type,
                                    // this will advance the cursor for us.
                                    // The "next" token *should* be a Symbol::Colon followed by
                                    // an Ident
                                    // @TODO Support pointer types.
                                    if let Some(colon) = self.consume() {
                                        if colon.token_type != TokenType::Symbol(Symbol::Colon) {
                                            return Err(SyntaxError::UnexpectedTokenButGot(
                                                TokenType::Symbol(Symbol::Colon),
                                                colon,
                                            ));
                                        }
                                    }

                                    interface.members.push(InterfaceProperty {
                                        name: Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: name.to_string(),
                                        },
                                        r#type: Type::TypeName(Identifier {
                                            immutable: true,
                                            access_modifier: lexer::AccessModifier::Pub,
                                            name: "unknown".to_string(),
                                        }),
                                    });
                                }
                            }
                            _ => {
                                return Err(SyntaxError::UnexpectedToken(
                                    self.current_token.clone().unwrap(),
                                ));
                            }
                        }
                    }
                }
            }
        };

        return Ok(());
    }

    pub fn parse(&mut self) -> Trie {
        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = &self.current_token {
            let node = match token.token_type {
                lexer::TokenType::Keyword(lexer::Keyword::Import) => self.parse_import(),
                lexer::TokenType::Keyword(lexer::Keyword::Interface) => {
                    self.parse_interface_declaration()
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
    fn test_parse_input() {
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
}
