#![allow(non_snake_case)]

use alloc::string::String;

use super::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterSymbol {
    pub Name: String,
    IsReadOnly: bool,
    pub Type: TypeSymbol,
}

impl ParameterSymbol {
    pub fn new(name: String, isReadOnly: bool, type_symbol: TypeSymbol) -> ParameterSymbol {
        ParameterSymbol {
            Name: name,
            IsReadOnly: isReadOnly,
            Type: type_symbol,
        }
    }
}