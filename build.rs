use cfgrammar::yacc::YaccGrammar;
use lrtable::{from_yacc, statetable::StateTable, Minimiser, StateGraph};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let in_file = Path::new("elp_lang.y");
    let out_grm = Path::new(&env::var("OUT_DIR").unwrap()).join("elp_lang.grm");
    let out_sgraph = Path::new(&env::var("OUT_DIR").unwrap()).join("elp_lang.sgraph");
    let out_stable = Path::new(&env::var("OUT_DIR").unwrap()).join("elp_lang.stable");

    let grm = YaccGrammar::new_from_file(in_file).unwrap();
    let sgraph = StateGraph::from(&grm);
    let stable = StateTable::new(&sgraph, Minimiser::Pager);

    File::create(out_grm)
        .unwrap()
        .write_all(format!("{:?}", grm).as_bytes())
        .unwrap();
    File::create(out_sgraph)
        .unwrap()
        .write_all(format!("{:?}", sgraph).as_bytes())
        .unwrap();
    File::create(out_stable)
        .unwrap()
        .write_all(format!("{:?}", stable).as_bytes())
        .unwrap();
}
