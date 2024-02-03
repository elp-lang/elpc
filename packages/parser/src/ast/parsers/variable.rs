use crate::ast::{
    lexer::AccessModifier,
    lexer_parser::{Identifier, Parser, VariableDeclaration},
    syntax_error::SyntaxError,
};

pub fn parse_expression(parser: &mut Parser) -> Result<VariableDeclaration, SyntaxError> {
    let declaration = VariableDeclaration {
        ident: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: AccessModifier::Pub,
        },
        r#type: crate::ast::lexer_parser::Type::Void,
        value: None,
    };

    while let Some(token) = parser.consume() {
        {
            return Err(SyntaxError::UnexpectedToken(token.clone()));
        }
    }

    Ok(declaration)
}
