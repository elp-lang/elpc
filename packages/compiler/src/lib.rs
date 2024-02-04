use parser;

pub trait CompilerTargetTriplet {
    fn from_system();
}

pub trait Compiler {
    fn compile(ast: parser::ast::Trie);
    fn link();
}
