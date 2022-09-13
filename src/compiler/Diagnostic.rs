#![allow(non_snake_case)]
#[warn(unused_imports)]

use alloc::string::String;

use super::Text::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub Span: TextSpan,
    pub Message: String,
}

impl Diagnostic {
    pub fn new(span: TextSpan, message: String) -> Diagnostic {
        Diagnostic {
            Span: span,
            Message: message,
        }
    }
}