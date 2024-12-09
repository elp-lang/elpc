use pest_ast::FromPest;

use crate::parser::Rule;

use super::import::Import;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression {
    #[pest_ast(rule(Rule::import))]
    Import(Import),
}
