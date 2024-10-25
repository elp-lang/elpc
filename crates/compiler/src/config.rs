use elp_parser::ast::ASTNodeMember;

pub struct CompilerConfig<'a> {
    manifest: Option<String>,
    enabled_modules: Vec<&'a dyn ASTNodeMember<'a>>,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            manifest: None,
            enabled_modules: vec![],
        }
    }
}
