#![allow(non_snake_case)]

use super::super::Syntax::ValueType::ValueType;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundLiteralExpression {
    pub Value: ValueType
}

impl BoundLiteralExpression {
    pub fn new(value: ValueType) -> BoundLiteralExpression {
        BoundLiteralExpression {
            Value: value
        }
    }

    pub fn Type(&self) -> TypeSymbol {
        match self.Value() {
            ValueType::Int32(_) => return TypeSymbol::Int,
            ValueType::Bool(_) => return TypeSymbol::Bool,
            ValueType::String(_) => return TypeSymbol::String,
            ValueType::Null => return TypeSymbol::Error,
        }
    }

    pub fn Value(&self) -> ValueType {
        self.Value.clone()
    }
}