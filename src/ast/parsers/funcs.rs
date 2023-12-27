use crate::ast::{
    lexer_parser::{Fn, Identifier, Parser, Type},
    syntax_error::SyntaxError,
};

pub fn parse_fn(parser: &mut Parser) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: crate::ast::lexer::AccessModifier::Pub,
        },
        params: vec![],
        returns: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: crate::ast::lexer::AccessModifier::Pub,
        },
    };

    Ok(fn_declaration)
}
