#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;

use super::ParameterSymbol::ParameterSymbol;
use super::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSymbol {
    pub Name: String,
    pub Parameter: Vec<ParameterSymbol>,
    pub Type: TypeSymbol,
}

impl FunctionSymbol {
    pub fn new(name: String, parameter: Vec<ParameterSymbol>, type_symbol: TypeSymbol) -> FunctionSymbol {
        FunctionSymbol {
            Name: name,
            Parameter: parameter,
            Type: type_symbol,
        }
    }
}