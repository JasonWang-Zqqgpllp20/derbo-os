use alloc::boxed::Box;

use super::BinaryExpressionSyntax::BinaryExpressionSyntax;
use super::LiteralExpressionSyntax::LiteralExpressionSyntax;
use super::ParenthesizedExpressionSyntax::ParenthesizedExpressionSyntax;
use super::UnaryExpressionSyntax::UnaryExpressionSyntax;
use super::AssignmentExpressionSyntax::AssignmentExpressionSyntax;
use super::NameExpressionSyntax::NameExpressionSyntax;
use super::CallExpressionSyntax::CallExpressionSyntax;

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxNode {
    AssignmentExpressionSyntax(Box<AssignmentExpressionSyntax>),
    BinaryExpressionSyntax(Box<BinaryExpressionSyntax>),
    CallExpressionSyntax(Box<CallExpressionSyntax>),
    NameExpressionSyntax(Box<NameExpressionSyntax>),
    LiteralExpressionSyntax(Box<LiteralExpressionSyntax>),
    ParenthesizedExpressionSyntax(Box<ParenthesizedExpressionSyntax>),
    UnaryExpressionSyntax(Box<UnaryExpressionSyntax>),
}