#![allow(non_snake_case)]

use super::BoundNode::BoundNode;
use super::BoundStatement::BoundStatement;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundIfStatement {
    pub Condition: BoundNode,
    pub ThenStatement: BoundStatement,
    pub ElseStatement: Option<BoundStatement>,
}

impl BoundIfStatement {
    pub fn new(condition: BoundNode, thenStatement: BoundStatement, elseStatement: Option<BoundStatement>) -> BoundIfStatement {
        BoundIfStatement {
            Condition: condition,
            ThenStatement: thenStatement,
            ElseStatement: elseStatement,
        }
    }
}