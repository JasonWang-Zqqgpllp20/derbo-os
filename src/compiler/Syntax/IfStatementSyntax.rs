#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;
use super::StatementSyntax::StatementSyntax;
// use super::ElseClauseSyntax::ElseClauseSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatementSyntax {
    IfKeyword: SyntaxToken,
    pub Condition: SyntaxNode,
    pub ThenStatement: StatementSyntax,
    pub ElseClause: Option<StatementSyntax>,        // ????
}

impl IfStatementSyntax {
    pub fn new(ifKeyword: SyntaxToken, condition: SyntaxNode, thenStatement: StatementSyntax,
                elseClause: Option<StatementSyntax>) -> IfStatementSyntax {
        IfStatementSyntax {
            IfKeyword: ifKeyword,
            Condition: condition,
            ThenStatement: thenStatement,
            ElseClause: elseClause,
        }
    }
}