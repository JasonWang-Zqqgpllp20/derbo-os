#![allow(non_snake_case)]

use alloc::vec::Vec;

use super::BoundNode::BoundNode;
use super::super::Symbol::FunctionSymbol::FunctionSymbol;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundCallExpression {
    pub Function: FunctionSymbol,
    pub Arguments: Vec<BoundNode>,
}

impl BoundCallExpression {
    pub fn new(function: FunctionSymbol, arguments: Vec<BoundNode>) -> BoundCallExpression {
        BoundCallExpression {
            Function: function,
            Arguments: arguments,
        }
    }

    pub fn Type(&self) -> TypeSymbol {
        self.Function.Type.clone()
    }
}