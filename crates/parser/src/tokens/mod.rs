use crate::span::Span;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    SOI,
    Keyword(Keyword),
    Ident(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    Identifier(String),
    Symbol(Symbol),
    MacroCall(String),
    WhiteSpace(WhiteSpace),
    AccessModifier(AccessModifier),
    Nil,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WhiteSpace {
    Tab,
    Space,
    NewLine,
    CarriageReturn,
    Other(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    ElseIf,
    While,
    For,
    Return,
    Import,
    From,
    Export,
    Interface,
    Enum,
    Object,
    Match,
    Component,
    Fn,
    Var,
    Const,
    True,
    False,
    Nil,
    Continue,
    Break,
    As,
    Typeof,
    Valueof,
    Public,
    Private,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    Dot,
    Comma,
    Colon,
    Arrow,
    Semicolon,
    DoubleColon,
    OpenParen,
    CloseParen,
    SingleSpeechMark,
    DoubleSpeechMark,
    OpenBlock,
    CloseBlock,
    Plus,
    Minus,
    Star,
    Slash,
    BackSlash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Exclamation,
    Less,
    Greater,
    SingleEqual,
    DoubleEqual,
    LessEqual,
    GreaterEqual,
    PlusPlus,
    MinusMinus,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    BitwiseLeftShiftAssign,
    BitwiseRightShiftAssign,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    QuestionMark,
    At,
    Hash,
    Dollar,
    Underscore,
    PipeAssign,
    CaretAssign,
    TildeAssign,
    Ellipsis,
    Other(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub source: Source,
}

#[derive(Clone, Debug, PartialEq)]
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
