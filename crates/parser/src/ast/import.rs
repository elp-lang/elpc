use crate::parser::ElpParseError;

use super::{expression::Expression, FromPest, StringValue};

#[derive(Debug, PartialEq, Eq)]
pub struct Import<'a> {
    pub names: Vec<ImportName<'a>>,
    pub from: StringValue<'a>,
}

impl<'a> FromPest<'a> for Import<'a> {
    fn from_pest(
        pair: pest::iterators::Pair<'a, crate::parser::Rule>,
    ) -> Result<Expression<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized,
    {
        let mut pair = pair.into_inner();
        let name_pairs = pair.next();
        let module = pair.next();

        Ok(Expression::Import(Import {
            names: vec![],
            from: StringValue {
                value: module.unwrap().as_str(),
            },
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImportName<'a> {
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

impl<'a> ImportName<'a> {
    fn from_pest(
        pair: pest::iterators::Pair<'a, crate::parser::Rule>,
    ) -> Result<ImportName<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized,
    {
        let mut pair = pair.into_inner();

        let name = pair.next().unwrap().as_str();
        let alias = pair.next().map(|p| p.as_str());

        Ok(ImportName { name, alias })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            expression::Expression,
            import::{Import, ImportName},
            StringValue,
        },
        parse_module_source,
    };

    #[test]
    fn test_parse_module_source() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
        let module_source = "import {some, stuff as nonsense} from \"module\"";
        let ast = parse_module_source(module_source)?;

        assert_eq!(ast.expressions.len(), 2);
        assert_eq!(
            ast.expressions[0],
            Expression::Import(Import {
                names: vec![ImportName {
                    name: "some",
                    alias: None,
                }],
                from: StringValue { value: "module" }
            })
        );

        Ok(())
    }
}
