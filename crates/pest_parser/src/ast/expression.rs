use crate::parser::{ElpParseError, Rule};

use super::{import::Import, FromPest};

#[derive(Debug)]
pub(crate) enum Expression<'a> {
    Import(Import<'a>),
}

impl<'a> FromPest<'a> for Expression<'a> {
    fn from_pest(pair: pest::iterators::Pair<'a, Rule>) -> Result<Expression<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized,
    {
        match pair.as_rule() {
            Rule::import => Ok(Import::from_pest(pair).unwrap()),
            _ => Err(ElpParseError::Unknown),
        }
    }
}
