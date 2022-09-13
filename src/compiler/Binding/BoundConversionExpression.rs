#![allow(non_snake_case)]

use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::BoundNode::BoundNode;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundConversionExpression {
    pub Type: TypeSymbol,
    pub Expression: BoundNode,
}

impl BoundConversionExpression {
    pub fn new(type_symbol: TypeSymbol, expression: BoundNode) -> BoundConversionExpression {
        BoundConversionExpression {
            Type: type_symbol,
            Expression: expression,
        }
    }
}