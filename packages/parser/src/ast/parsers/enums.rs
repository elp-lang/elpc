use crate::ast::{
    lexer::{AccessModifier, Symbol, TokenType},
    lexer_parser::{EnumDeclaration, EnumVariant, Identifier, Parser},
    syntax_error::SyntaxError,
};

fn parse_enum_variant(parser: &mut Parser) -> Result<EnumVariant, SyntaxError> {
    let mut variation = EnumVariant {
        ..Default::default()
    };

    while let Some(token) = parser.consume() {
        match &token.token_type {
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(variation)
}

pub fn parse_enum_declaration(parser: &mut Parser) -> Result<EnumDeclaration, SyntaxError> {
    let mut declaration = EnumDeclaration {
        name: None,
        variants: vec![],
    };

    let mut found_open_brace = false;

    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Ident(val) => {
                if found_open_brace {
                    return Err(SyntaxError::UnexpectedTokenButGot(
                        TokenType::Symbol(Symbol::Period),
                        token,
                    ));
                } else {
                    declaration.name = Some(Identifier {
                        name: val.clone(),
                        immutable: true,
                        access_modifier: AccessModifier::Pub,
                    });
                    continue;
                }
            }
            TokenType::Symbol(Symbol::OpenBlock) => {
                found_open_brace = true;
                continue;
            }
            TokenType::Symbol(Symbol::Period) => match parse_enum_variant(parser) {
                Ok(variant) => {
                    declaration.variants.push(variant);
                }
                Err(err) => {
                    return Err(err);
                }
            },
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Ok(declaration)
}

#[cfg(test)]
mod tests {}
