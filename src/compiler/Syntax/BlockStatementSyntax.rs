#![allow(non_snake_case)]

use alloc::vec::Vec;

use super::SyntaxToken::SyntaxToken;
use super::StatementSyntax::StatementSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatementSyntax {
    OpenBraceToken: SyntaxToken,
    pub Statements: Vec<StatementSyntax>,
    CloseBraceToken: SyntaxToken,
}

impl BlockStatementSyntax {
    pub fn new(openBraceToken: SyntaxToken, statements: Vec<StatementSyntax>, closeBraceToken: SyntaxToken) -> BlockStatementSyntax {
        BlockStatementSyntax {
            OpenBraceToken: openBraceToken,
            Statements: statements,
            CloseBraceToken: closeBraceToken,
        }
    }
}