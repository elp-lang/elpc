pub(crate) mod elp_type;
pub(crate) mod export;
pub(crate) mod expression;
pub(crate) mod import;
pub(crate) mod variable;

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

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::string))]
pub struct StringValue {
    #[pest_ast(inner(with(span_into_string)))]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ElpParser;
    use from_pest::FromPest;
    use import::Import;
    use pest::Parser;

    use crate::ast::{
        import::{ImportModulePath, ImportName, ImportNameAlias},
        StringValue,
    };

    #[test]
    fn single_expression_ast_generation() {
        let expression_str = "import {Bar, Baz as BazAlias} from \"foo\"";
        let mut pairs = ElpParser::parse(Rule::expression, expression_str).unwrap();
        let ast = Expression::from_pest(&mut pairs).unwrap();

        assert_eq!(
            ast,
            Expression::Import(Box::new(Import {
                names: vec![
                    ImportName {
                        name: "Bar".into(),
                        alias: None,
                    },
                    ImportName {
                        name: "Baz".to_string(),
                        alias: Some(ImportNameAlias {
                            alias: "BazAlias".into()
                        }),
                    }
                ],
                module_path: ImportModulePath {
                    module_path: StringValue {
                        value: "foo".into()
                    }
                }
            })),
        )
    }
}
