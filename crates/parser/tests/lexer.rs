// This tests the very first pass of parsing; of which there are multiple,
// the lexer on it's own produces tokens for further parsing, the output is a
// very verbose output whereas usual parsing would skip things like whitespace.

#[cfg(test)]
mod tests {
    use elp_parser::{
        lexer::Lexer,
        parsing_error::ParsingError,
        span::Span,
        tokens::{Keyword, Source, Symbol, Token, TokenType, WhiteSpace},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_var() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("var x = 10");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        span: Span::default(),
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Var),
                    source: Source {
                        span: Span {
                            end: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 3,
                            end: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("x".to_string()),
                    source: Source {
                        span: Span {
                            start: 4,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 5,
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 7,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(10),
                    source: Source {
                        span: Span {
                            start: 8,
                            end: 9,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 10,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_const() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("const x_1 = 10");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Const),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 5,
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("x_1".to_string()),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 9,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 10,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 11,
                            end: 11,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(10),
                    source: Source {
                        span: Span {
                            start: 12,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_interface() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "interface testing {
    test string
}",
        );
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 0,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Interface),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 9,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("testing".into()),
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 18,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 19,
                            end: 19,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 20,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("test".into()),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 27,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 28,
                            end: 28,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("string".into()),
                    source: Source {
                        span: Span {
                            start: 29,
                            end: 34,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 35,
                            end: 35,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 36,
                            end: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 37,
                            end: 37,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_type_alias() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("type Results = int");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Type),
                    source: Source {
                        span: Span {
                            end: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 4,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("Results".into()),
                    source: Source {
                        span: Span {
                            start: 5,
                            end: 11,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 12,
                            end: 12,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        span: Span {
                            start: 13,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("int".into()),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 18,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_functions() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "fn testFunction -> bool {
    true
}",
        );
        let mut tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Fn),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 2,
                            end: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("testFunction".to_string()),
                    source: Source {
                        span: Span {
                            start: 3,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Arrow),
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 18,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("bool".into()),
                    source: Source {
                        span: Span {
                            start: 19,
                            end: 22,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 23,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 24,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 25,
                            end: 25,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 26,
                            end: 29,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::BooleanLiteral(true),
                    source: Source {
                        span: Span {
                            start: 30,
                            end: 33,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 34,
                            end: 34,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 35,
                            end: 35,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 36,
                            end: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str(
            "fn isEven(num int32) -> bool {
    num % 2 == 0
}",
        );
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Fn),
                    source: Source {
                        span: Span {
                            end: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 2,
                            end: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("isEven".to_string()),
                    source: Source {
                        span: Span {
                            start: 3,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenParen),
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 9,
                            lines: vec![]
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("num".to_string()),
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 12,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 13,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("int32".to_string()),
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 18,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseParen),
                    source: Source {
                        span: Span {
                            start: 19,
                            end: 19,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 20,
                            end: 20,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Arrow),
                    source: Source {
                        span: Span {
                            start: 21,
                            end: 22,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 23,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("bool".into()),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 27,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 28,
                            end: 28,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 29,
                            end: 29,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 30,
                            end: 30,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 31,
                            end: 34,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("num".into()),
                    source: Source {
                        span: Span {
                            start: 35,
                            end: 37,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 38,
                            end: 38,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Modulo),
                    source: Source {
                        span: Span {
                            start: 39,
                            end: 39,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 40,
                            end: 40,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(2),
                    source: Source {
                        span: Span {
                            start: 41,
                            end: 41,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 42,
                            end: 42,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::DoubleEqual),
                    source: Source {
                        span: Span {
                            start: 43,
                            end: 44,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 45,
                            end: 45,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(0),
                    source: Source {
                        span: Span {
                            start: 46,
                            end: 46,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 47,
                            end: 47,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 48,
                            end: 48,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 49,
                            end: 49,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("isEven(1)");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Ident("isEven".into()),
                    source: Source {
                        span: Span {
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenParen),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(1),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 7,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseParen),
                    source: Source {
                        span: Span {
                            start: 8,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 9,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_imports() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("import { Thing as Alias } from \"myThing\"");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Import),
                    source: Source {
                        span: Span {
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 7,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 8,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("Thing".into()),
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::As),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("Alias".into()),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 22,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 23,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 24,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 25,
                            end: 25,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::From),
                    source: Source {
                        span: Span {
                            start: 26,
                            end: 29,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 30,
                            end: 30,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::StringLiteral("myThing".into()),
                    source: Source {
                        span: Span {
                            start: 31,
                            end: 40,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 40,
                            end: 40,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_components() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "export component Nil {
    nil
}",
        );
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 0,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Export),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Component),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("Nil".into()),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 19,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 20,
                            end: 20,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 21,
                            end: 21,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 22,
                            end: 22,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 23,
                            end: 26,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Nil,
                    source: Source {
                        span: Span {
                            start: 27,
                            end: 29,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 30,
                            end: 30,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 31,
                            end: 31,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 32,
                            end: 32,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_objects() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("export object {}");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Export),
                    source: Source {
                        span: Span {
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Object),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 12,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 13,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_enums() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "enum HttpStatus {
    OKAY,
    ERROR
}",
        );
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Enum),
                    source: Source {
                        span: Span {
                            end: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 4,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("HttpStatus".into()),
                    source: Source {
                        span: Span {
                            start: 5,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 21,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("OKAY".into()),
                    source: Source {
                        span: Span {
                            start: 22,
                            end: 25,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Comma),
                    source: Source {
                        span: Span {
                            start: 26,
                            end: 26,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 27,
                            end: 27,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 28,
                            end: 31,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Ident("ERROR".into()),
                    source: Source {
                        span: Span {
                            start: 32,
                            end: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 37,
                            end: 37,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 38,
                            end: 38,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 39,
                            end: 39,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_macro() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("@myMacro");
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::MacroCall("myMacro".into()),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 7,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 8,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_match() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "match HttpStatus {
    .OKAY -> 200
    .ERROR -> 500
}",
        );
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default()
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Match),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 5,
                            end: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("HttpStatus".into()),
                    source: Source {
                        span: Span {
                            start: 6,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 18,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 19,
                            end: 22,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Dot),
                    source: Source {
                        span: Span {
                            start: 23,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("OKAY".into()),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 27,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 28,
                            end: 28,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Arrow),
                    source: Source {
                        span: Span {
                            start: 29,
                            end: 30,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 31,
                            end: 31,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(200),
                    source: Source {
                        span: Span {
                            start: 32,
                            end: 34,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 35,
                            end: 35,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 36,
                            end: 39,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Dot),
                    source: Source {
                        span: Span {
                            start: 40,
                            end: 40,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Ident("ERROR".into()),
                    source: Source {
                        span: Span {
                            start: 41,
                            end: 45,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 46,
                            end: 46,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Arrow),
                    source: Source {
                        span: Span {
                            start: 47,
                            end: 48,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 49,
                            end: 49,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(500),
                    source: Source {
                        span: Span {
                            start: 50,
                            end: 52,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 53,
                            end: 53,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 54,
                            end: 54,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 55,
                            end: 55,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_if_else_elseif() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str(
            "if true {
    1
} elseif false {
    2
} else 3",
        );
        let tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::If),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 2,
                            end: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::BooleanLiteral(true),
                    source: Source {
                        span: Span {
                            start: 3,
                            end: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 7,
                            end: 7,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 8,
                            end: 8,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 9,
                            end: 9,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::IntegerLiteral(1),
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 15,
                            end: 15,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::ElseIf),
                    source: Source {
                        span: Span {
                            start: 18,
                            end: 23,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 24,
                            end: 24,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::BooleanLiteral(false),
                    source: Source {
                        span: Span {
                            start: 25,
                            end: 29,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 30,
                            end: 30,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        span: Span {
                            start: 31,
                            end: 31,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 32,
                            end: 32,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        span: Span {
                            start: 33,
                            end: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::IntegerLiteral(2),
                    source: Source {
                        span: Span {
                            start: 37,
                            end: 37,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        span: Span {
                            start: 38,
                            end: 38,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        span: Span {
                            start: 39,
                            end: 39,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 40,
                            end: 40,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Else),
                    source: Source {
                        span: Span {
                            start: 41,
                            end: 44,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        span: Span {
                            start: 45,
                            end: 45,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::IntegerLiteral(3),
                    source: Source {
                        span: Span {
                            start: 46,
                            end: 46,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 47,
                            end: 47,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_symbols() -> Result<(), Box<ParsingError>> {
        struct Test<'a> {
            input: &'a str,
            expected: TokenType,
        }

        let tests: Vec<Test> = vec![
            Test {
                input: "/=",
                expected: TokenType::Symbol(Symbol::SlashAssign),
            },
            Test {
                input: "&=",
                expected: TokenType::Symbol(Symbol::BitwiseAndAssign),
            },
            Test {
                input: "^=",
                expected: TokenType::Symbol(Symbol::BitwiseXorAssign),
            },
            Test {
                input: "|=",
                expected: TokenType::Symbol(Symbol::BitwiseOrAssign),
            },
            Test {
                input: "<<",
                expected: TokenType::Symbol(Symbol::BitwiseLeftShift),
            },
            Test {
                input: ">>",
                expected: TokenType::Symbol(Symbol::BitwiseRightShift),
            },
            Test {
                input: "&&",
                expected: TokenType::Symbol(Symbol::DoubleAmpersand),
            },
            Test {
                input: "||",
                expected: TokenType::Symbol(Symbol::DoublePipe),
            },
            Test {
                input: "|",
                expected: TokenType::Symbol(Symbol::Pipe),
            },
            Test {
                input: "&",
                expected: TokenType::Symbol(Symbol::Ampersand),
            },
            Test {
                input: "^",
                expected: TokenType::Symbol(Symbol::Caret),
            },
            Test {
                input: "/",
                expected: TokenType::Symbol(Symbol::Slash),
            },
            Test {
                input: "<",
                expected: TokenType::Symbol(Symbol::Less),
            },
            Test {
                input: "<=",
                expected: TokenType::Symbol(Symbol::LessEqual),
            },
            Test {
                input: ">",
                expected: TokenType::Symbol(Symbol::Greater),
            },
            Test {
                input: ">=",
                expected: TokenType::Symbol(Symbol::GreaterEqual),
            },
            Test {
                input: "-",
                expected: TokenType::Symbol(Symbol::Hyphen),
            },
            Test {
                input: "->",
                expected: TokenType::Symbol(Symbol::Arrow),
            },
            Test {
                input: "\\",
                expected: TokenType::Symbol(Symbol::BackSlash),
            },
            Test {
                input: ":",
                expected: TokenType::Symbol(Symbol::Colon),
            },
            Test {
                input: "nil",
                expected: TokenType::Nil,
            },
        ];

        for test in tests {
            let mut lexer = Lexer::new_str(test.input);
            let tokens = lexer.consume_all_tokens()?;

            assert_eq!(
                tokens,
                vec![
                    Token {
                        token_type: TokenType::SOI,
                        source: Source::default(),
                    },
                    Token {
                        token_type: test.expected,
                        source: Source {
                            span: Span {
                                end: test.input.len() - 1,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    },
                    Token {
                        token_type: TokenType::EOF,
                        source: Source {
                            span: Span {
                                start: test.input.len(),
                                end: test.input.len(),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    },
                ]
            );
        }

        Ok(())
    }

    #[test]
    fn test_numerical_lexing() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("1");
        let mut tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::IntegerLiteral(1),
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 1,
                            end: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("1.0");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::FloatLiteral(1.0),
                    source: Source {
                        span: Span {
                            start: 0,
                            end: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 3,
                            end: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_strings() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("\"Hello \\\"world\\\"\"");
        let mut tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("Hello \"world\"".into()),
                    source: Source {
                        span: Span {
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 17,
                            end: 17,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("'Hello world'");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("Hello world".into()),
                    source: Source {
                        span: Span {
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 13,
                            end: 13,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("'Hello\\nworld'");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("Hello\nworld".into()),
                    source: Source {
                        span: Span {
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 14,
                            end: 14,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("'\\tHello\\nworld\\r\\n'");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("\tHello\nworld\r\n".into()),
                    source: Source {
                        span: Span {
                            end: 20,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 20,
                            end: 20,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("'Hello \\'${name}\\''");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("Hello \'${name}\'".into()),
                    source: Source {
                        span: Span {
                            end: 19,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 19,
                            end: 19,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        lexer = Lexer::new_str("'\\\\'");
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::StringLiteral("\\".into()),
                    source: Source {
                        span: Span {
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 4,
                            end: 4,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_comments() -> Result<(), Box<ParsingError>> {
        let mut lexer = Lexer::new_str("// Single");
        let mut tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::CommentLine(" Single".into()),
                    source: Source {
                        span: Span {
                            start: 1,
                            end: 10,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 10,
                            end: 10,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        lexer = Lexer::new_str(
            "/*
Multi
Line
*/",
        );
        tokens = lexer.consume_all_tokens()?;

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source::default(),
                },
                Token {
                    token_type: TokenType::CommentBlock("\nMulti\nLine\n".into()),
                    source: Source {
                        span: Span {
                            start: 1,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        span: Span {
                            start: 16,
                            end: 16,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                },
            ]
        );

        Ok(())
    }
}
