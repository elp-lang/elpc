use super::{export::Export, import::Import};
use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression {
    Import(Box<Import>),
    Export(Box<Export>),
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        ast::{
            import::{ImportModulePath, ImportName, ImportNameAlias},
            StringValue,
        },
        parser::ElpParser,
    };
    use from_pest::FromPest;
    use pest::Parser;

    #[test]
    fn parse_import_expression() {
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
