use super::r#type::Types;

#[derive(PartialEq, Debug)]
pub struct VariableASTNode<'a> {
    r#type: &'a Types,
}

