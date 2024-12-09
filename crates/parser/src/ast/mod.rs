pub(crate) mod expression;
pub(crate) mod import;

use expression::Expression;
use pest::Span;
use pest_ast::FromPest;

use crate::parser::Rule;

fn span_into_string(span: Span) -> String {
    span.as_str().into()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct StringValue {
    #[pest_ast(inner(with(span_into_string)))]
    pub value: String,
}
