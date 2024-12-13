use super::expression::Expression;
use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::export))]
pub struct Export {
    pub expression: Expression,
}
