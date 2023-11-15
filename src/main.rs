use pest::{self, Parser};

use crate::ast::{Node, Operator};

#[derive(Parser)]
#[grammar = "./grammar.pest"] // relative to src
struct ElpParser;

fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn main() {
    
}
