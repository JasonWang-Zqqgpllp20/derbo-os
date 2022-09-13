#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct NameExpressionSyntax {
    pub IdentifierToken: SyntaxToken,
}

impl NameExpressionSyntax {
    pub fn new(identifierToken: SyntaxToken) -> NameExpressionSyntax {
        NameExpressionSyntax {
            IdentifierToken: identifierToken,
        }
    }

    pub fn Span(&self) -> TextSpan {
        self.IdentifierToken.Span()
    }

    // pub fn GetChildren(&self) -> SyntaxToken {
    //     return self.IdentifierToken.clone()
    // }
}