use crate::parser::{ElpParseError, Rule};

use super::{import::Import, FromPest};

#[derive(Debug)]
pub enum Expression<'a> {
    Import(Import<'a>),
    Eoi,
}

impl<'a> FromPest<'a> for Expression<'a> {
    fn from_pest(pair: pest::iterators::Pair<'a, Rule>) -> Result<Expression<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized,
    {
        match pair.as_rule() {
            Rule::import => Ok(Import::from_pest(pair).unwrap()),
            Rule::EOI => Ok(Expression::Eoi),
            _ => Err(ElpParseError::ExpectedButGot {
                msg: "unknown rule",
                expected: "Expression".into(),
                found: pair,
            }),
        }
    }
}
