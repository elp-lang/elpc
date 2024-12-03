use pest_ast::FromPest;

use crate::parser::Rule;

use super::utils::{span_into_str, span_into_str_option};

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct ImportStatement<'a> {
    pub import_names: Vec<ImportName<'a>>,
    pub module_path: ImportModulePath<'a>,
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
#[pest_ast(rule(Rule::import_module_path))]
pub struct ImportModulePath<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub path: &'a str,
}
