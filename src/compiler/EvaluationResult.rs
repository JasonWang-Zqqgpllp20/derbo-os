#![allow(non_snake_case)]

use alloc::vec::Vec;

use super::Diagnostic::Diagnostic;
use super::Syntax::ValueType::ValueType;

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluationResult {
    pub Diagnostics: Vec<Diagnostic>,
    pub Value: ValueType,
}

impl EvaluationResult {
    pub fn new(diagnostics: Vec<Diagnostic>, value: ValueType) -> EvaluationResult {
        EvaluationResult {
            Diagnostics: diagnostics,
            Value: value,
        }
    }
}