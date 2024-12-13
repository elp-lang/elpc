use super::variable::VariableDeclaration;
use crate::parser::Rule;
use pest_ast::FromPest;

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::variable_assignment_target))]
pub enum VariableAssignmentTarget {
    VariableDeclaration(Box<VariableDeclaration>),
}

#[derive(Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::variable_assignment))]
pub struct VariableAssignment {
    pub variable_declaration: VariableDeclaration,
}
