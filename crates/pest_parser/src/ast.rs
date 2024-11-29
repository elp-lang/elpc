use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name))]
pub struct ImportName<'pest> {
    #[pest_ast(outer())]
    pub name: &'pest str,
    #[pest_ast(outer())]
    pub alias: Option<&'pest str>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name_alias))]
pub struct ImportNameAlias<'pest> {
    pub name: &'pest str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    pub names: f64,
    pub module: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;
