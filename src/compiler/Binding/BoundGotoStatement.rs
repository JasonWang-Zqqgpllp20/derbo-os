#![allow(non_snake_case)]

use super::super::Binding::BoundLabel::BoundLabel;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundGotoStatement {
    pub Label: BoundLabel,
}

impl BoundGotoStatement {
    pub fn new(label: BoundLabel) -> BoundGotoStatement {
        BoundGotoStatement {
            Label: label
        }
    }
}