use crate::ast::{
    lexer::{AccessModifier::Pub, Symbol, TokenType},
    lexer_parser::{Fn, Identifier, Parameter, Parser, Type},
    syntax_error::SyntaxError,
};

use super::interface::parse_interface_declaration;

pub fn parse_fn(parser: &mut Parser) -> Result<Fn, SyntaxError> {
    let mut fn_declaration = Fn {
        name: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        },
        params: vec![],
        returns: Identifier {
            name: "".into(),
            immutable: true,
            access_modifier: Pub,
        },
    };

    let mut param = Parameter {
        name: Identifier {
            immutable: true,
            access_modifier: Pub,
            name: "".into(),
        },
        r#type: Type::TypeName(Identifier {
            immutable: true,
            access_modifier: Pub,
            name: "".into(),
        }),
    };

    // the next 3 tokens could/should be an ident, opening parenthesis and parameter.
    let next_tokens = parser.consume_n(3);

    Ok(fn_declaration)
}
