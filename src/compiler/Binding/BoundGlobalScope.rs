#![allow(non_snake_case)]

use alloc::vec::Vec;
use alloc::boxed::Box;

use super::super::Diagnostic::Diagnostic;
use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::BoundStatement::BoundStatement;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundGlobalScope {
    pub Previous: Box<Option<BoundGlobalScope>>,
    pub Diagnostics: Vec<Diagnostic>,
    pub Variables: Vec<VariableSymbol>,
    pub Statement: BoundStatement,
}

impl BoundGlobalScope {
    pub fn new(previous: Option<BoundGlobalScope>, 
                diagnostics: Vec<Diagnostic>, 
                variables: Vec<VariableSymbol>, 
                statement: BoundStatement) 
                -> BoundGlobalScope {
        BoundGlobalScope {
            Previous: Box::new(previous),
            Diagnostics: diagnostics,
            Variables: variables,
            Statement: statement,
        }        
    }
}