#![allow(non_snake_case)]

use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundErrorExpression {

}

impl BoundErrorExpression {
    pub fn new() -> BoundErrorExpression {
        BoundErrorExpression {}
    }

    pub fn Type(&self) -> TypeSymbol {
        TypeSymbol::Error
    }
}