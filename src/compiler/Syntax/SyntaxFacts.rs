use super::SyntaxKind::SyntaxKind;
use alloc::string::String;

pub struct SyntaxFacts();

impl SyntaxFacts {
    pub fn GetUnaryOperatorPrecedence(kind: SyntaxKind) -> i32 {
        match kind {
            SyntaxKind::PlusToken | SyntaxKind::MinusToken | SyntaxKind::BangToken | SyntaxKind::TildeToken => 
                return 6,
            _ =>
                return 0,
        }
    }

    pub fn GetBinaryOperatorPrecedence(kind: SyntaxKind) -> i32 {
        match kind {
            SyntaxKind::StarToken | SyntaxKind::SlashToken => 
                return 5,
            SyntaxKind::PlusToken | SyntaxKind::MinusToken => 
                return 4,
            SyntaxKind::EqualsEqualsToken | SyntaxKind::BangEqualsToken |
            SyntaxKind::LessToken | SyntaxKind::LessOrEqualsToken |
            SyntaxKind::GreaterToken | SyntaxKind::GreaterOrEqualsToken => 
                return 3,
            SyntaxKind::AmpersandToken | SyntaxKind::AmpersandAmpersandToken => 
                return 2,
            SyntaxKind::PipeToken | SyntaxKind::PipePipeToken | SyntaxKind::HatToken => 
                return 1,
            _ => 
                return 0,
        }
    }

    pub fn GetKeywordKind(text: String) -> SyntaxKind {
        match text.as_str() {
            "else" => 
                return SyntaxKind::ElseKeyword,
            "false" => 
                return SyntaxKind::FalseKeyword,
            "for" => 
                return SyntaxKind::ForKeyword,
            "if" => 
                return SyntaxKind::IfKeyword,
            "let" => 
                return SyntaxKind::LetKeyword,
            "to" => 
                return SyntaxKind::ToKeyword,
            "true" => 
                return SyntaxKind::TrueKeyword,
            "var" => 
                return SyntaxKind::VarKeyword,
            "while" => 
                return SyntaxKind::WhileKeyword,
            _ => 
                return SyntaxKind::IdentifierToken,
        }
    }

    pub fn GetText(kind: SyntaxKind) -> Result<String, ()> {
        match kind {
            SyntaxKind::PlusToken => 
                return Ok(String::from("+")),
            SyntaxKind::MinusToken => 
                return Ok(String::from("-")),
            SyntaxKind::StarToken =>
                return Ok(String::from("*")),
            SyntaxKind::SlashToken => 
                return Ok(String::from("/")),
            SyntaxKind::BangToken => 
                return Ok(String::from("!")),
            SyntaxKind::EqualsToken => 
                return Ok(String::from("=")),
            SyntaxKind::TildeToken => 
                return Ok(String::from("~")),
            SyntaxKind::LessToken => 
                return Ok(String::from("<")),
            SyntaxKind::LessOrEqualsToken => 
                return Ok(String::from("<=")),
            SyntaxKind::GreaterToken => 
                return Ok(String::from(">")),
            SyntaxKind::GreaterOrEqualsToken => 
                return Ok(String::from(">=")),
            SyntaxKind::AmpersandToken => 
                return Ok(String::from("&")),
            SyntaxKind::AmpersandAmpersandToken => 
                return Ok(String::from("&&")),
            SyntaxKind::PipeToken => 
                return Ok(String::from("|")),
            SyntaxKind::PipePipeToken => 
                return Ok(String::from("||")),
            SyntaxKind::HatToken => 
                return Ok(String::from("^")),
            SyntaxKind::EqualsEqualsToken => 
                return Ok(String::from("==")),
            SyntaxKind::BangEqualsToken => 
                return Ok(String::from("!=")),
            SyntaxKind::OpenParenthesisToken => 
                return Ok(String::from("(")),
            SyntaxKind::CloseParenthesisToken => 
                return Ok(String::from(")")),
            SyntaxKind::OpenBraceToken => 
                return Ok(String::from("{")),
            SyntaxKind::CloseBraceToken => 
                return Ok(String::from("}")),
            SyntaxKind::CommaToken => 
                return Ok(String::from(",")),
            SyntaxKind::ElseKeyword => 
                return Ok(String::from("else")),
            SyntaxKind::FalseKeyword => 
                return Ok(String::from("false")),
            SyntaxKind::ForKeyword => 
                return Ok(String::from("for")),
            SyntaxKind::IfKeyword => 
                return Ok(String::from("if")),
            SyntaxKind::LetKeyword => 
                return Ok(String::from("let")),
            SyntaxKind::ToKeyword => 
                return Ok(String::from("to")),
            SyntaxKind::TrueKeyword => 
                return Ok(String::from("true")),
            SyntaxKind::VarKeyword => 
                return Ok(String::from("var")),
            SyntaxKind::WhileKeyword => 
                return Ok(String::from("while")),
            _ => 
                return Err(()),
        }
    }
}