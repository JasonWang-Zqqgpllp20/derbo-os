#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;

use super::TextLine::TextLine;
// use super::TextSpan::TextSpan;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceText {
    pub text: String,
    pub Lines: Vec<TextLine>,
}

impl SourceText {
    pub fn new(text: String) -> SourceText {
        SourceText {
            text: text,
            Lines: Vec::new(),
        }
    }
    pub fn init(&mut self, text: String) -> SourceText {
        let lines = self.ParseLines(self.clone(), text.clone());
        SourceText {
            text: text,
            Lines: lines,
        }
    }

    pub fn index(&self, index: i32) -> char {
        self.text.as_bytes()[index as usize] as char
    }

    pub fn Length(&self) -> i32 {
        self.text.len() as i32
    }

    pub fn GetLineIndex(&self, position: i32) -> i32 {
        let mut lower: i32 = 0;
        let mut upper: i32 = self.Lines.len() as i32 - 1;

        while lower <= upper {
            let index: i32 = lower + (upper - lower) / 2;
            let start: i32 = self.Lines[index as usize].Start;

            if position == start {
                return index;
            }

            if start > position {
                upper = index - 1;
            } else {
                lower = index + 1;
            }
        }

        return lower - 1;
    }

    pub fn ParseLines(&mut self, sourceText: SourceText, text: String) -> Vec<TextLine> {
        let mut position = 0;
        let mut lineStart = 0;

        let len = text.len();
        while position < len {
            let lineBreakWidth = self.GetLineBreakWidth(text.clone(), position as i32);

            if lineBreakWidth == 0 {
                position += 1;
            } else {
                self.AddLine(sourceText.clone(), position as i32, lineStart, lineBreakWidth);

                position += lineBreakWidth as usize;
                lineStart = position as i32;
            }
        }

        if position > lineStart as usize {
            self.AddLine(sourceText, position as i32, lineStart, 0)
        }

        self.Lines.clone()
    }

    // doesn't do anything?????
    pub fn AddLine(&mut self, sourceText: SourceText, position: i32, lineStart: i32, lineBreakWidth: i32) {
        let lineLength = position - lineStart;
        let lindLengthIncludingLineBreak = lineLength + lineBreakWidth;
        let line = TextLine::new(sourceText, lineStart, lineLength, lindLengthIncludingLineBreak);
        self.Lines.push(line)
    }

    pub fn GetLineBreakWidth(&self, text: String, position: i32) -> i32 {
        let c = self.text.as_bytes()[position as usize] as char;
        let l;
        if position + 1 >= text.len() as i32 {
            l = '\0';
        } else {
            l = text.as_bytes()[position as usize + 1] as char;
        }

        if c == '\r' && l == '\n' {
            return 2;
        } else if c == '\r' || c == '\n' {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn From(text: String) -> SourceText {
        SourceText::new(text.clone()).init(text)
    }

    // pub fn ToString(&self) -> String {
    //     self.text.clone()
    // }

    pub fn ToString_i32(&self, start: i32, length: i32) -> String {
        String::from_utf8(self.text.as_bytes()[start as usize .. (start + length) as usize].to_vec()).unwrap()
    }

    // pub fn ToString_span(&self, span: TextSpan) -> String {
    //     self.ToString_i32(span.Start, span.Length)
    // }
}