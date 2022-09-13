#![allow(non_snake_case)]

use super::super::Syntax::SyntaxKind::SyntaxKind;
use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::BoundBinaryOperatorKind::BoundBinaryOperatorKind;

static OPERATORS: [BoundBinaryOperator; 21] = [
    BoundBinaryOperator { SyntaxKind: SyntaxKind::PlusToken, Kind: BoundBinaryOperatorKind::Addition, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::MinusToken, Kind: BoundBinaryOperatorKind::Subtraction, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::StarToken, Kind: BoundBinaryOperatorKind::Multiplication, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::SlashToken, Kind: BoundBinaryOperatorKind::Division, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::AmpersandToken, Kind: BoundBinaryOperatorKind::BitwiseAnd, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::PipeToken, Kind: BoundBinaryOperatorKind::BitwiseOr, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::HatToken, Kind: BoundBinaryOperatorKind::BitwiseXor, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Int, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::EqualsEqualsToken, Kind: BoundBinaryOperatorKind::Equals, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::BangEqualsToken, Kind: BoundBinaryOperatorKind::NotEquals, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::LessToken, Kind: BoundBinaryOperatorKind::Less, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::LessOrEqualsToken, Kind: BoundBinaryOperatorKind::LessOrEquals, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::GreaterToken, Kind: BoundBinaryOperatorKind::Greater, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::GreaterOrEqualsToken, Kind: BoundBinaryOperatorKind::GreaterOrEquals, LeftType: TypeSymbol::Int, RightType: TypeSymbol::Int, Type: TypeSymbol::Bool, },
    
    BoundBinaryOperator { SyntaxKind: SyntaxKind::AmpersandToken, Kind: BoundBinaryOperatorKind::BitwiseAnd, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::AmpersandAmpersandToken, Kind: BoundBinaryOperatorKind::LogicalAnd, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::PipeToken, Kind: BoundBinaryOperatorKind::BitwiseOr, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::PipePipeToken, Kind: BoundBinaryOperatorKind::LogicalOr, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::HatToken, Kind: BoundBinaryOperatorKind::BitwiseXor, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::EqualsEqualsToken, Kind: BoundBinaryOperatorKind::Equals, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },
    BoundBinaryOperator { SyntaxKind: SyntaxKind::BangEqualsToken, Kind: BoundBinaryOperatorKind::NotEquals, LeftType: TypeSymbol::Bool, RightType: TypeSymbol::Bool, Type: TypeSymbol::Bool, },

    BoundBinaryOperator { SyntaxKind: SyntaxKind::PlusToken, Kind: BoundBinaryOperatorKind::Addition, LeftType: TypeSymbol::String, RightType: TypeSymbol::String, Type: TypeSymbol::String, },
];

#[derive(Clone, Debug, PartialEq)]
pub struct BoundBinaryOperator {
    SyntaxKind: SyntaxKind,
    pub Kind: BoundBinaryOperatorKind,
    LeftType: TypeSymbol,
    RightType: TypeSymbol,
    pub Type: TypeSymbol,
}

impl BoundBinaryOperator {
    pub fn Bind(syntaxKind: SyntaxKind,
                leftType: TypeSymbol,
                rightType: TypeSymbol,
                ) -> Result<BoundBinaryOperator, ()> {
        for op in OPERATORS.clone() {
            if op.SyntaxKind == syntaxKind && op.LeftType == leftType && op.RightType == rightType {
                return Ok(op);
            }
        }

        return Err(())
    }
}