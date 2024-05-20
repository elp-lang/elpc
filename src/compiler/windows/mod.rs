use crate::compiler::{AppITM, CompilationError, Compiler, CompilerPlugin};
use crate::parser::lexer_parser::Trie;

pub struct WindowsITM {

}

impl AppITM for WindowsITM {
    async fn compile_to_binary(&self) -> Result<(), CompilationError> {
        todo!()
    }
}

pub struct WindowsCompiler<'a> {
    ast: &'a mut Trie,
    plugins: Vec<Box<dyn CompilerPlugin>>,
}

impl<'a> Compiler for WindowsCompiler {
    async fn new(ast: &mut Trie) -> &dyn Compiler {
        &Self {
            ast,
            plugins: vec![],
        }
    }

    async fn ast_to_app_itm(&self) -> Result<&mut dyn AppITM, CompilationError> {
        let mut itm = WindowsITM{};
    }

    async fn add_plugin(&mut self, plugin: &dyn CompilerPlugin) -> Option<CompilationError> {
        plugin.register(&mut self)
    }
}