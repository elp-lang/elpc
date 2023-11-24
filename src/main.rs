mod ast;

use ast::lexer::Lexer;
use std::fs;

fn main() {
    let source = fs::read_to_string("./examples/kitchen-sink.velp")
        .expect("Couldn't open kitchen-sink.velp");
    let mut lexer = Lexer::new(source);

    match lexer.consume_all_tokens() {
        Ok(tokens) => print!("TOKENS: \n{:#?}\n", tokens),
        Err(err) => panic!("{:#?}", err),
    }
}
