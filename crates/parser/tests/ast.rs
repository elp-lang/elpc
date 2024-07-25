#[cfg(test)]
mod tests {
    use elp_parser::{
        ast::{ASTNode, ASTTree},
        lexer::{token_stream::TokenStream, Lexer},
        parsing_error::ParsingError,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ast_tree() -> Result<(), ParsingError> {
        let mut lexer = Lexer::new_str("public var myVar = 1");
        let tokens = lexer.consume_all_tokens()?;
        let mut token_stream = TokenStream::new(tokens);
        let mut ast_tree = ASTTree::new(&token_stream);

        let tree = ast_tree.parse_tokens()?;

        assert_eq!(tree.get_children(), vec![]);

        Ok(())
    }
}
