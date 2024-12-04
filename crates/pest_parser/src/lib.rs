pub mod ast;
pub mod parser;

use ast::Program;
use parser::{ElpParser, Rule};
use pest::Parser;

pub fn parse_module_source(module_source: &str) -> Result<Program<'_>, Box<dyn std::error::Error>> {
    let mut parse_tree = ElpParser::parse(Rule::program, module_source)?;
    println!("PT: {:#?}", parse_tree);
    let ast: ast::Program = ast::Program::from_pest(&mut parse_tree.next().unwrap().into_inner())?;
    println!("AST: {:#?}", ast);

    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::parse_module_source;

    #[test]
    fn test_parse_module_source() {
        let module_source = "import {some, stuff} from \"module\"";
        let ast = parse_module_source(module_source);

        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(ast.unwrap().expressions.len(), 1);
    }
}
