use crate::span::Span;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    SOI,
    AccessModifier(AccessModifier),
    BooleanLiteral(bool),
    CommentBlock(String),
    CommentLine(String),
    Component(String),
    FloatLiteral(f64),
    Ident(String),
    Identifier(String),
    IntegerLiteral(i64),
    Keyword(Keyword),
    MacroCall(String),
    Nil,
    StringLiteral(String),
    Symbol(Symbol),
    WhiteSpace(WhiteSpace),
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WhiteSpace {
    CarriageReturn,
    NewLine,
    Other(String),
    Space,
    Tab,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    As,
    Break,
    Component,
    Const,
    Continue,
    Else,
    ElseIf,
    Enum,
    Export,
    False,
    Fn,
    For,
    From,
    If,
    Import,
    Interface,
    Match,
    Nil,
    Object,
    Private,
    Public,
    Return,
    True,
    Typeof,
    Valueof,
    Var,
    While,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    Ampersand,
    Arrow,
    At,
    BackSlash,
    BitwiseAnd,
    BitwiseAndAssign,
    BitwiseLeftShift,
    BitwiseOrAssign,
    BitwiseRightShift,
    BitwiseXorAssign,
    Caret,
    CloseBlock,
    CloseParen,
    Colon,
    Comma,
    Dollar,
    Dot,
    DoubleAmpersand,
    DoubleColon,
    DoubleEqual,
    DoublePipe,
    DoubleSpeechMark,
    Ellipsis,
    Exclamation,
    Greater,
    GreaterEqual,
    Hash,
    Hyphen,
    Less,
    LessEqual,
    Minus,
    MinusAssign,
    MinusMinus,
    Modulo,
    OpenBlock,
    OpenParen,
    Other(String),
    Percent,
    PercentAssign,
    Pipe,
    Plus,
    PlusAssign,
    PlusPlus,
    QuestionMark,
    Semicolon,
    SingleEqual,
    SingleSpeechMark,
    Slash,
    SlashAssign,
    Star,
    StarAssign,
    Tilde,
    Underscore,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub source: Source,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Source {
    pub name: String,
    pub path: String,
    pub span: Span,
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.name, self.path, self.span)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_display() {
        let source = Source {
            name: "test".into(),
            ..Default::default()
        };

        assert_eq!("test::[]:0-0", format!("{}", source));
    }
}
