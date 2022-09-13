#[warn(unused_imports)]

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSymbol {
    Int,
    Bool,
    String,
    Error,
    Void,
}