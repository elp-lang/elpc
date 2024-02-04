use crate::ast::{
    lexer::AccessModifier, lexer_parser::Parser, syntax_error::SyntaxError, Identifier, Type,
    VariableDeclaration,
};

pub fn parse_expression(parser: &mut Parser) -> Result<VariableDeclaration, SyntaxError> {
    let declaration = VariableDeclaration {
        ident: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: AccessModifier::Pub,
        },
        r#type: Type::Void,
        value: None,
    };

    while let Some(token) = parser.consume() {
        {
            return Err(SyntaxError::UnexpectedToken(token.clone()));
        }
    }

    Ok(declaration)
}
