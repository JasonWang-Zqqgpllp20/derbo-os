#![allow(non_snake_case)]

use super::BoundNode::BoundNode;
use super::BoundStatement::BoundStatement;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundWhileStatement {
    pub Condition: BoundNode,
    pub Body: BoundStatement,
}

impl BoundWhileStatement {
    pub fn new(condition: BoundNode, body: BoundStatement) -> BoundWhileStatement {
        BoundWhileStatement {
            Condition: condition,
            Body: body,
        }
    }
}