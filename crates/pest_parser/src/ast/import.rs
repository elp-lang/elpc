use crate::parser::ElpParseError;

use super::{expression::Expression, FromPest, StringValue};

#[derive(Debug)]
pub struct Import<'a> {
    pub names: Vec<ImportName<'a>>,
    pub from: StringValue<'a>,
}

#[derive(Debug)]
pub struct ImportName<'a> {
    pub name: &'a str,
    pub alias: Option<&'a str>,
}

impl<'a> FromPest<'a> for Import<'a> {
    fn from_pest(
        pair: pest::iterators::Pair<'a, crate::parser::Rule>,
    ) -> Result<Expression<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized,
    {
        let mut pair = pair.into_inner();
        // let names = pair.next();
        let module = pair.next();

        Ok(Expression::Import(Import {
            names: vec![],
            from: StringValue {
                value: module.unwrap().as_str(),
            },
        }))
    }
}
