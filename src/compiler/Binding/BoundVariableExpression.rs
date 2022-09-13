#![allow(non_snake_case)]

use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundVariableExpression {
    pub Variable: VariableSymbol,
}

impl BoundVariableExpression {
    pub fn new(variable: VariableSymbol) -> BoundVariableExpression {
        BoundVariableExpression {
            Variable: variable,
        }
    }

    pub fn Type(&self) -> TypeSymbol {
        self.Variable.Type.clone()
    }
}