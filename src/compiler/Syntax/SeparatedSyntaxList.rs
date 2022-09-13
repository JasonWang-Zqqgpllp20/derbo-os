#![allow(non_snake_case)]

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;

#[derive(Clone, Debug, PartialEq)]
pub enum ListValueType {
    SyntaxNode(Box<SyntaxNode>),
    SyntaxToken(Box<SyntaxToken>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeparatedSyntaxList {
    pub nodesAndSeparators: Vec<ListValueType>,
}

impl SeparatedSyntaxList {
    pub fn new(nodesAndSeparators: Vec<ListValueType>) -> SeparatedSyntaxList {
        SeparatedSyntaxList {
            nodesAndSeparators: nodesAndSeparators
        }
    }

    pub fn Count(&self) -> i32 {
        (self.nodesAndSeparators.len() as i32 + 1) / 2
    }

    pub fn Index(&self, index: i32) -> ListValueType {
        self.nodesAndSeparators[index as usize].clone()
    }

    // pub fn GetSparator(&self, index: i32) -> Option<ListValueType> {
    //     if index == self.Count() - 1 {
    //         return None
    //     } else {
    //         Some(self.nodesAndSeparators[index as usize].clone())
    //     }
    // }

    // pub fn GetWithSeparators(&self) -> Vec<ListValueType> {
    //     self.nodesAndSeparators.clone()
    // }

    pub fn iter(&self) -> Vec<ListValueType> {
        self.nodesAndSeparators.clone()
    }
}