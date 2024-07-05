use std::env;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("elp.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("elp.y");

fn main() {
    // Get the `LexerDef` for the `calc` language.
    let lexerdef = calc_l::lexerdef();
    let args: Vec<String> = env::args().collect();
    // Now we create a lexer with the `lexer` method with which we can lex an
    // input.
    let lexer = lexerdef.lexer(&args[1]);
    // Pass the lexer to the parser and lex and parse the input.
    let (res, errs) = calc_y::parse(&lexer);
    for e in errs {
        println!("{}", e.pp(&lexer, &calc_y::token_epp));
    }
    match res {
        Some(r) => println!("Result: {:?}", r),
        _ => eprintln!("Unable to evaluate expression."),
    }
}

