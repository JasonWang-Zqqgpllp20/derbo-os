use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::SyntaxToken::SyntaxToken;
use super::SyntaxKind::SyntaxKind;
use super::BinaryExpressionSyntax::BinaryExpressionSyntax;
use super::ParenthesizedExpressionSyntax::ParenthesizedExpressionSyntax;
use super::LiteralExpressionSyntax::LiteralExpressionSyntax;
use super::UnaryExpressionSyntax::UnaryExpressionSyntax;
use super::SyntaxFacts::SyntaxFacts;
use super::SyntaxNode::SyntaxNode;
use super::Lexer::Lexer;
use super::ValueType::ValueType;
use super::super::DiagnosticBag::DiagnosticBag;
use super::AssignmentExpressionSyntax::AssignmentExpressionSyntax;
use super::NameExpressionSyntax::NameExpressionSyntax;
use super::super::Text::SourceText::SourceText;
use super::CompilationUnitSyntax::CompilationUnitSyntax;
use super::StatementSyntax::StatementSyntax;
use super::BlockStatementSyntax::BlockStatementSyntax;
use super::ExpressionStatementSyntax::ExpressionStatementSyntax;
use super::VariableDeclarationSyntax::VariableDeclarationSyntax;
use super::IfStatementSyntax::IfStatementSyntax;
use super::ElseClauseSyntax::ElseClauseSyntax;
use super::WhileStatementSyntax::WhileStatementSyntax;
use super::ForStatementSyntax::ForStatementSyntax;
use super::CallExpressionSyntax::CallExpressionSyntax;
use super::SeparatedSyntaxList::{SeparatedSyntaxList, ListValueType};

#[derive(Clone, Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<SyntaxToken>,
    text: SourceText,
    diagnostics: DiagnosticBag,
    position: i32,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            text: SourceText::new(String::from("")),
            diagnostics: DiagnosticBag::new(),
            position: 0,
        }
    }

    pub fn init(&mut self, text: SourceText) {
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        let mut lexer = Lexer::new(text.clone());

        let mut token = lexer.Lex();
        while token.Kind != SyntaxKind::EndOfFileToken {
            if token.Kind != SyntaxKind::WhitespaceToken && token.Kind != SyntaxKind::BadToken {
                tokens.push(token.clone());
            }

            token = lexer.Lex();
        }
        tokens.push(token.clone());

        self.text = text;
        self.tokens = tokens;
        self.diagnostics.AddRange(lexer.Diagnostics());
    }

    pub fn Diagnostics(&self) -> DiagnosticBag {
        self.diagnostics.clone()
    }

    fn Peek(&self, offset: i32) -> SyntaxToken {
        let index = (self.position + offset) as usize;
        if index >= self.tokens.len() {
            return (self.tokens[self.tokens.len() - 1]).clone();
        }
        return (self.tokens[index]).clone();
    }

    fn Current(&self) -> SyntaxToken {
        self.Peek(0)
    }

    fn NextToken(&mut self) -> SyntaxToken {
        let current = self.Current();
        self.position += 1;
        return current;
    }

    fn MatchToken(&mut self, kind: SyntaxKind) -> SyntaxToken {
        if self.Current().Kind == kind {
            return (self.NextToken()).clone();
        } else {
            self.diagnostics.ReportUnexpectedToken(self.Current().Span(), self.Current().Kind, kind);
            return SyntaxToken::new(kind, self.Current().Position, String::from(""), ValueType::Null);
        }
    }

    // pub fn Parse(&mut self) -> SyntaxTree {
    //     let expression = self.ParseExpression(0);
    //     let endOfFileTOken = self.MatchToken(SyntaxKind::EndOfFileToken);
    //     return SyntaxTree::new(self.text.clone(), self.diagnostics.diagnostics.clone(), expression, endOfFileTOken);
    // }
    pub fn ParseCompilationUnit(&mut self) -> CompilationUnitSyntax {
        let statement = self.ParseStatement();
        let endOfFileToken = self.MatchToken(SyntaxKind::EndOfFileToken);

        return CompilationUnitSyntax::new(statement, endOfFileToken);
    }

    fn ParseExpression(&mut self) -> SyntaxNode {
        return self.ParseAssignmentExpression();
    }

    fn ParseStatement(&mut self) -> StatementSyntax {
        match self.Current().Kind {
            SyntaxKind::OpenBraceToken =>
                return self.ParseBlockStatement(),
            SyntaxKind::LetKeyword | SyntaxKind::VarKeyword=> 
                return self.ParseVariableDeclaration(),
            SyntaxKind::IfKeyword =>
                return self.ParseIfStatement(),
            SyntaxKind::WhileKeyword =>
                return self.ParseWhileStatement(),
            SyntaxKind::ForKeyword =>
                return self.ParseForStatement(),
            _ => 
                return self.ParseExpressionStatement(),
        }
    }

    fn ParseBlockStatement(&mut self) -> StatementSyntax {
        let mut statements: Vec<StatementSyntax> = Vec::new(); 

        let openBraceToken = self.MatchToken(SyntaxKind::OpenBraceToken);

        while self.Current().Kind != SyntaxKind::EndOfFileToken &&
              self.Current().Kind != SyntaxKind::CloseBraceToken {
            let statement = self.ParseStatement();
            statements.push(statement);
        }

        let closeBraceToken = self.MatchToken(SyntaxKind::CloseBraceToken);

        return StatementSyntax::BlockStatementSyntax(
            Box::new(BlockStatementSyntax::new(openBraceToken, statements, closeBraceToken))
        );
    }

    fn ParseVariableDeclaration(&mut self) -> StatementSyntax {
        let expected: SyntaxKind;
        match self.Current().Kind {
            SyntaxKind::LetKeyword => expected = SyntaxKind::LetKeyword,
            _ => expected = SyntaxKind::VarKeyword,
        }
        let keyword = self.MatchToken(expected);
        let identifier = self.MatchToken(SyntaxKind::IdentifierToken);
        let equals = self.MatchToken(SyntaxKind::EqualsToken);
        let initializer = self.ParseExpression();
        
        return StatementSyntax::VariableDeclarationSyntax(
            Box::new(VariableDeclarationSyntax::new(keyword, identifier, equals, initializer)));
    }

    fn ParseIfStatement(&mut self) -> StatementSyntax {
        let keyword = self.MatchToken(SyntaxKind::IfKeyword);
        let condition = self.ParseExpression();
        let statement = self.ParseStatement();
        let elseClause = self.ParseElseClause();
        return StatementSyntax::IfStatementSyntax(
            Box::new(IfStatementSyntax::new(keyword, condition, statement, elseClause))     // ????
        );
    }

    fn ParseElseClause(&mut self) -> Option<StatementSyntax> {
        if self.Current().Kind != SyntaxKind::ElseKeyword {
            return None;
        } 

        let keyword = self.NextToken();
        let statement = self.ParseStatement();
        return Some(StatementSyntax::ElseClauseSyntax(
            Box::new(ElseClauseSyntax::new(keyword, statement))
        ));
    }

    fn ParseWhileStatement(&mut self) -> StatementSyntax {
        let keyword = self.MatchToken(SyntaxKind::WhileKeyword);
        let condition = self.ParseExpression();
        let body = self.ParseStatement();
        return StatementSyntax::WhileStatementSyntax(
            Box::new(WhileStatementSyntax::new(keyword, condition, body))
        );
    }

    fn ParseForStatement(&mut self) -> StatementSyntax {
        let keyword = self.MatchToken(SyntaxKind::ForKeyword);
        let identifier = self.MatchToken(SyntaxKind::IdentifierToken);
        let equalsToken = self.MatchToken(SyntaxKind::EqualsToken);
        let lowerBound = self.ParseExpression();
        let toKeyword = self.MatchToken(SyntaxKind::ToKeyword);
        let upperBound = self.ParseExpression();
        let body = self.ParseStatement();
        
        return StatementSyntax::ForStatementSyntax(
            Box::new(ForStatementSyntax::new(keyword, identifier, equalsToken, lowerBound, toKeyword, upperBound, body))
        );
    }

    fn ParseExpressionStatement(&mut self) -> StatementSyntax {
        let expression = self.ParseExpression();
        return StatementSyntax::ExpressionStatementSyntax(
            Box::new(ExpressionStatementSyntax::new(expression))); 
    }

    fn ParseAssignmentExpression(&mut self) -> SyntaxNode {
        if self.Peek(0).Kind == SyntaxKind::IdentifierToken &&
            self.Peek(1).Kind == SyntaxKind::EqualsToken {
            let identifierToken = self.NextToken();
            let operatorToken = self.NextToken();
            let right = self.ParseAssignmentExpression();
            return SyntaxNode::AssignmentExpressionSyntax(Box::new(AssignmentExpressionSyntax::new(identifierToken, operatorToken, right)));
        }

        return self.ParseBinaryExpression(0);
    }

    fn ParseBinaryExpression(&mut self, parentPrecedence: i32) -> SyntaxNode {
        let mut left: SyntaxNode;
        let unaryOperatorPrecedence = SyntaxFacts::GetUnaryOperatorPrecedence(self.Current().Kind);
        if unaryOperatorPrecedence != 0 && unaryOperatorPrecedence >= parentPrecedence {
            let operatorToken = self.NextToken();
            let operand = self.ParseExpression();
            left = SyntaxNode::UnaryExpressionSyntax(Box::new(UnaryExpressionSyntax::new(operatorToken, operand)));
        } else {
            left = self.ParsePrimaryExpression();
        }

        loop {
            let precedence = SyntaxFacts::GetBinaryOperatorPrecedence(self.Current().Kind);
            if precedence == 0 || precedence <= parentPrecedence {
                break;
            }

            let operatorToken = self.NextToken();
            let right = self.ParseBinaryExpression(precedence);
            left = SyntaxNode::BinaryExpressionSyntax(Box::new(BinaryExpressionSyntax::new(left, operatorToken, right)));
        }

        return left;
    }

    fn ParsePrimaryExpression(&mut self) -> SyntaxNode {
        match self.Current().Kind {
            SyntaxKind::OpenParenthesisToken => {
                return self.ParseParenthesizedExpression();
            },
            SyntaxKind::FalseKeyword | SyntaxKind::TrueKeyword => {
                return self.ParseBooleanLiteral();
            },
            SyntaxKind::NumberToken => {
                return self.ParseNumberLiteral();
            },
            SyntaxKind::StringToken => {
                return self.ParseStringLiteral();
            }
            _ => {
                return self.ParseNameOrCallExpression();
            }
        }
    }

    fn ParseParenthesizedExpression(&mut self) -> SyntaxNode {
        let left = self.MatchToken(SyntaxKind::OpenParenthesisToken);
        let expression = self.ParseExpression();
        let right = self.MatchToken(SyntaxKind::CloseParenthesisToken);
        return SyntaxNode::ParenthesizedExpressionSyntax(
            Box::new(ParenthesizedExpressionSyntax::new(left, expression, right))
        );
    }

    fn ParseBooleanLiteral(&mut self) -> SyntaxNode {
        let isTrue = self.Current().Kind == SyntaxKind::TrueKeyword;
        let keywordToken;

        match isTrue {
            true => {
                keywordToken = self.MatchToken(SyntaxKind::TrueKeyword);
            },
            false => {
                keywordToken = self.MatchToken(SyntaxKind::FalseKeyword);
            }
        }
        return SyntaxNode::LiteralExpressionSyntax(
            Box::new(LiteralExpressionSyntax::new(keywordToken, ValueType::Bool(isTrue)))
        );
    }

    fn ParseNumberLiteral(&mut self) -> SyntaxNode {
        let numberToken = self.MatchToken(SyntaxKind::NumberToken);
        let value = numberToken.Value.clone();
        return SyntaxNode::LiteralExpressionSyntax(
            Box::new(LiteralExpressionSyntax::new(numberToken, value))
        );
    }

    fn ParseStringLiteral(&mut self) -> SyntaxNode {
        let stringToken = self.MatchToken(SyntaxKind::StringToken);
        let value = stringToken.Value.clone();
        return SyntaxNode::LiteralExpressionSyntax(
            Box::new(LiteralExpressionSyntax::new(stringToken, value))
        );
    }

    fn ParseNameOrCallExpression(&mut self) -> SyntaxNode {
        if self.Peek(0).Kind == SyntaxKind::IdentifierToken &&
           self.Peek(1).Kind == SyntaxKind::OpenParenthesisToken {
            return self.ParseCallExpression();
        }

        return self.ParseNameExpression();
    }

    fn ParseCallExpression(&mut self) -> SyntaxNode {
        let identifier = self.MatchToken(SyntaxKind::IdentifierToken);
        let openParenthesisToken = self.MatchToken(SyntaxKind::OpenParenthesisToken);
        let arguments = self.ParseArguments();
        let closeParenthesisToken = self.MatchToken(SyntaxKind::CloseParenthesisToken);
        return SyntaxNode::CallExpressionSyntax(
            Box::new(CallExpressionSyntax::new(identifier, openParenthesisToken, arguments, closeParenthesisToken))
        );
    }

    fn ParseArguments(&mut self) -> SeparatedSyntaxList {
        let mut nodesAndSeparators: Vec<ListValueType> = Vec::new();

        while self.Current().Kind != SyntaxKind::CloseParenthesisToken &&
               self.Current().Kind != SyntaxKind::EndOfFileToken
        {
            let expression = self.ParseExpression();
            nodesAndSeparators.push(ListValueType::SyntaxNode(Box::new(expression)));

            if self.Current().Kind != SyntaxKind::CloseParenthesisToken {
                let comma = self.MatchToken(SyntaxKind::CommaToken);
                nodesAndSeparators.push(ListValueType::SyntaxToken(Box::new(comma)));
            }
        }

        return SeparatedSyntaxList::new(nodesAndSeparators);
    }

    fn ParseNameExpression(&mut self) -> SyntaxNode {
        let identifierToken = self.MatchToken(SyntaxKind::IdentifierToken);
        return SyntaxNode::NameExpressionSyntax(
            Box::new(NameExpressionSyntax::new(identifierToken))
        );
    }
}