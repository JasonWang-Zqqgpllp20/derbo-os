#![allow(non_snake_case)]

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SyntaxKind {
    // Tokens
    BadToken,
    EndOfFileToken,
    WhitespaceToken,
    NumberToken,
    StringToken,
    PlusToken,
    MinusToken,
    StarToken,
    SlashToken,
    BangToken,
    EqualsToken,
    TildeToken,
    HatToken,
    AmpersandToken,
    AmpersandAmpersandToken,
    PipeToken,
    PipePipeToken,
    EqualsEqualsToken,
    BangEqualsToken,
    LessToken,
    LessOrEqualsToken,
    GreaterToken,
    GreaterOrEqualsToken,
    OpenParenthesisToken,
    CloseParenthesisToken,
    OpenBraceToken,
    CloseBraceToken,
    CommaToken,
    IdentifierToken,

    // Keywords
    ElseKeyword,
    FalseKeyword,
    ForKeyword,
    IfKeyword,
    LetKeyword,
    ToKeyword,
    TrueKeyword,
    VarKeyword,
    WhileKeyword,

    // // Nodes
    // CompilationUnit,
    // ElseClause,

    // // Statements
    // BlockStatement,
    // VariableDeclaration,
    // IfStatement,
    // WhileStatement,
    // ForStatement,
    // ExpressionStatement,

    // // Expressions
    // LiteralExpression,
    // NameExpression,
    // UnaryExpression,
    // BinaryExpression,
    // ParenthesizedExpression,
    // AssignmentExpression,
    // CallExpression,
}