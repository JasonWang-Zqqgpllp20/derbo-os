#![allow(non_snake_case)]

use super::BoundNode::BoundNode;
use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::BoundBinaryOperator::BoundBinaryOperator;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundBinaryExpression {
    pub Left: BoundNode,
    pub Op: BoundBinaryOperator,
    pub Right: BoundNode,
}

impl BoundBinaryExpression {
    pub fn new(left: BoundNode, op: BoundBinaryOperator, right: BoundNode) -> BoundBinaryExpression {
        BoundBinaryExpression {
            Left: left,
            Op: op,
            Right: right,
        }
    }

    pub fn Type(&self) -> TypeSymbol {
        self.Op.Type.clone()
    }
}