mod parser;

use std::fs;

fn main() {
    parser::parse(
        fs::read_to_string("./examples/HelloWorld/Package.elp").expect("Couldn't open Package.elp"),
    );
}
