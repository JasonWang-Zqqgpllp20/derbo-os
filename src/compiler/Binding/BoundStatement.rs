#![allow(non_snake_case)]

use alloc::boxed::Box;

use super::BoundNode::BoundNode;
use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::BoundBlockStatement::BoundBlockStatement;
use super::BoundExpressionStatement::BoundExpressionStatement;
// use super::BoundGlobalScope::BoundGlobalScope;
use super::BoundIfStatement::BoundIfStatement;
use super::BoundForStatement::BoundForStatement;
use super::BoundWhileStatement::BoundWhileStatement;
use super::BoundGotoStatement::BoundGotoStatement;
use super::BoundConditionalGotoStatement::BoundConditionalGotoStatement;
use super::BoundLabelStatement::BoundLabelStatement;

#[derive(Clone, Debug, PartialEq)]
pub enum BoundStatement {
    BoundBlockStatement(Box<BoundBlockStatement>),
    BoundExpressionStatement(Box<BoundExpressionStatement>),
    // BoundGlobalScope(Box<BoundGlobalScope>),
    BoundVariableDeclaration(Box<BoundVariableDeclaration>),
    BoundIfStatement(Box<BoundIfStatement>),
    BoundForStatement(Box<BoundForStatement>),
    BoundWhileStatement(Box<BoundWhileStatement>),
    BoundGotoStatement(Box<BoundGotoStatement>),
    BoundConditionalGotoStatement(Box<BoundConditionalGotoStatement>),
    BoundLabelStatement(Box<BoundLabelStatement>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct BoundVariableDeclaration {
    pub Variable: VariableSymbol,
    pub Initializer: BoundNode,
}

impl BoundVariableDeclaration {
    pub fn new(variable: VariableSymbol, initializer: BoundNode) -> BoundVariableDeclaration {
        BoundVariableDeclaration {
            Variable: variable,
            Initializer: initializer,
        }
    }
}