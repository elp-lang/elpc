use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::parser::lexer_parser::Trie;

#[cfg(target_os = "macos")]
pub mod apple;

#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "windows")]
pub mod windows;

pub async fn create_target_compiler(ast: &mut Trie) -> Result<&dyn Compiler, CompilationError> {
    #[cfg(target_os = "windows")]
    return windows::WindowsCompiler::new(ast).await;

    #[cfg(target_os = "android")]
    return Box::new(android::TargetCompiler);

    #[cfg(target_os = "macos")]
    return Box::new(apple::TargetCompiler);
}

#[derive(Debug)]
pub enum CompilationError {
    GenericError(String),
    PluginRegistrationError(String),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Compilation error: {}", self))
    }
}

impl Error for CompilationError {}

#[async_trait]
pub trait Compiler {
    // This function should turn the AST into an intermediate representation that will be
    // compiled into a binary in the future.
    async fn ast_to_app(&self) -> Result<(), CompilationError>;
}
