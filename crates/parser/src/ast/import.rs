use pest_ast::FromPest;

use crate::parser::Rule;

use super::span_into_string;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    #[pest_ast(inner(with(span_into_string)))]
    pub names: String,
}
