mod parser;
mod compiler;

use parser::{lexer::Lexer, lexer_parser::Parser};
use std::fs;

#[tokio::main]
async fn main() {
    print!("compilation started");
    let source = fs::read_to_string("./examples/kitchen-sink.velp")
        .expect("Couldn't open kitchen-sink.velp");
    let mut lexer = Lexer::new(source);

    let tokens = lexer.consume_all_tokens();
    let mut parser = Parser::new(tokens);

    let mut ast = parser.parse();

    let compiler = compiler::create_target_compiler(&mut ast);
    let application_itm = compiler.ast_to_app_itm().await?;
    let compilation_result = application_itm.compile_to_binary().await?;

    print!("BUILT!\n");
}
