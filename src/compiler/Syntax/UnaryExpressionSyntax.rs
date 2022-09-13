#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpressionSyntax {
    pub OperatorToken: SyntaxToken,
    pub Operand: SyntaxNode,
}

impl UnaryExpressionSyntax {
    pub fn new(operatorToken: SyntaxToken, operand: SyntaxNode) -> UnaryExpressionSyntax {
        UnaryExpressionSyntax {
            OperatorToken: operatorToken,
            Operand: operand,
        }
    }

    pub fn Span(&self) -> TextSpan {
        let span: TextSpan;
        match &self.Operand {
            SyntaxNode::AssignmentExpressionSyntax(a) => 
                span = a.Span(),
            SyntaxNode::BinaryExpressionSyntax(b) => 
                span = b.Span(),
            SyntaxNode::NameExpressionSyntax(n) => 
                span = n.Span(),
            SyntaxNode::LiteralExpressionSyntax(l) => 
                span = l.Span(),
            SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                span = p.Span(),
            SyntaxNode::UnaryExpressionSyntax(u) => 
                span = u.Span(),
            SyntaxNode::CallExpressionSyntax(c) =>     // fake, not support for CallExpressionSyntax
                span = c.Span()
        }

        return TextSpan::FromBounds(span.Start, span.end())
    }

    // pub fn GetChildren(&self) -> (SyntaxToken, SyntaxNode) {
    //     (self.OperatorToken.clone(), self.Operand.clone())
    // }
}