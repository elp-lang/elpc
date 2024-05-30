use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::compiler::{CompilationError, Compiler};
use crate::parser::lexer_parser::Trie;

pub struct WindowsITM {}

impl WindowsITM {
    async fn compile_to_binary(&self) -> Result<(), CompilationError> {
        todo!()
    }
}

pub struct WindowsCompiler<'a> {
    ast: Arc<Mutex<&'a mut Trie>>,
}

unsafe impl<'a> Send for WindowsCompiler<'a> {}
unsafe impl<'a> Sync for WindowsCompiler<'a> {}

impl<'a> WindowsCompiler<'a> {
    pub(crate) async fn new(ast: &'a mut Trie) -> Result<Self, CompilationError> {
        Ok(Self {
            ast: Arc::new(Mutex::new(ast)),
        })
    }
}

#[async_trait]
impl<'a> Compiler for WindowsCompiler<'_> {
    async fn ast_to_app(&self) -> Result<(), CompilationError> {
        let mut itm = Box::new(WindowsITM {});

        Ok(())
    }
}
