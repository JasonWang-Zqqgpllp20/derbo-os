use alloc::boxed::Box;

use super::BoundBinaryExpression::BoundBinaryExpression;
use super::BoundLiteralExpression::BoundLiteralExpression;
use super::BoundUnaryExpression::BoundUnaryExpression;
use super::BoundAssignmentExpression::BoundAssignmentExpression;
use super::BoundVariableExpression::BoundVariableExpression;
use super::BoundErrorExpression::BoundErrorExpression;
use super::BoundConversionExpression::BoundConversionExpression;
use super::BoundCallExpression::BoundCallExpression;

#[derive(Clone, Debug, PartialEq)]
pub enum BoundNode {
    BoundAssignmentExpression(Box<BoundAssignmentExpression>),
    BoundVariableExpression(Box<BoundVariableExpression>),
    BoundLiteralExpression(Box<BoundLiteralExpression>),
    BoundUnaryExpression(Box<BoundUnaryExpression>),
    BoundBinaryExpression(Box<BoundBinaryExpression>),
    BoundErrorExpression(Box<BoundErrorExpression>),
    BoundConversionExpression(Box<BoundConversionExpression>),
    BoundCallExpression(Box<BoundCallExpression>),
}