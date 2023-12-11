mod ast;

use ast::lexer::Lexer;
use ast::lexer_parser::Parser;
use std::fs;

fn main() {
    print!("compilation started");
    let source = fs::read_to_string("./examples/kitchen-sink.velp")
        .expect("Couldn't open kitchen-sink.velp");
    let mut lexer = Lexer::new(source);

    let tokens = lexer.consume_all_tokens();
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();
    print!("ast: {:#?}", ast);
}
