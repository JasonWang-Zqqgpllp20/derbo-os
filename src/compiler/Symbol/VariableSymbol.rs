#![allow(non_snake_case)]

use alloc::string::String;

use super::super::Syntax::ValueType::ValueType;
use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableSymbol {
    pub Name: String,
    pub IsReadOnly: bool,
    pub Type: TypeSymbol,
}

impl VariableSymbol {
    pub fn new(name: String, isReadOnly: bool, variable_type: TypeSymbol) -> VariableSymbol {
        VariableSymbol {
            Name: name,
            IsReadOnly: isReadOnly,
            Type: variable_type,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableSymbolDictionary {
    pub Key: VariableSymbol,
    pub Value: ValueType            // what is an 'object' type?
}

impl VariableSymbolDictionary {
    pub fn new(key: VariableSymbol, value: ValueType) -> VariableSymbolDictionary {
        VariableSymbolDictionary {
            Key: key,
            Value: value,
        }
    }
}