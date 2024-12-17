use crate::parser::Rule;
use pest_ast::FromPest;

use super::IDENT;

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
    pub pointer_semantics: Option<PointerSemantics>,
    pub variable_name: IDENT,

    // A member chain is something like this:
    //   `foo.bar.baz` becomes `vec![IDENT { value: "foo".into() }, IDENT { value: "bar".into() }, IDENT { value: "baz".into() }]`
    pub member_chain: Vec<IDENT>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ElpParser;
    use from_pest::FromPest;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn variable_access() {
        let expression_str = "hello.world.my.name.is.dave";
        let mut pairs = ElpParser::parse(Rule::variable_access, expression_str).unwrap();
        let ast = VariableAccess::from_pest(&mut pairs).unwrap();

        assert_eq!(
            ast,
            VariableAccess {
                variable_name: IDENT {
                    value: "hello".into(),
                },
                member_chain: vec![
                    IDENT {
                        value: "world".into()
                    },
                    IDENT { value: "my".into() },
                    IDENT {
                        value: "name".into()
                    },
                    IDENT { value: "is".into() },
                    IDENT {
                        value: "dave".into()
                    },
                ],
                pointer_semantics: None,
            }
        )
    }
}
