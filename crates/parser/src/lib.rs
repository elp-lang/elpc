pub mod ast;
pub mod parser;

use ast::{FromPest, Program};
use parser::{ElpParser, Rule};
use pest::Parser;

pub fn parse_module_source<'a>(
    module_source: &'a str,
) -> Result<Program<'a>, Box<dyn std::error::Error + 'a>> {
    let parse_tree = ElpParser::parse(Rule::program, module_source)?;

    let mut expressions: Vec<ast::expression::Expression> = Vec::new();

    for pair in parse_tree {
        expressions.push(ast::expression::Expression::from_pest(pair)?);
    }

    let ast = Program { expressions };

    Ok(ast)
}
