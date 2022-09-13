#![allow(non_snake_case)]

use alloc::vec::Vec;
use alloc::string::String;

// use super::SyntaxToken::SyntaxToken;
use super::Parser::Parser;
use super::super::Diagnostic::Diagnostic;
use super::super::Text::SourceText::SourceText;
// use super::Lexer::Lexer;
// use super::SyntaxKind::SyntaxKind;
use super::CompilationUnitSyntax::CompilationUnitSyntax;

#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxTree {
    pub Text: SourceText,
    pub Diagnostics: Vec<Diagnostic>,
    pub Root: CompilationUnitSyntax,
}

impl SyntaxTree {
    pub fn new(text: SourceText) -> SyntaxTree {
        let mut parser = Parser::new();
        parser.init(text.clone());
        let root = parser.ParseCompilationUnit();
        let diagnostics = parser.Diagnostics().diagnostics;

        SyntaxTree {
            Text: text,
            Diagnostics: diagnostics,
            Root: root,
        }
    }

    pub fn Parse_from_Str(text: String) -> SyntaxTree {
        let sourceText = SourceText::From(text);
        return SyntaxTree::Parse(sourceText);
    }

    pub fn Parse(text: SourceText) -> SyntaxTree {
        return SyntaxTree::new(text);
    }

    // pub fn ParseTokens_from_Str(text: String) -> Vec<SyntaxToken> {
    //     let sourceText = SourceText::From(text);
    //     return SyntaxTree::ParseTokens(sourceText);
    // }

    // pub fn ParseTokens(text: SourceText) -> Vec<SyntaxToken> {
    //     let mut lexer = Lexer::new(text);
    //     let mut tokens: Vec<SyntaxToken> = Vec::new();

    //     loop {
    //         let token = lexer.Lex();
    //         tokens.push(token.clone());
    //         if token.Kind == SyntaxKind::EndOfFileToken {
    //             break;
    //         }
    //     }

    //     return tokens;
    // }
}