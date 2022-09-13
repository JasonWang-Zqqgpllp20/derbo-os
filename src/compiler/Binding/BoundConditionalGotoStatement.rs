#![allow(non_snake_case)]

use super::BoundLabel::BoundLabel;
use super::BoundNode::BoundNode;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundConditionalGotoStatement {
    pub Label: BoundLabel,
    pub Condition: BoundNode,
    pub JumpIfFalse: bool,
}

impl BoundConditionalGotoStatement {
    pub fn new(label: BoundLabel, condition: BoundNode, jumpIfFalse: bool) -> BoundConditionalGotoStatement {
        BoundConditionalGotoStatement {
            Label: label,
            Condition: condition,
            JumpIfFalse: jumpIfFalse,
        }
    }
}