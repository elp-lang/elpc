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
        let mut lexer = Lexer::new_str("const x = 10");
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
                    token_type: TokenType::Ident("x".to_string()),
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
                    token_type: TokenType::Symbol(Symbol::SingleEqual),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 8,
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
                    token_type: TokenType::IntegerLiteral(10),
                    source: Source {
                        name: "".to_string(),
                        path: "".to_string(),
                        span: Span {
                            start: 10,
                            end: 11,
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
                            start: 12,
                            end: 12,
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
}
