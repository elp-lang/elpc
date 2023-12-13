use crate::ast::lexer;

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

    fn parse_import(&mut self) -> Result<AstNode, String> {
        print!("parsing import\n");
        match &self.current_token {
            None => return Err(format!("Expected 'import'")),
            Some(token) => {
                if token.token_type != lexer::TokenType::Keyword(lexer::Keyword::Import) {
                    return Err(format!("Expected 'import', got '{:#?}'.", token.token_type));
                } else {
                    let mut import_statement = ImportStatement {
                        members: vec![],
                        source_path: "".to_string(),
                    };

                    // Collect all tokens until we get to lexer::TokenType::CloseBlock
                    // this will include Ident, Comma and Whitespace.
                    let mut found_opening_brace = false;
                    let mut found_closing_brace = false;
                    let mut imports: Vec<lexer::Token> = vec![];
                    while self.consume().is_some() {
                        print!("in import loop, token: {:#?}\n", self.current_token);
                        if let Some(token) = &self.current_token {
                            match &token.token_type {
                                // Skip whitespace and the opening brace but mark it as found.
                                lexer::TokenType::OpenBlock => {
                                    if found_opening_brace {
                                        return Err::<AstNode, String>(
                                            "Unexpected opening brace".to_string(),
                                        );
                                    } else {
                                        found_opening_brace = true;
                                    }
                                }
                                lexer::TokenType::CloseBlock => {
                                    if !found_opening_brace {
                                        return Err(
                                            "Expected opening brace but found closing brace."
                                                .to_string(),
                                        );
                                    } else {
                                        found_closing_brace = true;
                                    }
                                }
                                lexer::TokenType::Keyword(lexer::Keyword::Import) => {
                                    continue;
                                }
                                lexer::TokenType::Keyword(lexer::Keyword::From) => {
                                    if !found_closing_brace {
                                        return Err("Expected } but found from.".to_string());
                                    }
                                }
                                lexer::TokenType::Whitespace(..) => continue,
                                lexer::TokenType::Ident(..) => {
                                    if found_closing_brace {
                                        import_statement.source_path = token.value.clone();
                                    } else {
                                        imports.push(token.clone());
                                    }
                                }
                                tok => {
                                    println!("Found unexpected {:#?}", tok.clone());
                                    break;
                                }
                            };
                        }
                    }

                    if !found_closing_brace {
                        return Err("Expected a }".to_string());
                    }

                    import_statement.members = imports
                        .iter()
                        .map(|token| Identifier {
                            name: token.value.clone(),
                            access_modifier: lexer::AccessModifier::Pub,
                        })
                        .collect();

                    return Ok(AstNode::Import(import_statement));
                }
            }
        }
    }

    pub fn parse(&mut self) -> Trie {
        let mut tree = Trie { nodes: vec![] };

        while let Some(token) = &self.current_token {
            print!("current: {:#?}\n", token);
            let node = match token.token_type {
                lexer::TokenType::Keyword(lexer::Keyword::Import) => self.parse_import(),
                lexer::TokenType::Keyword(lexer::Keyword::Fn) => todo!(),
                lexer::TokenType::Keyword(lexer::Keyword::Var) => todo!(),
                lexer::TokenType::SOI => continue,
                lexer::TokenType::EOF => todo!(),
                lexer::TokenType::LiteralBoolean(_) => todo!(),
                lexer::TokenType::DoubleSpeechMark => todo!(),
                lexer::TokenType::SingleSpeechMark => todo!(),
                lexer::TokenType::OpenBlock => todo!(),
                lexer::TokenType::CloseBlock => todo!(),
                lexer::TokenType::ReturnType => todo!(),
                lexer::TokenType::Ident(_) => todo!(),
                lexer::TokenType::Symbol(_) => todo!(),
                lexer::TokenType::Whitespace(_) => continue,
                lexer::TokenType::AccessModifier(_) => todo!(),
            };

            print!("got node? {:#?}\n", node);
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
                        access_modifier: lexer::AccessModifier::Const,
                    }),
                    source_path: "elp".to_string(),
                }))
            }
        );
    }
}
