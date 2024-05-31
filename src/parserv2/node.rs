use crate::lexer::lexer;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Import(ImportStatement),
    InterfaceDeclaration(InterfaceDeclaration),
    EnumDeclaration(EnumDeclaration),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(Fn),
    LiteralNumber(Literal),
    LiteralFloat(Literal),
    LiteralString(Literal),
    Expression(Expression),
    IfStatement(Expression, Option<Block>, Option<IfStatement>),
    MatchStatement(Expression, Vec<MatchCase>),
}

unsafe impl Send for AstNode {}
unsafe impl Sync for AstNode {}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub ident: Identifier,
    pub r#type: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Trie {
    pub nodes: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub struct Fn {
    pub name: Option<Identifier>,
    pub is_call: bool,
    pub is_callable: bool,
    pub params: Vec<Parameter>,
    pub returns: Box<Type>,
    pub block: Option<Box<Block>>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ImportStatementMember {
    pub name: Identifier,
    pub alias: Option<String>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ImportStatement {
    pub members: Vec<ImportStatementMember>,
    pub source_path: String,
}

#[derive(Default, Debug, PartialEq)]
pub struct ObjectDeclaration {
    pub name: Identifier,
    pub implements: Vec<*const InterfaceDeclaration>,
    pub members: Vec<*const InterfaceProperty>,
}

#[derive(Default, Debug, PartialEq)]
pub struct InterfaceDeclaration {
    pub name: Identifier,
    pub members: Vec<InterfaceProperty>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceProperty {
    pub name: Identifier,
    pub r#type: Type,
}

#[derive(Default, Debug, PartialEq)]
pub struct EnumVariant {
    pub name: Identifier,
    pub r#type: Option<EnumVariantType>,
}

#[derive(Debug, PartialEq)]
pub struct EnumDeclaration {
    pub name: Option<Identifier>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Default, Debug, PartialEq)]
pub enum EnumVariantType {
    #[default]
    Option,
    Action(Vec<Parameter>),
}

#[derive(Default, Debug, PartialEq)]
pub struct Parameter {
    pub position: i32,
    pub name: Option<Identifier>,
    pub value: Option<Expression>,
    pub r#type: Type,
}

#[derive(Default, Debug, PartialEq)]
pub enum Type {
    TypeName(Identifier),
    FnType(Fn),
    Literal(Literal),
    InterfaceType(InterfaceDeclaration),
    ObjectType(ObjectDeclaration),
    EnumType(EnumDeclaration),
    Undefined,
    #[default]
    Void,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Interface(InterfaceDeclaration),
    Literal(Literal),
    Function(Fn),
    IfStatement(Box<IfStatement>),
    Block(Vec<Expression>),
    VariableDeclaration(Box<VariableDeclaration>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: Option<Identifier>,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Option<Block>,
    pub branches: Option<Vec<Box<AstNode>>>,
}

#[derive(Debug, PartialEq)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub body: AstNode,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    MemberAccess(Identifier),
    Boolean(bool),
}

#[derive(Default, Debug, PartialEq)]
pub struct Identifier {
    pub immutable: bool,
    pub access_modifier: lexer::AccessModifier,
    pub name: String,
}
