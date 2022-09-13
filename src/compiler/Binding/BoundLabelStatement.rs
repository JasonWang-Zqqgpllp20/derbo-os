#![allow(non_snake_case)]

use super::super::Binding::BoundLabel::BoundLabel;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundLabelStatement {
    pub Label: BoundLabel,
}

impl BoundLabelStatement {
    pub fn new(label: BoundLabel) -> BoundLabelStatement {
        BoundLabelStatement {
            Label: label
        }
    }
}