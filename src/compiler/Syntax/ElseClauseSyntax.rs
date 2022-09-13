#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::StatementSyntax::StatementSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct ElseClauseSyntax {
    ElseKeyword: SyntaxToken,
    pub ElseStatement: StatementSyntax,
}

impl ElseClauseSyntax {
    pub fn new(elseKeyword: SyntaxToken, elseStatement: StatementSyntax) -> ElseClauseSyntax {
        ElseClauseSyntax {
            ElseKeyword: elseKeyword,
            ElseStatement: elseStatement,
        }
    }
}