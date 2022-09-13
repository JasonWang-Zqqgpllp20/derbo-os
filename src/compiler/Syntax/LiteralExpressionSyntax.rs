#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::ValueType::ValueType;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralExpressionSyntax {
    pub NumberToken: SyntaxToken,
    pub Value: ValueType,
}

impl LiteralExpressionSyntax {
    pub fn new(numberToken: SyntaxToken, value: ValueType) -> LiteralExpressionSyntax {
        LiteralExpressionSyntax {
            NumberToken: numberToken,
            Value: value
        }
    }

    pub fn Span(&self) -> TextSpan {
        self.NumberToken.Span()
    }

    // fn GetChildren(&self) -> SyntaxToken {
    //     self.NumberToken.clone()
    // }
}