#[cfg(test)]
mod tests {
    use elp_parser::{
        lexer::Lexer,
        span::Span,
        tokens::{Keyword, Source, Symbol, Token, TokenType, WhiteSpace},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_var() {
        let mut lexer = Lexer::new_str("var x = 10");
        let tokens = lexer.consume_all_tokens();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 0,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Var),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 2,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 3,
                            end: 3,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Ident("x".to_string()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 4,
                            end: 4,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 5,
                            end: 5,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 6,
                            end: 6,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 7,
                            end: 7,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(10),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 8,
                            end: 9,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 10,
                            end: 10,
                            lines: vec![],
                        },
                    }
                }
            ]
        );
    }

    #[test]
    fn test_const() {
        let mut lexer = Lexer::new_str("const x_1 = 10");
        let tokens = lexer.consume_all_tokens();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 0,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Const),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 4,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 5,
                            end: 5,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Ident("x_1".to_string()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 6,
                            end: 8,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 9,
                            end: 9,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 10,
                            end: 10,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 11,
                            end: 11,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::IntegerLiteral(10),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 12,
                            end: 13,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 14,
                            end: 14,
                            lines: vec![],
                        },
                    }
                }
            ]
        );
    }

    #[test]
    fn test_interface() {
        let mut lexer = Lexer::new_str(
            "interface testing {
    test string
}",
        );
        let tokens = lexer.consume_all_tokens();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 0,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Interface),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 8,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 9,
                            end: 9,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::Ident("testing".into()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 10,
                            end: 16,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 17,
                            end: 17,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 18,
                            end: 18,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 19,
                            end: 19,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 20,
                            end: 23,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::Ident("test".into()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 24,
                            end: 27,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 28,
                            end: 28,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::Ident("string".into()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 29,
                            end: 34,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 35,
                            end: 35,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 36,
                            end: 36,
                            lines: vec![],
                        },
                    },
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 37,
                            end: 37,
                            lines: vec![],
                        },
                    },
                },
            ]
        )
    }

    #[test]
    fn test_functions() {
        let mut lexer = Lexer::new_str(
            "fn testFunction -> bool {
    true
}",
        );
        let tokens = lexer.consume_all_tokens();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::SOI,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 0,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Keyword(Keyword::Fn),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 0,
                            end: 1,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 2,
                            end: 2,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Ident("testFunction".to_string()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 3,
                            end: 14,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 15,
                            end: 15,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::Arrow),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 16,
                            end: 17,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 18,
                            end: 18,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Ident("bool".into()),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 19,
                            end: 22,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Space),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 23,
                            end: 23,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::OpenBlock),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 24,
                            end: 24,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 25,
                            end: 25,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::Other("    ".into())),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 26,
                            end: 29,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::BooleanLiteral(true),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 30,
                            end: 33,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::WhiteSpace(WhiteSpace::NewLine),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 34,
                            end: 34,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::Symbol(Symbol::CloseBlock),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 35,
                            end: 35,
                            lines: vec![],
                        },
                    }
                },
                Token {
                    token_type: TokenType::EOF,
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 36,
                            end: 36,
                            lines: vec![],
                        },
                    }
                }
            ]
        );
    }
}
