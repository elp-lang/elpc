use crate::ast;
use pest::{
    error::{Error, LineColLocation},
    iterators::Pairs,
    Lines, Parser,
};
use pest_derive::Parser;

use std::result::Result as StdResult;

#[derive(Parser)]
#[grammar = "./elp.pest"]
struct ElpParser;

pub fn parse(source: String) -> StdResult<Vec<ast::nodes::Expression>, Error<Rule>> {
    let parse_result: Pairs<Rule>;

    match ElpParser::parse(Rule::Program, &source) {
        Ok(parser) => parse_result = parser,
        Err(e) => print_syntax_error_and_exit(e, source.clone()),
    }

    parse_result
        .into_iter()
        .filter_map(|pair| {
            if let Rule::Expression = pair.as_rule() {
                Some(build_ast_from_expr(pair))
            } else {
                None
            }
        })
        .collect()
}

fn print_syntax_error_and_exit(error: Error<Rule>, source: String) -> ! {
    let (line_num, col_num) = match error.line_col {
        LineColLocation::Pos(x) => (x.0, x.1),
        LineColLocation::Span(start, end) => (start.0, end.0),
    };
    let source_lines: Vec<&str>;
    let mut offset_line = false;
    if source.lines().count() > 30 {
        source_lines = source
            .lines()
            .skip(line_num - 15)
            .take(line_num + 15)
            .collect();
        offset_line = true;
    } else {
        source_lines = source.lines().collect();
    }

    for (line_number, line) in source_lines.iter().enumerate() {
        println!("{}: {}", line_number + 1, line);
        if line_number == line_num - 1 {
            let repeated_string: String = std::iter::repeat("-").take(col_num + 3).collect();
            println!("{}^", repeated_string);
        }
    }
    panic!();
}

fn build_ast_from_expr(
    pair: pest::iterators::Pair<Rule>,
) -> StdResult<ast::nodes::Expression, Error<Rule>> {
    match pair.as_rule() {
        Rule::Expression => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::Ident => build_ast_from_ident(pair.into_inner().next().unwrap()),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_ident(
    pair: pest::iterators::Pair<Rule>,
) -> StdResult<ast::nodes::Expression, Error<Rule>> {
    let ident = ast::nodes::Ident {
        name: pair.as_str().to_string(),
    };
    return Ok(ast::nodes::Expression {
        kind: ast::nodes::ExpressionKind::Ident(Box::<ast::nodes::Ident>::new(ident)),
    });
}
