use super::{block::Block, elp_type::ElpType, expression::Expression, span_into_string, IDENT};
use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::SELF))]
pub struct SelfToken;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::function_argument))]
pub struct FunctionArgument {
    #[pest_ast(inner(with(span_into_string)))]
    pub name: String,

    pub type_annotation: Option<ElpType>,
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::function_arguments))]
pub struct FunctionArguments {
    pub has_self: Option<SelfToken>,
    pub arguments: Vec<FunctionArgument>,
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::function_return_type))]
pub struct FunctionReturnType {
    pub type_annotation: ElpType,
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::function_return_value))]
pub struct FunctionReturnValue {
    pub value: Box<Expression>,
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::function_def))]
pub struct FunctionDef {
    #[pest_ast(inner(with(span_into_string)))]
    pub name: String,
    pub members: Vec<IDENT>,
    pub arguments: FunctionArguments,
    pub returns: Option<FunctionReturnType>,
    pub block: Box<Block>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::StringValue, parser::ElpParser};
    use from_pest::FromPest;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple_function_def() {
        let expression_str = "fn hello(name String) -> String { return \"hello {name}\" }";
        let mut pairs = ElpParser::parse(Rule::function_def, expression_str).unwrap();
        let ast = FunctionDef::from_pest(&mut pairs).unwrap();

        assert_eq!(
            ast,
            FunctionDef {
                name: "hello".into(),
                members: vec![],
                arguments: FunctionArguments {
                    has_self: None,
                    arguments: vec![FunctionArgument {
                        name: "name".into(),
                        type_annotation: Some(ElpType {
                            name: "String".into(),
                            type_parameters: None,
                        }),
                    }],
                },
                returns: Some(FunctionReturnType {
                    type_annotation: ElpType {
                        name: "String".into(),
                        type_parameters: None,
                    },
                }),
                block: Box::new(Block {
                    expressions: vec![Expression::FunctionReturnValue(Box::new(
                        FunctionReturnValue {
                            value: Box::new(Expression::String(Box::new(StringValue {
                                value: "hello {name}".into()
                            })))
                        }
                    ))]
                })
            }
        )
    }
}
