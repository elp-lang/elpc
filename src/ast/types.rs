pub enum ExpressionKind {
    TypeName(str),
    Ident(str),
    IdentType(str, TypeName(str)),
    Function(str),
    Export(),
    Array(),
}

pub struct Expression {
    pub kind: ExpressionKind,
}
