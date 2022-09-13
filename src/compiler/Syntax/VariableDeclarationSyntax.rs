#![allow(non_snake_case)]

use super::SyntaxToken::SyntaxToken;
use super::SyntaxNode::SyntaxNode;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclarationSyntax {
    pub Keyword: SyntaxToken,
    pub Identifier: SyntaxToken,
    EqualsToken: SyntaxToken,
    pub Initializer: SyntaxNode,
}

impl VariableDeclarationSyntax {
    pub fn new(keyword: SyntaxToken, identifier: SyntaxToken, equalsToken: SyntaxToken, initializer: SyntaxNode) -> VariableDeclarationSyntax {
        VariableDeclarationSyntax {
            Keyword: keyword,
            Identifier: identifier,
            EqualsToken: equalsToken,
            Initializer: initializer,
        }
    }
}