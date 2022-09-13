#[warn(unused_imports)]

#[derive(Clone, Debug, PartialEq)]
pub enum BoundUnaryOperatorKind {
    Identity,
    Negation,
    LogicalNegation,
    OnesComplement
}