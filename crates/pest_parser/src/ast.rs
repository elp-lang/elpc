use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct ImportName {
    pub name: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct ImportNameAlias {
    pub name: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import {
    #[pest_ast(outer(with(span_into_str), with(str::parse), with(Result::unwrap)))]
    pub names: f64,
}
