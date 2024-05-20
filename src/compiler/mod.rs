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

pub fn create_target_compiler(ast: &mut Trie) -> &dyn Compiler {
    #[cfg(target_os = "windows")]
    return &windows::WindowsCompiler::new(ast);

    #[cfg(target_os = "android")]
    return &android::TargetCompiler;

    #[cfg(target_os = "macos")]
    return &apple::TargetCompiler;
}

#[derive(Debug)]
pub enum CompilationError {
    GenericError(String),
    PluginRegistrationError(String)
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Compilation error: {}", self))
    }
}

impl Error for CompilationError {}

pub trait AppITM {
    // Create a binary from an app intermediary representation.
    async fn compile_to_binary(&self) -> Result<(), CompilationError>;
}

#[async_trait]
pub trait CompilerPlugin {
    fn new() -> &dyn CompilerPlugin;
    async fn register(&self, compiler: &mut dyn Compiler) -> Option<CompilationError>;
    async fn run(&self, trie: &mut Trie) -> Result<&mut Trie, CompilationError>;
}

#[async_trait]
pub trait Compiler {
    async fn new(ast: &mut Trie) -> &dyn Compiler;

    // This function should turn the AST into an intermediate representation that will be
    // compiled into a binary in the future.
    async fn ast_to_app_itm(&self) -> Result<&mut dyn AppITM, CompilationError>;
    async fn add_plugin(&mut self, plugin: &dyn CompilerPlugin) -> Option<CompilationError>;
}