use crate::ast::{
    lexer::{Keyword, Symbol, TokenType},
    lexer_parser::{Expression, Literal, Parser},
    syntax_error::SyntaxError,
};

use super::{
    funcs::parse_fn, ident::parse_ident, interface::parse_interface_declaration,
    string_literals::parse_string_literal, variable::parse_variable,
};

// An Expression can be any of the following types of code:
//   * FunctionCall
//   * Function
//   * Literal
//   * Interface
//   * Enum
//   * Import
//   * Variable
pub fn parse_expression(parser: &mut Parser) -> Result<Expression, SyntaxError> {
    while let Some(token) = parser.consume() {
        match &token.token_type {
            TokenType::Keyword(Keyword::Interface) => {
                parser.unconsume();
                match parse_interface_declaration(parser) {
                    Ok(interface) => return Ok(Expression::Interface(interface)),
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "interface parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            TokenType::Keyword(Keyword::Fn) => {
                parser.unconsume();
                match parse_fn(parser) {
                    Ok(r#fn) => return Ok(Expression::Function(r#fn)),
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "function parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            TokenType::Keyword(Keyword::Const) | TokenType::Keyword(Keyword::Var) => {
                match parse_variable(parser) {
                    Ok(var) => {
                        return Ok(Expression::VariableDeclaration(Box::new(var)));
                    }
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "variable parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            TokenType::Symbol(Symbol::DoubleSpeechMark) => {
                match parse_string_literal(parser, Symbol::DoubleSpeechMark) {
                    Ok(literal) => {
                        return Ok(Expression::Literal(literal));
                    }
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "string literal parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            TokenType::Symbol(Symbol::SingleSpeechMark) => {
                match parse_string_literal(parser, Symbol::SingleSpeechMark) {
                    Ok(literal) => {
                        return Ok(Expression::Literal(literal));
                    }
                    Err(err) => {
                        return Err(SyntaxError::WrappedWithContextMessage(
                            "string literal parsing".into(),
                            Box::new(err),
                        ));
                    }
                }
            }
            TokenType::Ident(val) => match parse_ident(parser) {
                Ok(result) => {}
                Err(err) => {
                    return Err(SyntaxError::WrappedWithContextMessage(
                        "string literal parsing".into(),
                        Box::new(err),
                    ));
                }
            },
            TokenType::BooleanLiteral(b) => {
                return Ok(Expression::Literal(Literal::Boolean(*b)));
            }
            TokenType::FloatLiteral(f) => {
                return Ok(Expression::Literal(Literal::Float(*f)));
            }
            TokenType::IntegerLiteral(i) => {
                return Ok(Expression::Literal(Literal::Number(*i)));
            }
            TokenType::StringLiteral(str) => {
                return Ok(Expression::Literal(Literal::String(str.to_string())));
            }
            TokenType::Whitespace(_) => continue,
            _ => {
                return Err(SyntaxError::UnexpectedToken(token));
            }
        }
    }

    Err(SyntaxError::MissingToken("Expression"))
}
