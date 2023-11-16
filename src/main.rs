mod ast;

use ast::parser::parse;
use std::fs;

fn main() {
    let _ = parse(
        fs::read_to_string("./examples/HelloWorld/Package.elp").expect("Couldn't open Package.elp"),
    );
}
