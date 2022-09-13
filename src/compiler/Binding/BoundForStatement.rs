#![allow(non_snake_case)]

use super::BoundNode::BoundNode;
use super::BoundStatement::BoundStatement;
use super::super::Symbol::VariableSymbol::VariableSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundForStatement {
    pub Variable: VariableSymbol,
    pub LowerBound: BoundNode,
    pub UpperBound: BoundNode,
    pub Body: BoundStatement,
}

impl BoundForStatement {
    pub fn new(variable: VariableSymbol, lowerBound: BoundNode, upperBound: BoundNode, body: BoundStatement) -> BoundForStatement {
        BoundForStatement {
            Variable: variable,
            LowerBound: lowerBound,
            UpperBound: upperBound,
            Body: body,
        }
    }
}