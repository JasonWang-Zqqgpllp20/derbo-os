#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::super::Symbol::FunctionSymbol::FunctionSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundScope {
    pub Parent: Option<Box<BoundScope>>,
    pub variables: Option<Vec<(String, VariableSymbol)>>,
    pub functions: Option<Vec<(String, FunctionSymbol)>>,
}

impl BoundScope {
    pub fn new(parent: BoundScope) -> BoundScope {
        BoundScope {
            Parent: Some(Box::new(parent)),
            variables: Some(Vec::new()),
            functions: Some(Vec::new()),
        }
    }

    pub fn TryDeclareVariable(&mut self, variable: VariableSymbol) -> bool {
        if self.variables == None {
            self.variables = Some(Vec::new());
        }

        let mut find: bool = false;
        for vari in &self.variables.clone().unwrap() {
            if vari.0 == variable.Name {
                find = true;
                break;
            }
        }

        if find == true {
            return false;
        }
        else {
            match self.variables.clone() {
                Some(varis) => {
                    let mut v = varis;
                    v.push((variable.Name.clone(), variable));
                    self.variables = Some(v);
                }
                None => {}
            }
            return true;
        }
    }

    pub fn TryLookupVariable(&self, name: String) -> Result<VariableSymbol, ()> {
        let variable: VariableSymbol;

        match self.variables.clone() {
            Some(varis) => {
                for vari in varis {
                    if vari.0 == name {
                        variable = vari.1;
                        return Ok(variable);
                    }
                }
            }
            None => {}
        }
        
        match &self.Parent {
            Some(parent) => {
                return parent.TryLookupVariable(name);
            },
            None => {
                return Err(());
            }
        }
    }

    pub fn TryDeclareFunction(&mut self, function: FunctionSymbol) -> bool {
        if self.functions == None {
            self.functions = Some(Vec::new());
        }

        let mut find: bool = false;
        for func in &self.functions.clone().unwrap() {
            if func.0 == function.Name {
                find = true;
                break;
            }
        }

        if find == true {
            return false;
        }
        else {
            match self.functions.clone() {
                Some(funcs) => {
                    let mut v = funcs;
                    v.push((function.Name.clone(), function));
                    self.functions = Some(v);
                }
                None => {}
            }
            return true;
        }
    }

    pub fn TryLookupFunction(&self, name: String) -> Result<FunctionSymbol, ()> {
        let function: FunctionSymbol;

        match self.functions.clone() {
            Some(funcs) => {
                for func in funcs {
                    if func.0 == name {
                        function = func.1;
                        return Ok(function);
                    }
                }
            }
            None => {}
        }
        
        match &self.Parent {
            Some(parent) => {
                return parent.TryLookupFunction(name);
            },
            None => {
                return Err(());
            }
        }
    }

    pub fn GetDeclaredVariable(&self) -> Vec<VariableSymbol> {
        match self.variables.clone() {
            Some(varis) => {
                let mut v = Vec::new();
                for value in varis {
                    v.push(value.1.clone())
                }
                
                return v
            },
            None => {
                return Vec::new()
            }
        }
    }

    // pub fn GetDeclaredFunction(&self) -> Vec<FunctionSymbol> {
    //     match self.functions.clone() {
    //         Some(funcs) => {
    //             let mut v = Vec::new();
    //             for value in funcs {
    //                 v.push(value.1.clone())
    //             }
                
    //             return v
    //         },
    //         None => {
    //             return Vec::new()
    //         }
    //     }
    // }
}