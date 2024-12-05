use from_pest::ConversionError;

use crate::parser::Rule;

pub trait FromPest<'a> {
    fn from_pest(
        parse_tree: pest::iterators::Pair<'a, Rule>,
    ) -> Result<Self, ConversionError<Rule>>
    where
        Self: std::marker::Sized;
}

#[derive(Debug)]
pub struct Program<'a> {
    pub expressions: Vec<Expression<'a>>,
}

impl<'a> FromPest<'a> for Program<'a> {
    fn from_pest(parse_tree: pest::iterators::Pair<'a, Rule>) -> Result<Self, ConversionError<Rule>>
    where
        Self: std::marker::Sized,
    {
        let mut expressions = Vec::new();
        for expr in parse_tree.into_inner() {
            expressions.push(Expression::from_pest(expr)?);
        }
        Ok(Program { expressions })
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Import(Import<'a>),
}

impl<'a> FromPest<'a> for Expression<'a> {
    fn from_pest(parse_tree: pest::iterators::Pair<'a, Rule>) -> Result<Self, ConversionError<Rule>>
    where
        Self: std::marker::Sized,
    {
        match parse_tree.as_rule() {
            Rule::import => {
                let mut names = Vec::new();
                for name in parse_tree.into_inner() {
                    let name = name.as_str();
                    let alias = name.split_once(" as ").map(|(name, alias)| alias.trim());
                    names.push(ImportName { name, alias });
                }
                let from = StringValue {
                    value: parse_tree.as_str(),
                };
                Ok(Expression::Import(Import { names, from }))
            }
            _ => Err(ConversionError::NoMatch),
        }
    }
}

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

#[derive(Debug)]
pub struct StringValue<'a> {
    pub value: &'a str,
}
