#![allow(non_snake_case)]

#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    pub Start: i32,
    pub Length: i32,
}

impl TextSpan {
    pub fn new(start: i32, length: i32) -> TextSpan {
        TextSpan {
            Start: start,
            Length: length,
        }
    }

    pub fn end(&self) -> i32 {
        self.Start + self.Length
    }

    pub fn FromBounds(start: i32, end: i32) -> TextSpan {
        TextSpan::new(start, end)
    }

    // pub fn ToString(&self) -> String {
    //     let s = format!("{}..{}", self.Start, self.Length);
    //     String::from(s)
    // }
}