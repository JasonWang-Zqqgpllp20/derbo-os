#![allow(non_snake_case)]

use super::super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct Conversion {
    pub Exists: bool,
    IsIdentity: bool,
    IsImplicit: bool,
    IsExplicit: bool,
}

impl Conversion {
    pub fn new(exists: bool, isIdentity: bool, isImplicit: bool) -> Conversion {
        Conversion {
            Exists: exists,
            IsIdentity: isIdentity,
            IsImplicit: isImplicit,
            IsExplicit: exists && !isImplicit,
        }
    }

    pub fn Classify(from: TypeSymbol, to: TypeSymbol) -> Conversion {
        if from == to {
            return Conversion::Identity();
        }

        if from == TypeSymbol::Bool || from == TypeSymbol::Int {
            if to == TypeSymbol::String {
                return Conversion::Explicit();
            }
        }

        if from == TypeSymbol::String {
            if to == TypeSymbol::Bool || to == TypeSymbol::Int {
                return Conversion::Explicit()
            }
        }

        return Conversion::None();
    }

    pub fn None() -> Conversion {
        Conversion::new(false, false, false)
    }

    pub fn Identity() -> Conversion {
        Conversion::new(true, true, true)
    }

    // pub fn Implicit() -> Conversion {
    //     Conversion::new(true, false, true)
    // }

    pub fn Explicit() -> Conversion {
        Conversion::new(true, false, false)
    }
}