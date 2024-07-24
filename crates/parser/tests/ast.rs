#[cfg(test)]
mod tests {
    use elp_parser::{
        ast::ASTTree,
        lexer::{token_stream::TokenStream, Lexer},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ast_tree() {
        let lexer = Lexer::new_str("public var myVar = 1");
        let tokens = lexer.consume_all_tokens();
        let token_stream = TokenStream::new(tokens);
        let ast_tree = ASTTree::new(&token_stream);

        let tree = ast_tree.parse_tokens();

        assert_eq!(tree, nil);
    }
}
