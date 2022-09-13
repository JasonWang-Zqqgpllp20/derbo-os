#![allow(non_snake_case)]

use super::SyntaxNode::SyntaxNode;
use super::SyntaxToken::SyntaxToken;
use super::super::Text::TextSpan::TextSpan;
// use super::ExpressionSyntax::ExpressionSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpressionSyntax {
    pub Left: SyntaxNode,
    pub OperatorToken: SyntaxToken,
    pub Right: SyntaxNode,
}

impl BinaryExpressionSyntax {
    pub fn new(left: SyntaxNode, operatorToken: SyntaxToken, right: SyntaxNode) -> BinaryExpressionSyntax {
        BinaryExpressionSyntax {
            Left: left,
            OperatorToken: operatorToken,
            Right: right,
        }
    }

    pub fn Span(&self) -> TextSpan {
        let first_span: TextSpan;
        match &self.Left {
            SyntaxNode::AssignmentExpressionSyntax(a) => 
                first_span = a.Span(),
            SyntaxNode::BinaryExpressionSyntax(b) => 
                first_span = b.Span(),
            SyntaxNode::NameExpressionSyntax(n) => 
                first_span = n.Span(),
            SyntaxNode::LiteralExpressionSyntax(l) => 
                first_span = l.Span(),
            SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                first_span = p.Span(),
            SyntaxNode::UnaryExpressionSyntax(u) => 
                first_span = u.Span(),
            SyntaxNode::CallExpressionSyntax(c) =>     // fake, not support for CallExpressionSyntax
                first_span = c.Span()
        }
        
        let last_span: TextSpan;
        match &self.Right {
            SyntaxNode::AssignmentExpressionSyntax(a) => 
                last_span = a.Span(),
            SyntaxNode::BinaryExpressionSyntax(b) => 
                last_span = b.Span(),
            SyntaxNode::NameExpressionSyntax(n) => 
                last_span = n.Span(),
            SyntaxNode::LiteralExpressionSyntax(l) => 
                last_span = l.Span(),
            SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                last_span = p.Span(),
            SyntaxNode::UnaryExpressionSyntax(u) => 
                last_span = u.Span(),
            SyntaxNode::CallExpressionSyntax(c) =>     // fake, not support for CallExpressionSyntax
                last_span = c.Span()
        }

        return TextSpan::FromBounds(first_span.Start, last_span.end())
    }

    // fn GetChildren(&self) -> (SyntaxNode, SyntaxToken, SyntaxNode) {
    //     (self.Left.clone(), self.OperatorToken.clone(), self.Right.clone())
    // }
}