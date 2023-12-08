mod ast;

use ast::lexer::Lexer;
use std::fs;

fn main() {
    print!("compilation started");
    let source = fs::read_to_string("./examples/kitchen-sink.velp")
        .expect("Couldn't open kitchen-sink.velp");
    let mut lexer = Lexer::new(source);

    let tokens = lexer.consume_all_tokens();
    print!("TOKENS: \n{:#?}\n", tokens);
}
