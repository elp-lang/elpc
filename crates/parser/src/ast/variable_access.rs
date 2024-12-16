use crate::parser::Rule;
use pest_ast::FromPest;

use super::{span_into_string, IDENT};

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::pointer_semantics))]
pub enum PointerSemantics {
    #[pest_ast(rule(Rule::POINTER))]
    Pointer,
    #[pest_ast(rule(Rule::REFERENCE))]
    Reference,
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::variable_access))]
pub struct VariableAccess {
    pub pointer_semantics: PointerSemantics,

    #[pest_ast(inner(with(span_into_string)))]
    pub variable_name: String,

    // A member chain is something like this:
    //   `foo.bar.baz` becomes `vec![IDENT { value: "foo".into() }, IDENT { value: "bar".into() }, IDENT { value: "baz".into() }]`
    pub member_chain: Vec<IDENT>,
}
