use pest::Parser;
use pest_derive::Parser;

use crate::ast::types::Expression;

#[derive(Parser)]
#[grammar = "./elp.pest"]
struct ElpParser;

pub fn parse(file: String) {
    let mut ast = vec![];
    let pairs = ElpParser::parse(Rule::Program, &file)
        .expect("Failed to parse file")
        .into_iter()
        .filter_map(|pair| {
            if let Rule::Expression = pair.as_rule() {
                Some(build_ast_from_expr(pair))
            } else {
                None
            }
        })
        .collect();
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Expression {
    match pair.as_rule() {
        // Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}
