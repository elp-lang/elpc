pub mod ast;
pub mod parser;

use parser::{ElpParser, Rule};
use pest::Parser;

pub fn parse_module_source(module_source: String) -> Result<(), Box<dyn std::error::Error>> {
    let pairs = ElpParser::parse(Rule::program, module_source.as_str())?;

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        /*for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!(),
            };
        }*/
    }

    Ok(())
}
