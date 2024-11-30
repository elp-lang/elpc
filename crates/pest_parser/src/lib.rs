pub mod ast;
pub mod parser;

use ast::Program;
use from_pest::FromPest;
use parser::{ElpParser, Rule};
use pest::Parser;

pub fn parse_module_source(module_source: &str) -> Result<Program<'_>, Box<dyn std::error::Error>> {
    let mut parse_tree = ElpParser::parse(Rule::program, module_source)?;

    let ast = ast::Program::from_pest(&mut parse_tree)?;

    println!("Parsed AST: {:?}", ast);

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::parse_module_source;

    #[test]
    fn test_parse_module_source() {
        let module_source = "";

        let ast = parse_module_source(module_source).unwrap();

        assert_eq!(ast.public_variables.len(), 1);
    }
}
