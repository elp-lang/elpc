pub mod utils;
use pest_ast::FromPest;
use utils::{span_into_str, span_into_str_option, span_into_vec_str};

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::program))]
pub struct Program<'a> {
    pub expressions: Vec<Expression<'a>>,
    _eoi: Eoi,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression<'a> {
    Import(Import<'a>),
    Number(Number<'a>),
    Block(Block<'a>),
    String(StringValue<'a>),
    Macro(Macro<'a>),
    Export(Box<Expression<'a>>),
    Enum(Enum<'a>),
    IfTree(IfTree<'a>),
    ObjectDef(ObjectDef<'a>),
    ComponentDef(ComponentDef<'a>),
    FunctionDef(FunctionDef<'a>),
    FunctionCall(FunctionCall<'a>),
    ObjectInstantiation(ObjectInstantiation<'a>),
    MatchTree(MatchTree<'a>),
    VariableAssignment(VariableAssignment<'a>),
    VariableAccess(VariableAccess<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    ObjectMethodAssignment(ObjectMethodAssignment<'a>),
    ContextualVariableAccess(ContextualVariableAccess<'a>),
    FunctionReturnValue(FunctionReturnValue<'a>),
    FunctionComponentCall(FunctionComponentCall<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import))]
pub struct Import<'a> {
    pub names: Vec<ImportName<'a>>,
    pub from: StringValue<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::import_name))]
pub struct ImportName<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,

    #[pest_ast(inner(with(span_into_str_option)))]
    pub alias: Option<&'a str>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct StringValue<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub value: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::number))]
pub struct Number<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub value: &'a str, // or any desired numeric type like f64 with conversion
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::block))]
pub struct Block<'a> {
    pub expressions: Vec<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::r#macro))]
pub struct Macro<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::r#enum))]
pub struct Enum<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub members: Vec<EnumMember<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::enum_member))]
pub struct EnumMember<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub types: Vec<ElpType<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::elp_type))]
pub struct ElpType<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub generics: Option<Vec<ElpType<'a>>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::if_tree))]
pub struct IfTree<'a> {
    pub condition: Box<Expression<'a>>,
    pub body: Block<'a>,
    pub elseif_branch: Option<Box<ElseIfTree<'a>>>,
    pub else_branch: Option<Box<ElseTree<'a>>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::elseif_tree))]
pub struct ElseIfTree<'a> {
    pub condition: Box<Expression<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::else_tree))]
pub struct ElseTree<'a> {
    pub body: Block<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_def))]
pub struct ObjectDef<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub implements: Option<Vec<ElpType<'a>>>,
    pub members: Vec<ObjectMember<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_member))]
pub struct ObjectMember<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub key_type: Option<ElpType<'a>>,
    pub default_value: Option<Box<Expression<'a>>>,
    pub tags: Option<Vec<Tag<'a>>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_member_visibility))]
pub struct ObjectMemberVisibility<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub visibility: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_key_tags))]
pub struct Tag<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    #[pest_ast(inner(with(span_into_str)))]
    pub value: &'a str, // The string portion.
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_method_assignment))]
pub struct ObjectMethodAssignment<'a> {
    pub target: VariableAccess<'a>,
    #[pest_ast(inner(with(span_into_str)))]
    pub method_name: &'a str,
    pub value: Box<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::component_def))]
pub struct ComponentDef<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub arguments: Option<Vec<FunctionArgument<'a>>>,
    pub return_type: Option<ElpType<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_def))]
pub struct FunctionDef<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub arguments: Option<Vec<FunctionArgument<'a>>>,
    pub return_type: Option<ElpType<'a>>,
    pub body: Block<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_component_call))]
pub struct FunctionComponentCall<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub arguments: Vec<Expression<'a>>,
    pub children: Option<Block<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_argument))]
pub struct FunctionArgument<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub argument_type: ElpType<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_call))]
pub struct FunctionCall<'a> {
    pub target: Box<Expression<'a>>,
    pub generics: Option<Vec<ElpType<'a>>>,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::variable_assignment))]
pub struct VariableAssignment<'a> {
    pub target: Box<Expression<'a>>,
    pub value: Box<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::variable_access))]
pub struct VariableAccess<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,

    #[pest_ast(inner(with(span_into_vec_str)))]
    pub qualifiers: Vec<&'a str>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::object_instantiation))]
pub struct ObjectInstantiation<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub generics: Option<Vec<ElpType<'a>>>,
    pub arguments: Vec<Expression<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_tree))]
pub struct MatchTree<'a> {
    pub value: Box<Expression<'a>>,
    pub arms: Vec<MatchArm<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_arm))]
pub struct MatchArm<'a> {
    pub pattern: MatchPattern<'a>,
    pub body: MatchBody<'a>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_range))]
pub struct MatchRange<'a> {
    pub ranges: Vec<MatchRangeItem<'a>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_rangeables))]
pub enum MatchRangeItem<'a> {
    StringLiteral(#[pest_ast(inner(with(span_into_str)))] &'a str),
    NumberLiteral(#[pest_ast(inner(with(span_into_str)))] &'a str),
    VariableAccess(VariableAccess<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_arm))]
pub enum MatchPattern<'a> {
    FunctionCall(FunctionCall<'a>),
    VariableAccess(VariableAccess<'a>),
    ContextualVariableAccess(ContextualVariableAccess<'a>),
    Range(MatchRange<'a>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::match_arm_body))]
pub enum MatchBody<'a> {
    Block(Block<'a>),
    Expression(Box<Expression<'a>>),
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::variable_declaration))]
pub struct VariableDeclaration<'a> {
    #[pest_ast(inner(with(span_into_str)))]
    pub name: &'a str,
    pub variable_type: Option<ElpType<'a>>,
    pub value: Option<Box<Expression<'a>>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::contextual_variable_access))]
pub struct ContextualVariableAccess<'a> {
    pub context: Box<Expression<'a>>,
    #[pest_ast(inner(with(span_into_str)))]
    pub variable_name: &'a str,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function_return_value))]
pub struct FunctionReturnValue<'a> {
    pub value: Option<Box<Expression<'a>>>,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;
