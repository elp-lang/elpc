mod ast;

use ast::lexer::Lexer;
use std::fs;

fn main() {
    let source = fs::read_to_string("./examples/kitchen-sink.velp")
        .expect("Couldn't open kitchen-sink.velp");
    let tokens = Lexer::new(source);

    print!("{:?}", tokens);
}
