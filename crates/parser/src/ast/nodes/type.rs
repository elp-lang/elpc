use std::marker;

#[derive(PartialEq, Debug)]
pub enum IntrinsicTypes {
    Bool,
    String,
    Int,
    Float,
    Char,
    Byte,
    Function,
    Interface,
    Nil,
    InvalidUnknown,
}

#[derive(PartialEq, Debug)]
pub enum Types {
    Intrinsic(IntrinsicTypes),
    User(String),
}
/// A type alias is a type hint to the compiler that supports
/// generic parameters.
///
/// @example Optional<String>
/// @example Tuple<Int, String>
/// @example Array<MyUnspecializedThing<MySpecialisedObject>>
#[derive(PartialEq, Debug)]
pub struct TypeAliasASTNode<'a> {
    r#type: Box<Types>,
    _marker: marker::PhantomData<&'a ()>,
}
