pub mod ast;
pub mod parser;

use ast::{FromPest, Program};
use parser::{ElpParser, Rule};
use pest::Parser;

pub fn parse_module_source<'a>(
    module_source: &'a str,
) -> Result<Program<'a>, Box<dyn std::error::Error + 'a>> {
    let parse_tree = ElpParser::parse(Rule::program, module_source)?;
    println!("PT: {:#?}", parse_tree);

    let mut expressions: Vec<ast::expression::Expression> = Vec::new();

    for pair in parse_tree {
        expressions.push(ast::expression::Expression::from_pest(pair)?);
    }

    let ast = Program { expressions };
    println!("AST: {:#?}", ast);

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::parse_module_source;

    #[test]
    fn test_parse_module_source() {
        let module_source = "import {some, stuff as nonsense} from \"module\"";
        let ast = parse_module_source(module_source);

        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(ast.unwrap().expressions.len(), 1);
    }
}
