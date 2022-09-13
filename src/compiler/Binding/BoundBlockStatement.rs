#![allow(non_snake_case)]

use alloc::vec::Vec;

use super::BoundStatement::BoundStatement;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundBlockStatement {
    pub Statements: Vec<BoundStatement>,
}

impl BoundBlockStatement {
    pub fn new(statements: Vec<BoundStatement>) -> BoundBlockStatement {
        BoundBlockStatement {
            Statements: statements,
        }
    }
}