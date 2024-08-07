#[cfg(test)]
mod tests {
    use elp_parser::{lexer::Lexer, parsing_error::ParsingError};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ast_tree() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("public var myVar = 1");
        let tokens = lexer.consume_all_tokens()?;

        Ok(())
    }
}
