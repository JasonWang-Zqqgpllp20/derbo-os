#![allow(non_snake_case)]

use alloc::vec::Vec;
use alloc::boxed::Box;

use super::Syntax::SyntaxTree::SyntaxTree;
use super::Binding::Binder::Binder;
use super::Evaluator::Evaluator;
use super::EvaluationResult::EvaluationResult;
use super::Symbol::VariableSymbol::VariableSymbolDictionary;
use super::Syntax::ValueType::ValueType;
use super::Binding::BoundGlobalScope::BoundGlobalScope;
use super::Binding::BoundScope::BoundScope;
use super::Binding::BoundBlockStatement::BoundBlockStatement;
use super::Lowering::Lowerer::Lowerer;

#[derive(Clone, Debug, PartialEq)]
pub struct Compilation {
    globalScope: Option<BoundGlobalScope>,
    pub Previous: Box<Option<Compilation>>,
    pub SyntaxTree: SyntaxTree,
}

impl Compilation {
    pub fn new(privious: Option<Compilation>, syntaxTree: SyntaxTree) -> Compilation {
        Compilation {
            globalScope: None,
            Previous: Box::new(privious),
            SyntaxTree: syntaxTree,
        }
    }

    fn GlobalScope(&mut self) -> Option<BoundGlobalScope> {
        match self.globalScope {
            Some(_) => {},
            None => {
                let binder = Binder::new(
                    BoundScope {
                        Parent: None,
                        variables: Some(Vec::new()),
                        functions: Some(Vec::new()),
                    }
                );

                let globalScope: BoundGlobalScope;
                match &*self.Previous {
                    Some(pre) => {
                        globalScope = binder.BindGlobalScope(pre.clone().GlobalScope(), self.SyntaxTree.Root.clone());
                    },
                    None => {
                        globalScope = binder.BindGlobalScope(None, self.SyntaxTree.Root.clone());
                    }
                }
                
                self.globalScope = Some(globalScope);
            }
        }
        return self.globalScope.clone();
    }

    pub fn ContinueWith(&self, syntaxTree: SyntaxTree) -> Compilation {
        return Compilation::new(Some(self.clone()), syntaxTree);
    }

    pub fn Evaluate(&mut self, variables: Vec<VariableSymbolDictionary>) -> (EvaluationResult, Vec<VariableSymbolDictionary>) {
        let mut diagnostics = self.SyntaxTree.Diagnostics.clone();
        for diag in self.GlobalScope().unwrap().Diagnostics {
            diagnostics.push(diag);
        }

        if diagnostics.len() != 0 {
            return (EvaluationResult::new(diagnostics, ValueType::Null), variables);
        }
        
        let statement = self.GetStatement();
        let mut evaluetor = Evaluator::new(statement.clone(), variables);
        let (value, vari_ret) = evaluetor.Evaluate();

        return (EvaluationResult::new(diagnostics, value.unwrap()), vari_ret);
    }

    pub fn GetStatement(&mut self) -> BoundBlockStatement {
        let result = self.GlobalScope().unwrap().Statement;
        let lowerer = Lowerer::new();
        let ret = lowerer.Lower(result);
        
        return ret;
    }
}