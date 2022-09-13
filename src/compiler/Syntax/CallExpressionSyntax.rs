#![allow(non_snake_case)]

use alloc::vec::Vec;

use super::SyntaxToken::SyntaxToken;
use super::SeparatedSyntaxList::SeparatedSyntaxList;
use super::super::Text::TextSpan::TextSpan;
use super::SeparatedSyntaxList::ListValueType;
use super::SyntaxNode::SyntaxNode;

#[derive(Clone, Debug, PartialEq)]
pub struct CallExpressionSyntax {
    pub Identifier: SyntaxToken,
    OpenParenthesisToken: SyntaxToken,
    pub Arguments: SeparatedSyntaxList,
    CloseParenthesisToken: SyntaxToken,
}

impl CallExpressionSyntax {
    pub fn new(identifier: SyntaxToken,
                openParenthesisToken: SyntaxToken,
                arguments: SeparatedSyntaxList,
                closeParenthesisToken: SyntaxToken)
                -> CallExpressionSyntax {
        CallExpressionSyntax {
            Identifier: identifier,
            OpenParenthesisToken: openParenthesisToken,
            Arguments: arguments,
            CloseParenthesisToken: closeParenthesisToken,
        }
    }

    pub fn Span(&self) -> TextSpan {
        let mut span_vec: Vec<TextSpan> = Vec::new();
        for arguments in self.Arguments.iter() {
            match arguments {
                ListValueType::SyntaxNode(argument) => {
                    match *argument {
                        SyntaxNode::AssignmentExpressionSyntax(a) => {
                            span_vec.push(a.Span());
                        }
                        SyntaxNode::BinaryExpressionSyntax(b) => {
                            span_vec.push(b.Span());
                        }
                        SyntaxNode::NameExpressionSyntax(n) => {
                            span_vec.push(n.Span());
                        }
                        SyntaxNode::LiteralExpressionSyntax(l) => {
                            span_vec.push(l.Span());
                        }
                        SyntaxNode::ParenthesizedExpressionSyntax(p) => {
                            span_vec.push(p.Span());
                        }
                        SyntaxNode::UnaryExpressionSyntax(u) => {
                            span_vec.push(u.Span());
                        }
                        SyntaxNode::CallExpressionSyntax(c) => {
                            span_vec.push(c.Span());
                        }
                    }
                }
                _ => {}
            }
        }

        TextSpan::new(span_vec[0].Start, span_vec[span_vec.len() - 1].Length)
    }
}