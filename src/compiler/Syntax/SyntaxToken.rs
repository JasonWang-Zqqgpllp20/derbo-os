use alloc::string::String;

use super::SyntaxKind::SyntaxKind;
// use super::SyntaxNode::SyntaxNode;
use super::ValueType::ValueType;
use super::super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxToken {
    pub Kind: SyntaxKind,
    pub Position: i32,
    pub Text: Option<String>,
    pub Value: ValueType,
}

impl SyntaxToken {
    pub fn new(kind: SyntaxKind, position: i32, text: String, value: ValueType) -> SyntaxToken {
        SyntaxToken {
            Kind: kind,
            Position: position,
            Text: Some(text),
            Value: value,
        }
    }

    pub fn Span(&self) -> TextSpan {
        TextSpan::new(self.Position, self.Text.clone().unwrap().len() as i32)
    }

    pub fn IsMissing(&self) -> bool {
        self.Text == None
    }
}