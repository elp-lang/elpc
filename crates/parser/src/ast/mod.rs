pub(crate) mod expression;
pub(crate) mod import;

use expression::Expression;

use crate::parser::{ElpParseError, Rule};

pub trait FromPest<'a> {
    fn from_pest(
        pair: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Expression<'a>, ElpParseError<'a>>
    where
        Self: std::marker::Sized;
}

#[derive(Debug)]
pub struct Program<'a> {
    pub expressions: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StringValue<'a> {
    pub value: &'a str,
}
