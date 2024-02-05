use compiler::Compiler;
use parser::ast;

pub mod platform_build;
pub struct AppleElpCompiler {}

impl Compiler for AppleElpCompiler {
    fn compile(ast: ast::Trie) {
        todo!()
    }

    fn link() {
        todo!()
    }
}
