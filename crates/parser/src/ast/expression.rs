use super::{export::Export, import::Import, variable::VariableDeclaration};
use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression {
    Import(Box<Import>),
    Export(Box<Export>),
    VariableDeclaration(Box<VariableDeclaration>),
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        ast::{
            import::{ImportModulePath, ImportName, ImportNameAlias},
            variable::VariableMutability,
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

    #[test]
    fn parse_export_expression() {
        let expression_str = "export const hello = \"world\"";
        let mut pairs = ElpParser::parse(Rule::expression, expression_str).unwrap();
        let ast = Expression::from_pest(&mut pairs).unwrap();

        assert_eq!(
            ast,
            Expression::Export(Box::new(Export {
                expression: Expression::VariableDeclaration(Box::new(VariableDeclaration {
                    mutability: VariableMutability::Immutable,
                    name: "hello".into(),
                    type_annotation: None
                }))
            }))
        )
    }
}
