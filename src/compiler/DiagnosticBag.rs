#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use super::Diagnostic::Diagnostic;
use super::Text::TextSpan::TextSpan;
use super::Syntax::SyntaxKind::SyntaxKind;
use super::Symbol::TypeSymbol::TypeSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> DiagnosticBag {
        DiagnosticBag {
            diagnostics: Vec::new(),
        }
    }

    pub fn AddRange(&mut self, diagnostics: DiagnosticBag) {
        for diag in diagnostics.diagnostics {
            self.diagnostics.push(diag);
        }
    }

    pub fn Report(&mut self, span: TextSpan, message: String) {
        let diagnostic = Diagnostic::new(span, message);
        self.diagnostics.push(diagnostic);
    }

    pub fn ReportInvalidNumber(&mut self, span: TextSpan, text: String, value_type: TypeSymbol) {
        let message = format!("{} {} {} {:?}", "The number", text, "isn't valid", value_type);
        self.Report(span, message);
    }

    pub fn ReportBadCharacter(&mut self, position: i32, character: char) {
        let span = TextSpan::new(position, 1);
        let message = format!("{} '{}'", "Bad character input: ", character);
        self.Report(span, message);
    }

    pub fn ReportUnterminatedString(&mut self, span: TextSpan) {
        let message = String::from("Unterminated string literal.");
        self.Report(span, message);
    }

    pub fn ReportUnexpectedToken(&mut self, span: TextSpan, actualKind: SyntaxKind, expectedKind: SyntaxKind) {
        let message = format!("{} <{:?}> {} <{:?}>", "Unexpected token", actualKind, ", expected", expectedKind);
        self.Report(span, message);
    }

    pub fn ReportUndefinedUnaryOperator(&mut self, span: TextSpan, operatorText: String, TypeSymbol: TypeSymbol) {
        let message = format!("{} '{}' {} '{:?}'", "Unary operator", operatorText, "is not defined for type", TypeSymbol);
        self.Report(span, message);
    }

    pub fn ReportUndefinedBinaryOperator(&mut self, span: TextSpan, operatorText: String, leftType: TypeSymbol, rightType: TypeSymbol) {
        let message = format!("{} '{}' {} '{:?}' {} '{:?}'", "Binary operator", operatorText, "is not defined for types", leftType, "and", rightType);
        self.Report(span, message);
    }
    
    pub fn ReportUndefinedName(&mut self, span: TextSpan, name: String) {
        let message = format!("{} '{}' {}", "Variable", name, "doesn't exist.");
        self.Report(span, message);
    }

    pub fn ReportCannotConvert(&mut self, span: TextSpan, fromType: TypeSymbol, toType: TypeSymbol) {
        let message = format!("{} '{:?}' {} '{:?}'", "Cannot convert type", fromType, "to", toType);
        self.Report(span, message);
    }

    pub fn ReportVariableAlreadyDeclared(&mut self, span: TextSpan, name: String) {
        let message = format!("{} '{}' {}", "Variable", name, "is already declared");
        self.Report(span, message);
    }

    pub fn ReportCannotAssign(&mut self, span: TextSpan, name: String) {
        let message = format!("{} '{}' {}", "Variable", name, "is read-only and cannot be assigned to");
        self.Report(span, message);
    }

    pub fn ReportUndefinedFunction(&mut self, span: TextSpan, name: String) {
        let message = format!("{} '{}' {}", "Function", name, "doesn't exist");
        self.Report(span, message);
    }

    pub fn ReportWrongArgumentCount(&mut self, span: TextSpan, name: String, expectedCount: i32, actualCount: i32) {
        let message = format!("{} '{}' {} {:?} {} {:?}", "Function", name, "requires", expectedCount, "arguments but was given", actualCount);
        self.Report(span, message);
    }

    pub fn ReportWrongArgumentType(&mut self, span: TextSpan, name: String, expectedType: TypeSymbol, actualType: TypeSymbol) {
        let message = format!("{} '{}' {} {:?} {} {:?}", "Parameter", name, "requires a value of type", expectedType, "but was given a value of type", actualType);
        self.Report(span, message);
    }

    pub fn ReportExpressionMustHaveValue(&mut self, span: TextSpan) {
        let message = format!("{}", "Expression must have a value");
        self.Report(span, message);
    }
}