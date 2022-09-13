#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentExpressionSyntax {
    pub IdentifierToken: SyntaxToken,
    pub EqualsToken: SyntaxToken,
    pub Expression: SyntaxNode,
}

impl AssignmentExpressionSyntax {
    pub fn new(identifierToken: SyntaxToken, equalsToken: SyntaxToken, expression: SyntaxNode) -> AssignmentExpressionSyntax {
        AssignmentExpressionSyntax {
            IdentifierToken: identifierToken,
            EqualsToken: equalsToken,
            Expression: expression,
        }
    }

    pub fn Span(&self) -> TextSpan {
        let span: TextSpan;
        match &self.Expression {
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

    // pub fn GetChildren(&self) -> (SyntaxToken, SyntaxToken, SyntaxNode) {
    //     return (self.IdentifierToken.clone(), self.EqualsToken.clone(), self.Expression.clone());
    // }
}