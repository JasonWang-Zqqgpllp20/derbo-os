#![allow(non_snake_case)]

use super::StatementSyntax::StatementSyntax;
use super::SyntaxToken::SyntaxToken;

#[derive(Clone, Debug, PartialEq)]
pub struct CompilationUnitSyntax {
    pub Statement: StatementSyntax,
    EndOfFileToken: SyntaxToken,
}

impl CompilationUnitSyntax {
    pub fn new(statement: StatementSyntax, endOfFileToken: SyntaxToken) -> CompilationUnitSyntax {
        CompilationUnitSyntax {
            Statement: statement,
            EndOfFileToken: endOfFileToken,
        }
    }
}