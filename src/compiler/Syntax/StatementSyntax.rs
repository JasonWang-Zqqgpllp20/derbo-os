#![allow(non_snake_case)]

use alloc::boxed::Box;

use super::BlockStatementSyntax::BlockStatementSyntax;
// use super::CompilationUnitSyntax::CompilationUnitSyntax;
use super::ExpressionStatementSyntax::ExpressionStatementSyntax;
use super::VariableDeclarationSyntax::VariableDeclarationSyntax;
use super::IfStatementSyntax::IfStatementSyntax;
use super::ElseClauseSyntax::ElseClauseSyntax;
use super::ForStatementSyntax::ForStatementSyntax;
use super::WhileStatementSyntax::WhileStatementSyntax;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementSyntax {
    BlockStatementSyntax(Box<BlockStatementSyntax>),
    // CompilationUnitSyntax(Box<CompilationUnitSyntax>),
    ExpressionStatementSyntax(Box<ExpressionStatementSyntax>),
    VariableDeclarationSyntax(Box<VariableDeclarationSyntax>),
    
    IfStatementSyntax(Box<IfStatementSyntax>),
    ElseClauseSyntax(Box<ElseClauseSyntax>),
    ForStatementSyntax(Box<ForStatementSyntax>),
    WhileStatementSyntax(Box<WhileStatementSyntax>),
}