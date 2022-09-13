#![allow(non_snake_case)]

use crate::{print, println};
use alloc::vec::Vec;
use super::Syntax::SyntaxTree::SyntaxTree;
use super::Compilation::Compilation;
use super::Symbol::VariableSymbol::VariableSymbolDictionary;
use crate::api::code2stringvec;

pub fn compile_run(text_list: Vec<Vec<char>>) {
    let text_list = code2stringvec(text_list);
    let mut variables: Vec<VariableSymbolDictionary> = Vec::new();
    let mut previous: Option<Compilation> = None;

    for text in text_list {let syntax_tree = SyntaxTree::Parse_from_Str(text);

        let mut compilation;
        match previous.clone() {
            Some(prev) => {
                compilation = prev.ContinueWith(syntax_tree.clone());
            },
            None => {
                compilation =  Compilation::new(previous.clone(), syntax_tree.clone());
            },
        }
        
        let (result, vari_ret) = compilation.Evaluate(variables.clone());
        variables = vari_ret;

        if result.Diagnostics.len() == 0 {
            match result.Value {
                _ => {}
            }
            previous = Some(compilation);
            
        } else {
            for diag in result.Diagnostics {
                let lineIndex = syntax_tree.Text.GetLineIndex(diag.Span.Start);
                let line = syntax_tree.Text.Lines[lineIndex as usize].clone();
                let lineNumber = lineIndex + 1;
                let character = diag.Span.Start - line.Start + 1;

                print!("({}, {})", lineNumber, character);
                println!("{}", diag.Message);
            }
        }
    }
}