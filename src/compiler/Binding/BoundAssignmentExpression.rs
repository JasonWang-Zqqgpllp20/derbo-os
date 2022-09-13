#![allow(non_snake_case)]
#[warn(unused_imports)]

use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::BoundNode::BoundNode;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundAssignmentExpression {
    pub Variable: VariableSymbol,
    pub Expression: BoundNode,
}

impl BoundAssignmentExpression {
    pub fn new(variable: VariableSymbol, expression: BoundNode) -> BoundAssignmentExpression {
        BoundAssignmentExpression {
            Variable: variable,
            Expression: expression,
        }
    }

    pub fn Type(&self) -> TypeSymbol {       // what kind of type? I mean, this is the same as Kind() or self.Expreesion
        self.Variable.Type.clone()
    }
}