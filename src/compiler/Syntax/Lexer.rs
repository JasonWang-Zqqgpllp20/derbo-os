#![allow(non_snake_case)]

use alloc::string::String;

// use alloc::string::String;
use super::super::api::{isLetter, isDigit, isWhiteSpace};
use super::SyntaxFacts::SyntaxFacts;
use super::SyntaxKind::SyntaxKind;
use super::SyntaxToken::SyntaxToken;
use super::ValueType::ValueType;
use super::super::DiagnosticBag::DiagnosticBag;
use super::super::Text::TextSpan::TextSpan;
use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::super::Text::SourceText::SourceText;

#[derive(Clone, Debug, PartialEq)]
pub struct Lexer {
    text: SourceText,
    position: usize,
    start: usize,
    kind: SyntaxKind,
    value: ValueType,
    pub diagnostics: DiagnosticBag,
}

impl Lexer {
    pub fn new(text: SourceText) -> Lexer {
        Lexer {
            text: text,
            position: 0,
            start: 0,
            kind: SyntaxKind::BadToken,
            value: ValueType::Null,
            diagnostics: DiagnosticBag::new(),
        }
    }

    fn Peek(&self, offset: i32) -> char {
        let index = self.position + offset as usize;
        if index >= self.text.Length() as usize {
            return '\0';
        } else {
            return self.text.index(index as i32);
        }
    }

    fn Current(&self) -> char {
        self.Peek(0)
    }
    fn Lookahead(&self) -> char {
        self.Peek(1)
    }

    pub fn Diagnostics(&self) -> DiagnosticBag {
        self.diagnostics.clone()
    }

    pub fn Lex(&mut self) -> SyntaxToken {
        self.start = self.position;
        self.kind = SyntaxKind::BadToken;
        self.value = ValueType::Null;

        match self.Current() {
            '\0' => {
                self.kind = SyntaxKind::EndOfFileToken;
            },
            '+' => {
                self.kind = SyntaxKind::PlusToken;
                self.position += 1;
            },
            '-' => {
                self.kind = SyntaxKind::MinusToken;
                self.position += 1;
            },
            '*' => {
                self.kind = SyntaxKind::StarToken;
                self.position += 1;
            },
            '/' => {
                self.kind = SyntaxKind::SlashToken;
                self.position += 1;
            },
            '(' => {
                self.kind = SyntaxKind::OpenParenthesisToken;
                self.position += 1;
            },
            ')' => {
                self.kind = SyntaxKind::CloseParenthesisToken;
                self.position += 1;
            },
            '{' => {
                self.kind = SyntaxKind::OpenBraceToken;
                self.position += 1;
            },
            '}' => {
                self.kind = SyntaxKind::CloseBraceToken;
                self.position += 1;
            },
            ',' => {
                self.kind = SyntaxKind::CommaToken;
                self.position += 1;
            },
            '~' => {
                self.kind = SyntaxKind::TildeToken;
                self.position += 1;
            },
            '^' => {
                self.kind = SyntaxKind::HatToken;
                self.position += 1;
            },
            '&' => {
                self.position += 1;
                if self.Current() != '&' {
                    self.kind = SyntaxKind::AmpersandToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::AmpersandAmpersandToken;
                }
            },
            '|' => {
                self.position += 1;
                if self.Current() != '|' {
                    self.kind = SyntaxKind::PipeToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::PipePipeToken;
                }
            },
            '=' => {
                self.position += 1;
                if self.Current() != '=' {
                    self.kind = SyntaxKind::EqualsToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::EqualsEqualsToken;
                }
            },
            '!' => {
                self.position += 1;
                if self.Current() != '=' {
                    self.kind = SyntaxKind::BangToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::BangEqualsToken;
                }
            },
            '<' => {
                self.position += 1;
                if self.Current() != '=' {
                    self.kind = SyntaxKind::LessToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::LessOrEqualsToken;
                }
            },
            '>' => {
                self.position += 1;
                if self.Current() != '=' {
                    self.kind = SyntaxKind::GreaterToken;
                } else {
                    self.position += 1;
                    self.kind = SyntaxKind::GreaterOrEqualsToken;
                }
            },
            '"' => {
                self.ReadString();
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                self.ReadNumberToken();
            },
            ' ' | '\t' | '\n' | '\r' => {
                self.ReadWhiteSpace();
            },
            _ => {
                if isLetter(self.Current()) {
                    self.ReadIdentifierOrKeyword();
                } else if isWhiteSpace(self.Current()) {
                    self.ReadWhiteSpace();
                } else {
                    self.diagnostics.ReportBadCharacter(self.position as i32, self.Current());
                    self.position += 1;
                }
            }
        }

        let length = self.position - self.start;
        match SyntaxFacts::GetText(self.kind) {
            Ok(text) => {
                return SyntaxToken::new(self.kind, self.start as i32, text, self.value.clone());
            },
            Err(_) => {
                let text = self.text.ToString_i32(self.start as i32, length as i32);
                return SyntaxToken::new(self.kind, self.start as i32, text, self.value.clone());
            }
        }

        
    }

    fn ReadWhiteSpace(&mut self) {
        while isWhiteSpace(self.Current()) {
            self.position += 1;
        }

        self.kind = SyntaxKind::WhitespaceToken;
    }

    fn ReadString(&mut self) {
        // Skip the current quote
        self.position += 1;

        let mut sb = String::from("");
        let mut done = false;

        while !done {
            match self.Current() {
                '\0' | '\r' | '\n' => {
                    let span = TextSpan::new(self.start as i32, 1);
                    self.diagnostics.ReportUnterminatedString(span);
                    done = true;
                }
                '"' => {
                    if self.Lookahead() == '"' {
                        sb.push(self.Current());
                        self.position += 2;
                    }
                    else
                    {
                        self.position += 1;
                        done = true;
                    }
                }
                _ => {
                    sb.push(self.Current());
                    self.position += 1;
                }
                    
            }
        }

        self.kind = SyntaxKind::StringToken;
        self.value = ValueType::String(sb);
    }

    fn ReadNumberToken(&mut self) {
        while isDigit(self.Current()) {
            self.position += 1;
        }

        let length = self.position - self.start;
        let text = self.text.ToString_i32(self.start as i32, length as i32);
        match text.parse::<i32>() {
            Ok(value) => self.value = ValueType::Int32(value),
            Err(_) => {
                self.value = ValueType::Null;
                self.diagnostics.ReportInvalidNumber(
                    TextSpan::new(self.start as i32, length as i32), text, TypeSymbol::Int);
            }
        }

        self.kind = SyntaxKind::NumberToken;
    }
    
    fn ReadIdentifierOrKeyword(&mut self) {
        while isLetter(self.Current()) {
            self.position += 1;
        }

        let length = self.position - self.start;
        let text = self.text.ToString_i32(self.start as i32, length as i32);
        
        self.kind = SyntaxFacts::GetKeywordKind(text);
    }
}