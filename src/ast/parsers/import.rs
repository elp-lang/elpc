use crate::ast::{
    lexer::{self, TokenType},
    lexer_parser::{AstNode, Identifier, ImportStatement, Parser},
    syntax_error::SyntaxError,
};

pub fn parse_import(parser: &mut Parser) -> Result<AstNode, SyntaxError> {
    match parser.current_token.clone() {
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
                while parser.consume().is_some() {
                    if let Some(token) = &parser.current_token {
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
                                    let ast_node = Err(SyntaxError::UnexpectedTokenButGot(
                                        TokenType::Symbol(lexer::Symbol::CloseBlock),
                                        token.clone(),
                                    ));
                                    return ast_node;
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
                                    parser.current_token.clone().unwrap(),
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
