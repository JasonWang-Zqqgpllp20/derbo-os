#![allow(non_snake_case)]

use super::SyntaxNode::SyntaxNode;

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatementSyntax {
    pub Expression: SyntaxNode,
}

impl ExpressionStatementSyntax {
    pub fn new(expression: SyntaxNode) -> ExpressionStatementSyntax {
        ExpressionStatementSyntax {
            Expression: expression,
        }
    }
}