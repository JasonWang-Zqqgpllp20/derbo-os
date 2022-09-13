#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct ParenthesizedExpressionSyntax {
    OpenParenthesisToken: SyntaxToken,
    pub Expression: SyntaxNode,
    CloseParenthesisToken: SyntaxToken,
}

impl ParenthesizedExpressionSyntax {
    pub fn new(openParenthesisToken: SyntaxToken,
                expression: SyntaxNode,
                closeParenthesisToken: SyntaxToken)
                -> ParenthesizedExpressionSyntax {
        ParenthesizedExpressionSyntax {
            OpenParenthesisToken: openParenthesisToken,
            Expression: expression,
            CloseParenthesisToken: closeParenthesisToken,
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

    // fn GetChildren(&self) -> (SyntaxToken, SyntaxNode, SyntaxToken) {
    //     (self.OpenParenthesisToken.clone(), self.Expression.clone(), self.CloseParenthesisToken.clone())
    // }
}