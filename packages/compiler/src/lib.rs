use parser::{self, ast};

pub trait CompilerTargetTriplet {
    fn from_system();
}

pub trait Compiler {
    fn compile(ast: ast::Trie);
    fn link();
}
