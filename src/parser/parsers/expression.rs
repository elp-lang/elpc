
use crate::parser::lexer::{Keyword, Symbol, TokenType};
use crate::parser::lexer_parser::{Expression, Literal, Parser};
use crate::parser::syntax_error::SyntaxError;

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
                return match parse_interface_declaration(parser) {
                    Ok(interface) => Ok(Expression::Interface(interface)),
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "interface parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            TokenType::Keyword(Keyword::Fn) => {
                parser.unconsume();
                return match parse_fn(parser) {
                    Ok(r#fn) => Ok(Expression::Function(r#fn)),
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "function parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            TokenType::Keyword(Keyword::Const) | TokenType::Keyword(Keyword::Var) => {
                return match parse_variable(parser) {
                    Ok(var) => {
                        Ok(Expression::VariableDeclaration(Box::new(var)))
                    }
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "variable parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            TokenType::Symbol(Symbol::DoubleSpeechMark) => {
                return match parse_string_literal(parser, Symbol::DoubleSpeechMark) {
                    Ok(literal) => {
                        Ok(Expression::Literal(literal))
                    }
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "string literal parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            TokenType::Symbol(Symbol::SingleSpeechMark) => {
                return match parse_string_literal(parser, Symbol::SingleSpeechMark) {
                    Ok(literal) => {
                        Ok(Expression::Literal(literal))
                    }
                    Err(err) => {
                        Err(SyntaxError::WrappedWithContextMessage(
                            "string literal parsing".into(),
                            Box::new(err),
                        ))
                    }
                }
            }
            TokenType::Ident(_) => return match parse_ident(parser) {
                Ok(result) => {
                    Ok(result)
                }
                Err(err) => {
                    Err(SyntaxError::WrappedWithContextMessage(
                        "ident parsing".into(),
                        Box::new(err),
                    ))
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
                return Err(SyntaxError::WrappedWithContextMessage(
                    "parsing expression".to_string(),
                    Box::new(SyntaxError::UnexpectedToken(token)),
                ));
            }
        }
    }

    Err(SyntaxError::MissingToken("Expression"))
}
