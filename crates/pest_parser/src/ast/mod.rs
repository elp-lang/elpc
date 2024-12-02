pub mod utils;
use pest_ast::FromPest;
use utils::{span_into_str, span_into_str_option};

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program<'a> {
    pub expressions: Vec<Expression<'a>>,
    _eoi: Eoi,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::IDENT))]
pub struct IDENT<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub ident: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_module_path))]
pub struct ImportModulePath<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub path: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'a> {
    Import(ImportStatement<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct ImportStatement<'a> {
    pub import_names: Vec<ImportName<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name))]
pub struct ImportName<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,

    #[pest_ast(inner(with(span_into_str_option)))]
    pub alias: Option<&'a str>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;
