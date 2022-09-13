#![allow(non_snake_case)]

use alloc::string::String;
use alloc::{vec, vec::Vec};

use super::FunctionSymbol::FunctionSymbol;
use super::ParameterSymbol::ParameterSymbol;
use super::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinFunctions {

}

impl BuiltinFunctions {
    pub fn GetAll() -> Vec<FunctionSymbol> {
        let mut v = Vec::new();
        v.push(BuiltinFunctions::Print());
        v.push(BuiltinFunctions::Input());
        v.push(BuiltinFunctions::Rand());
        v.push(BuiltinFunctions::Sleep());
        v.push(BuiltinFunctions::Breakpoint());
        v
    }

    pub fn Print() -> FunctionSymbol {
        FunctionSymbol::new(
            String::from("print"),
            vec![ParameterSymbol::new(String::from("text"), false, TypeSymbol::String)],
            TypeSymbol::Void
        )
    }
    
    pub fn Input() -> FunctionSymbol {
        FunctionSymbol::new(
            String::from("input"),
            vec![],
            TypeSymbol::String
        )
    }
    
    pub fn Rand() -> FunctionSymbol {
        FunctionSymbol::new(
            String::from("rand"),
            vec![ParameterSymbol::new(String::from("max"), false, TypeSymbol::Int)],
            TypeSymbol::Int
        )
    }
    
    pub fn Sleep() -> FunctionSymbol {
        FunctionSymbol::new(
            String::from("sleep"),
            vec![ParameterSymbol::new(String::from("time"), false, TypeSymbol::Int)],
            TypeSymbol::Void
        )
    }
    
    pub fn Breakpoint() -> FunctionSymbol {
        FunctionSymbol::new(
            String::from("breakpoint"),
            vec![],
            TypeSymbol::Void
        )
    }
}