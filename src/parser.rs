use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./elp.pest"] // relative to src
struct ElpParser;

pub fn parse(file: String) {
    let unit = ElpParser::parse(Rule::Program, &file)
        .expect("Failed to parse file")
        .next()
        .unwrap();

    print!("Unit {}", unit)
}
