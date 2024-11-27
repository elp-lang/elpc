use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name))]
pub struct ImportName {
    pub name: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name_alias))]
pub struct ImportNameAlias {
    pub name: String,
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
