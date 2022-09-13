#![allow(non_snake_case)]

use alloc::string::String;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundLabel {
    pub Name: String,
}

impl BoundLabel {
    pub fn new(name: String) -> BoundLabel {
        BoundLabel {
            Name: name,
        }
    }
}