#![allow(non_snake_case)]

use super::BoundNode::BoundNode;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundExpressionStatement {
    pub Expression: BoundNode,
}

impl BoundExpressionStatement {
    pub fn new(expression: BoundNode) -> BoundExpressionStatement {
        BoundExpressionStatement {
            Expression: expression,
        }
    }
}