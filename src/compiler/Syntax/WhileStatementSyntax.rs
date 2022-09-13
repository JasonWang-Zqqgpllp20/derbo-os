#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;
use super::StatementSyntax::StatementSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStatementSyntax {
    WhileKeyword: SyntaxToken,
    pub Condition: SyntaxNode,
    pub Body: StatementSyntax,
}

impl WhileStatementSyntax {
    pub fn new(whileKeyword: SyntaxToken, condition: SyntaxNode, body: StatementSyntax) -> WhileStatementSyntax {
        WhileStatementSyntax {
            WhileKeyword: whileKeyword,
            Condition: condition,
            Body: body,
        }
    }
}