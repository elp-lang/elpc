pub enum ExpressionKind {
    TypeName(Box<str>),
    Ident(Box<Ident>),
    IdentType(Box<str>),
    Function(Box<str>),
    Export(),
    Array(),
}

pub struct Expression {
    pub kind: ExpressionKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
}
