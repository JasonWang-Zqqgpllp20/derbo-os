use alloc::string::String;

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Int32(i32),
    Bool(bool),
    String(String),
    Null,
}