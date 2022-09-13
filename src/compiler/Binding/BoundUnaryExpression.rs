#![allow(non_snake_case)]

use super::BoundUnaryOperator::BoundUnaryOperator;
use super::BoundNode::BoundNode;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundUnaryExpression {
    pub Op: BoundUnaryOperator,
    pub Operand: BoundNode,
}

impl BoundUnaryExpression {
    pub fn new(op: BoundUnaryOperator, operand: BoundNode) -> BoundUnaryExpression {
        BoundUnaryExpression {
            Op: op,
            Operand: operand,
        }
    }

    pub fn Type(&self) -> TypeSymbol {
        let oprand_type: TypeSymbol;

        match self.Operand.clone() {
            BoundNode::BoundAssignmentExpression(a) => oprand_type = a.Type(),
            BoundNode::BoundBinaryExpression(b) => oprand_type = b.Type(),
            BoundNode::BoundLiteralExpression(l) => oprand_type = l.Type(),
            BoundNode::BoundUnaryExpression(u) => oprand_type = u.Type(),
            BoundNode::BoundVariableExpression(v) => oprand_type = v.Type(),
            BoundNode::BoundErrorExpression(e) => oprand_type = e.Type(),
            BoundNode::BoundCallExpression(c) => oprand_type = c.Type(),
            BoundNode::BoundConversionExpression(c) => oprand_type = c.Type,
        }

        oprand_type
    }
}