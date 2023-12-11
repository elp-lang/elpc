use crate::ast::lexer;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Trie {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug)]
pub struct ImportStatement {
    pub members: Vec<Identifier>,
    pub source_path: String,
}

#[derive(Debug)]
pub struct InterfaceProperty {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Identifier,
    pub variant_type: Option<EnumVariantType>,
}

#[derive(Debug)]
pub enum EnumVariantType {
    Option,
    Action(Vec<Parameter>),
}

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Debug)]
pub enum Type {
    TypeName(Identifier),
    FunctionType(Vec<Type>, Box<Type>),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    FunctionCall(Identifier, Vec<Argument>),
    Block(Vec<AstNode>),
    // Other expression types can be added based on your language's syntax.
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<AstNode>,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Option<Block>,
    pub else_statement: Option<Box<AstNode>>,
}

#[derive(Debug)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub body: AstNode,
}

#[derive(Debug)]
pub enum Pattern {
    MemberAccess(Identifier),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Identifier {
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
            tokens,
            position: 0,
            current_token: None,
        }
    }

    fn consume(&mut self) -> Option<lexer::Token> {
        self.position += 1;

        self.current_token = if let Some(token) = self.tokens.get(self.position) {
            Some(token.to_owned())
        } else {
            None
        };

        self.current_token
    }

    fn parse_import(&mut self) -> Result<AstNode, String> {
        match self.current_token {
            None => Err(format!("Expected 'import'")),
            Some(token) => {
                if token.token_type != lexer::TokenType::Keyword(lexer::Keyword::Import) {
                    Err(format!("Expected 'import', got '{:#?}'.", token.token_type))
                } else {
                    let import_statement = ImportStatement {
                        members: vec![],
                        source_path: "".to_string(),
                    };

                    // Collect all tokens until we get to lexer::TokenType::CloseBlock
                    // this will include Ident, Comma and Whitespace.
                    let mut found_opening_brace = false;
                    let mut found_closing_brace = false;
                    let mut imports: Vec<lexer::Token> = vec![];
                    while self.consume().is_some() {
                        if let Some(token) = self.current_token {
                            match token.token_type {
                                // Skip whitespace and the opening brace but mark it as found.
                                lexer::TokenType::OpenBlock => {
                                    if found_opening_brace {
                                        Err("Unexpected opening brace")
                                    } else {
                                        found_opening_brace = true;
                                        continue;
                                    }
                                }
                                lexer::TokenType::CloseBlock => {
                                    if !found_opening_brace {
                                        Err("Expected opening brace but found closing brace.")
                                    } else {
                                        found_closing_brace = true;
                                        break;
                                    }
                                }
                                lexer::TokenType::Whitespace(..) => continue,
                                lexer::TokenType::Ident(..) => {
                                    imports.append(token);
                                    continue;
                                }
                                _ => Err(format!("Expected import value. got {:#?}", token)),
                            };
                        }
                    }

                    import_statement.members = imports
                        .iter()
                        .map(|&token| Identifier {
                            name: token.value,
                            access_modifier: lexer::AccessModifier::Pub,
                        })
                        .collect();

                    Ok(AstNode::Import(import_statement))
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = self.current_token {
            let node = match token.token_type {
                lexer::TokenType::Keyword(lexer::Keyword::Import) => self.parse_import(),
                lexer::TokenType::Keyword(lexer::Keyword::Fn) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::Var) => todo!(),
                lexer::TokenType::SOI => todo!(),
                lexer::TokenType::EOF => todo!(),
                lexer::TokenType::LiteralBoolean(_) => todo!(),
                lexer::TokenType::DoubleSpeechMark => todo!(),
                lexer::TokenType::SingleSpeechMark => todo!(),
                lexer::TokenType::OpenBlock => todo!(),
                lexer::TokenType::CloseBlock => todo!(),
                lexer::TokenType::ReturnType => todo!(),
                lexer::TokenType::Ident(_) => todo!(),
                lexer::TokenType::Symbol(_) => todo!(),
                lexer::TokenType::Whitespace(_) => todo!(),
                lexer::TokenType::AccessModifier(_) => todo!(),
            };

            self.consume();
        }
    }
}
