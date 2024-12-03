pub mod import;
pub mod utils;

use import::ImportStatement;
use pest_ast::FromPest;
use utils::span_into_str;

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program<'a> {
    pub expressions: Vec<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::IDENT))]
pub struct IDENT<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub inner: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'a> {
    Import(ImportStatement<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;
