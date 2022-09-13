#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::StatementSyntax::StatementSyntax;
use super::SyntaxNode::SyntaxNode;

#[derive(Clone, Debug, PartialEq)]
pub struct ForStatementSyntax {
    Keyword: SyntaxToken,
    pub Identifier: SyntaxToken,
    EqualsToken: SyntaxToken,
    pub LowerBound: SyntaxNode,
    ToKeyword: SyntaxToken,
    pub UpperBound: SyntaxNode,
    pub Body: StatementSyntax,
}

impl ForStatementSyntax {
    pub fn new(keyword: SyntaxToken, identifier: SyntaxToken, equalsToken: SyntaxToken, lowerBound: SyntaxNode,
                toKeyword: SyntaxToken, upperBound: SyntaxNode, body: StatementSyntax) -> ForStatementSyntax {
        ForStatementSyntax {
            Keyword: keyword,
            Identifier: identifier,
            EqualsToken: equalsToken,
            LowerBound: lowerBound,
            ToKeyword: toKeyword,
            UpperBound: upperBound,
            Body: body,
        }
    }
}