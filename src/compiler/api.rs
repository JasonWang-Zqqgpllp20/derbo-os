#![allow(non_snake_case)]

pub fn isLetter(c: char) -> bool {
    if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') {
        return true;
    } else {
        return false;
    }
}
pub fn isWhiteSpace(c: char) -> bool {
    if c == ' ' || c == '\n' || c == '\r' || c == '\t' {
        return true;
    } else {
        return false;
    }
}
pub fn isDigit(c: char) -> bool {
    if c >= '0' && c <= '9' {
        return true;
    } else {
        return false;
    }
}