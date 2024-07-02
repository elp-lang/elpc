use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

lrlex_mod!("elp.l");
lrpar_mod!("elp.y");

fn main() {
    let lexerdef = elp_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                // Now we create a lexer with the `lexer` method with which
                // we can lex an input.
                let lexer = lexerdef.lexer(l);
                // Pass the lexer to the parser and lex and parse the input.
                let (res, errs) = elp_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &elp_y::token_epp));
                }
                match res {
                    Some(Ok(r)) => println!("Result: {:?}", r),
                    _ => eprintln!("Unable to evaluate expression."),
                }
            }
            _ => break,
        }
    }
}
