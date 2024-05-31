use crate::lexer::lexer::Lexer;
use crate::parserv2::Parser;
use crate::parserv2::visitors::string_literals::StringLiteralsVisitor;

mod lexer;
mod parserv2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("compilation started");
    let mut string_visitor = StringLiteralsVisitor::default();

    // let source = fs::read_to_string("./examples/kitchen-sink.velp")
    //     .expect("Couldn't open kitchen-sink.velp");
    let source: String = "\"Hello world\"".into();
    let mut lexer = Lexer::new(source);

    let tokens = lexer.consume_all_tokens();
    let mut parser = Parser::new(tokens);
    parser.register_visitor(Box::new(&mut string_visitor));

    let ast = parser.parse().await?;

    dbg!(ast);

    Ok(())
}
