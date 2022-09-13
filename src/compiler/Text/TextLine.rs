#![allow(non_snake_case)]

use super::SourceText::SourceText;
// use super::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct TextLine {
    Text: SourceText,
    pub Start: i32,
    Length: i32,
    LengthIncludingLineBreak: i32,
}

impl TextLine {
    pub fn new(text: SourceText, start: i32, length: i32, lengthIncludingLineBreak: i32) -> TextLine {
        TextLine {
            Text: text,
            Start: start,
            Length: length,
            LengthIncludingLineBreak: lengthIncludingLineBreak,
        }
    }

    // pub fn End(&self) -> i32 {
    //     self.Start + self.Length
    // }

    // pub fn Span(&self) -> TextSpan {
    //     TextSpan::new(self.Start, self.Length)
    // }

    // pub fn ToString(&self) -> String {
    //     self.Text.ToString_span(self.Span())
    // }
}