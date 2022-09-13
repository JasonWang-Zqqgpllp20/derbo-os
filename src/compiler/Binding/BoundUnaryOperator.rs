#![allow(non_snake_case)]

use super::super::Syntax::SyntaxKind::SyntaxKind;
use super::BoundUnaryOperatorKind::BoundUnaryOperatorKind;
use super::super::Symbol::TypeSymbol::TypeSymbol;

static OPERATORS: [BoundUnaryOperator; 4] = [
    BoundUnaryOperator { SyntaxKind: SyntaxKind::BangToken, Kind: BoundUnaryOperatorKind::LogicalNegation, TypeSymbol: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    
    BoundUnaryOperator { SyntaxKind: SyntaxKind::PlusToken, Kind: BoundUnaryOperatorKind::Identity, TypeSymbol: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundUnaryOperator { SyntaxKind: SyntaxKind::MinusToken, Kind: BoundUnaryOperatorKind::Negation, TypeSymbol: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundUnaryOperator { SyntaxKind: SyntaxKind::TildeToken, Kind: BoundUnaryOperatorKind::OnesComplement, TypeSymbol: TypeSymbol::Int, Type: TypeSymbol::Int, },
];

#[derive(Clone, Debug, PartialEq)]
pub struct BoundUnaryOperator {
    SyntaxKind: SyntaxKind,
    pub Kind: BoundUnaryOperatorKind,
    pub TypeSymbol: TypeSymbol,
    pub Type: TypeSymbol,
}

impl BoundUnaryOperator {
    pub fn Bind(syntaxKind: SyntaxKind, TypeSymbol: TypeSymbol) -> Result<BoundUnaryOperator, ()> {
        for op in OPERATORS.clone() {
            if op.SyntaxKind == syntaxKind && op.TypeSymbol == TypeSymbol {
                return Ok(op)
            }
        }
        return Err(())
    }
}