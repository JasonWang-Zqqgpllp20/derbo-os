#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::BoundNode::BoundNode;
use super::BoundLiteralExpression::BoundLiteralExpression;
use super::BoundUnaryExpression::BoundUnaryExpression;
use super::BoundBinaryExpression::BoundBinaryExpression;
use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::BoundUnaryOperator::BoundUnaryOperator;
use super::BoundBinaryOperator::BoundBinaryOperator;
use super::super::Syntax::SyntaxNode::SyntaxNode;
use super::super::Syntax::ValueType::ValueType;
use super::super::Syntax::LiteralExpressionSyntax::LiteralExpressionSyntax;
use super::super::Syntax::UnaryExpressionSyntax::UnaryExpressionSyntax;
use super::super::Syntax::BinaryExpressionSyntax::BinaryExpressionSyntax;
use super::super::Syntax::ParenthesizedExpressionSyntax::ParenthesizedExpressionSyntax;
use super::super::Syntax::NameExpressionSyntax::NameExpressionSyntax;
use super::super::Syntax::AssignmentExpressionSyntax::AssignmentExpressionSyntax;
use super::super::Symbol::VariableSymbol::VariableSymbol;
use super::super::DiagnosticBag::DiagnosticBag;
use super::BoundVariableExpression::BoundVariableExpression;
use super::BoundAssignmentExpression::BoundAssignmentExpression;
use super::BoundScope::BoundScope;
use super::BoundGlobalScope::BoundGlobalScope;
use super::super::Syntax::CompilationUnitSyntax::CompilationUnitSyntax;
use super::super::Syntax::StatementSyntax::StatementSyntax;
use super::BoundStatement::{BoundStatement, BoundVariableDeclaration};
use super::super::Syntax::SyntaxKind::SyntaxKind;
use super::BoundBlockStatement::BoundBlockStatement;
use super::super::Syntax::BlockStatementSyntax::BlockStatementSyntax;
use super::super::Syntax::VariableDeclarationSyntax::VariableDeclarationSyntax;
use super::super::Syntax::ExpressionStatementSyntax::ExpressionStatementSyntax;
use super::BoundExpressionStatement::BoundExpressionStatement;
use super::super::Text::TextSpan::TextSpan;
use super::super::Syntax::IfStatementSyntax::IfStatementSyntax;
use super::super::Syntax::ForStatementSyntax::ForStatementSyntax;
use super::super::Syntax::WhileStatementSyntax::WhileStatementSyntax;
use super::BoundIfStatement::BoundIfStatement;
use super::BoundForStatement::BoundForStatement;
use super::BoundWhileStatement::BoundWhileStatement;
use super::super::Syntax::SyntaxToken::SyntaxToken;
use super::BoundErrorExpression::BoundErrorExpression;
use super::super::Symbol::BuiltinFunctions::BuiltinFunctions;
use super::super::Syntax::CallExpressionSyntax::CallExpressionSyntax;
use super::Conversion::Conversion;
use super::BoundConversionExpression::BoundConversionExpression;
use super::super::Syntax::SeparatedSyntaxList::ListValueType;
use super::super::Symbol::FunctionSymbol::FunctionSymbol;
use super::BoundCallExpression::BoundCallExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct Binder {
    pub scope: BoundScope,
    pub diagnostics: DiagnosticBag,
}

impl Binder {
    pub fn new(parent: BoundScope) -> Binder {
        Binder {
            scope: BoundScope::new(parent),
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub fn Diagnostics(&self) -> DiagnosticBag {
        self.diagnostics.clone()
    }

    pub fn BindGlobalScope(&self, previous: Option<BoundGlobalScope>, syntax: CompilationUnitSyntax) -> BoundGlobalScope {
        let parentScope = self.CreateParentScope(previous.clone());
        let mut binder = Binder::new(parentScope);
        let expression = binder.BindStatement(syntax.Statement);
        let variables = binder.scope.GetDeclaredVariable();
        let mut diagnostics = binder.Diagnostics();


        match previous.clone() {
            Some(pre) => {
                diagnostics.AddRange(DiagnosticBag  { diagnostics: pre.Diagnostics } );
            },
            None => {},
        }

        return BoundGlobalScope::new(previous, diagnostics.diagnostics, variables, expression);
    }

    pub fn CreateParentScope(&self, mut previous: Option<BoundGlobalScope>) -> BoundScope {
        let mut stack: Vec<BoundGlobalScope> = Vec::new();
        loop {
            match previous {
                Some(prev) => {
                    stack.push(prev.clone());
                    previous = *prev.Previous;
                },
                None => break,
            }
        }

        let mut parent = self.CreateRootScope();

        while stack.len() > 0 {
            previous = stack.pop();
            let mut scope = BoundScope::new(parent);
            for v in previous.unwrap().Variables {
                scope.TryDeclareVariable(v);
            }

            parent = scope;
        }

        return parent;
    }

    fn CreateRootScope(&self) -> BoundScope {
        let mut result = BoundScope {
            Parent: None,
            variables: Some(Vec::new()),
            functions: Some(Vec::new()),
        };

        for f in BuiltinFunctions::GetAll() {
            result.TryDeclareFunction(f);
        }
            

        return result;
    }

    pub fn BindStatement(&mut self, syntax: StatementSyntax) -> BoundStatement {
        match syntax {
            StatementSyntax::BlockStatementSyntax(b) => {
                return self.BindBlockStatement(*b);
            },
            StatementSyntax::VariableDeclarationSyntax(v) => {
                return self.BindVariableDeclaration(*v);
            },
            StatementSyntax::IfStatementSyntax(i) =>{
                return self.BindIfStatement(*i);
            },
            StatementSyntax::WhileStatementSyntax(w) =>{
                return self.BindWhileStatement(*w);
            },
            StatementSyntax::ForStatementSyntax(f) =>{
                return self.BindForStatement(*f);
            },
            StatementSyntax::ExpressionStatementSyntax(e) => {
                return self.BindExpressionStatement(*e);
            },
            _ => return BoundStatement::BoundBlockStatement(        // fake
                Box::new(BoundBlockStatement::new(Vec::new()))
            ),
        }
    }

    pub fn BindBlockStatement(&mut self, syntax: BlockStatementSyntax) -> BoundStatement {
        let mut statements: Vec<BoundStatement> = Vec::new();
        self.scope = BoundScope::new(self.scope.clone());
        for statementSyntax in syntax.Statements {
            let statement = self.BindStatement(statementSyntax);
            statements.push(statement);
        }

        self.scope = *self.scope.Parent.clone().unwrap();

        return BoundStatement::BoundBlockStatement(
            Box::new(BoundBlockStatement::new(statements))
        );
    }

    pub fn BindVariableDeclaration(&mut self, syntax: VariableDeclarationSyntax) -> BoundStatement {
        let isReadOnly = syntax.Keyword.Kind == SyntaxKind::LetKeyword;
        let initializer = self.BindExpression(syntax.Initializer, false);
        let initializer_type;
        match initializer.clone() {
            BoundNode::BoundErrorExpression(e) => {
                initializer_type = (*e).Type();
            },
            BoundNode::BoundAssignmentExpression(a) => {
                initializer_type = a.Type();
            }
            BoundNode::BoundBinaryExpression(b) => {
                initializer_type = b.Type();
            }
            BoundNode::BoundLiteralExpression(l) => {
                initializer_type = l.Type();
            }
            BoundNode::BoundUnaryExpression(u) => {
                initializer_type = u.Type();
            }
            BoundNode::BoundVariableExpression(v) => {
                initializer_type = v.Type();
            }
            BoundNode::BoundCallExpression(c) => {
                initializer_type = c.Type();
            }
            BoundNode::BoundConversionExpression(c) => {
                initializer_type = c.Type;
            }
        }
        
        let variable = self.BindVariable(syntax.Identifier, isReadOnly, initializer_type);

        return BoundStatement::BoundVariableDeclaration(
            Box::new(BoundVariableDeclaration::new(variable, initializer))
        );
    }

    pub fn BindIfStatement(&mut self, syntax: IfStatementSyntax) -> BoundStatement {
        let condition = self.BindExpression_target(syntax.Condition, TypeSymbol::Bool);
        let thenStatement = self.BindStatement(syntax.ThenStatement);
        let elseStatement: Option<BoundStatement>;

        match syntax.ElseClause {
            Some(state) => {
                match state {
                    StatementSyntax::ElseClauseSyntax(e) => elseStatement = Some(self.BindStatement(e.ElseStatement)),
                    _ => elseStatement = None,
                }
                
            },
            None => {
                elseStatement = None;
            },
        }
        return BoundStatement::BoundIfStatement(
            Box::new(BoundIfStatement::new(condition, thenStatement, elseStatement))
        );
        
    }

    pub fn BindWhileStatement(&mut self, syntax: WhileStatementSyntax) -> BoundStatement {
        let condition = self.BindExpression_target(syntax.Condition, TypeSymbol::Bool);
        let body = self.BindStatement(syntax.Body);
        return BoundStatement::BoundWhileStatement(
            Box::new(BoundWhileStatement::new(condition, body))
        );
    }

    pub fn BindForStatement(&mut self, syntax: ForStatementSyntax) -> BoundStatement {
        let lowerBound = self.BindExpression_target(syntax.LowerBound, TypeSymbol::Int);
        let upperBound = self.BindExpression_target(syntax.UpperBound, TypeSymbol::Int);
        
        self.scope = BoundScope::new(self.scope.clone());

        let variable = self.BindVariable(syntax.Identifier, true, TypeSymbol::Int);

        let body = self.BindStatement(syntax.Body);
        
        self.scope = *self.scope.Parent.clone().unwrap();

        return BoundStatement::BoundForStatement(
            Box::new(BoundForStatement::new(variable, lowerBound, upperBound, body))
        );
    }

    pub fn BindExpressionStatement(&mut self, syntax: ExpressionStatementSyntax) -> BoundStatement {
        let expression = self.BindExpression(syntax.Expression, true);

        return BoundStatement::BoundExpressionStatement(
            Box::new(BoundExpressionStatement::new(expression))
        );
    }

    pub fn BindExpression_target(&mut self, syntax: SyntaxNode, targetType: TypeSymbol) -> BoundNode {
        let result = self.BindExpression(syntax.clone(), false);

        let result_type: TypeSymbol;
        match result.clone() {
            BoundNode::BoundErrorExpression(e) => {
                result_type = e.Type();
            },
            BoundNode::BoundAssignmentExpression(a) => {
                result_type = a.Type();
            }
            BoundNode::BoundBinaryExpression(b) => {
                result_type = b.Type();
            }
            BoundNode::BoundLiteralExpression(l) => {
                result_type = l.Type();
            }
            BoundNode::BoundUnaryExpression(u) => {
                result_type = u.Type();
            }
            BoundNode::BoundVariableExpression(v) => {
                result_type = v.Type();
            }
            BoundNode::BoundCallExpression(c) => {
                result_type = c.Type();
            }
            BoundNode::BoundConversionExpression(c) => {
                result_type = c.Type;
            }
        }

        let syntax_span: TextSpan;
        match syntax {
            SyntaxNode::ParenthesizedExpressionSyntax(p) => {
                syntax_span = p.Span();
            },
            SyntaxNode::LiteralExpressionSyntax(l) => {
                syntax_span = l.Span();
            },
            SyntaxNode::NameExpressionSyntax(n) => {
                syntax_span = n.Span();
            },
            SyntaxNode::AssignmentExpressionSyntax(a) => {
                syntax_span = a.Span();
            },
            SyntaxNode::UnaryExpressionSyntax(u) => {
                syntax_span = u.Span();
            },
            SyntaxNode::BinaryExpressionSyntax(b) => {
                syntax_span = b.Span();
            }
            SyntaxNode::CallExpressionSyntax(_) => {    // fake, not support for CallExpressionSyntax
                syntax_span = TextSpan::new(0, 0);
            }
        }

        if targetType != TypeSymbol::Error && result_type != TypeSymbol::Error && targetType != result_type  {
            self.diagnostics.ReportCannotConvert(syntax_span, result_type, targetType);
        }

        return result;
    }

    pub fn BindExpression(&mut self, syntax: SyntaxNode, canBeVoid: bool) -> BoundNode {
        let result = self.BindExpressionInternal(syntax.clone());

        let result_type: TypeSymbol;
        match result.clone() {
            BoundNode::BoundErrorExpression(e) => {
                result_type = e.Type();
            },
            BoundNode::BoundAssignmentExpression(a) => {
                result_type = a.Type();
            }
            BoundNode::BoundBinaryExpression(b) => {
                result_type = b.Type();
            }
            BoundNode::BoundLiteralExpression(l) => {
                result_type = l.Type();
            }
            BoundNode::BoundUnaryExpression(u) => {
                result_type = u.Type();
            }
            BoundNode::BoundVariableExpression(v) => {
                result_type = v.Type();
            }
            BoundNode::BoundCallExpression(c) => {
                result_type = c.Type();
            }
            BoundNode::BoundConversionExpression(c) => {
                result_type = c.Type;
            }
        }
        let syntax_span: TextSpan;
        match syntax {
            SyntaxNode::AssignmentExpressionSyntax(a) => 
                syntax_span = a.Span(),
            SyntaxNode::BinaryExpressionSyntax(b) => 
                syntax_span = b.Span(),
            SyntaxNode::NameExpressionSyntax(n) => 
                syntax_span = n.Span(),
            SyntaxNode::LiteralExpressionSyntax(l) => 
                syntax_span = l.Span(),
            SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                syntax_span = p.Span(),
            SyntaxNode::UnaryExpressionSyntax(u) => 
                syntax_span = u.Span(),
            SyntaxNode::CallExpressionSyntax(_) =>     // fake, not support for CallExpressionSyntax
                syntax_span = TextSpan::new(0, 0)
        }

        if !canBeVoid && result_type == TypeSymbol::Void {
            self.diagnostics.ReportExpressionMustHaveValue(syntax_span);
            return BoundNode::BoundErrorExpression(
                Box::new(BoundErrorExpression::new())
            );
        }

        return result;
    }

    fn BindExpressionInternal(&mut self, syntax: SyntaxNode) -> BoundNode {
        match syntax {
            SyntaxNode::ParenthesizedExpressionSyntax(p) => {
                return self.BindParenthesizedExpression(*p);
            },
            SyntaxNode::LiteralExpressionSyntax(l) => {
                return self.BindLiteralExpression(*l);
            },
            SyntaxNode::NameExpressionSyntax(n) => {
                return self.BindNameExpression(*n);
            },
            SyntaxNode::AssignmentExpressionSyntax(a) => {
                return self.BindAssignmentExpression(*a);
            },
            SyntaxNode::UnaryExpressionSyntax(u) => {
                return self.BindUnaryExpression(*u);
            },
            SyntaxNode::BinaryExpressionSyntax(b) => {
                return self.BindBinaryExpression(*b);
            },
            SyntaxNode::CallExpressionSyntax(c) => {            // fake
                return self.BindCallExpression(*c);
            }
        }
    }

    fn BindParenthesizedExpression(&mut self, syntax: ParenthesizedExpressionSyntax) -> BoundNode {
        return self.BindExpression(syntax.Expression, false);
    }

    fn BindLiteralExpression(&self, syntax: LiteralExpressionSyntax) -> BoundNode {
        match syntax.Value {
            ValueType::Int32(i) => {
                return BoundNode::BoundLiteralExpression(
                    Box::new(BoundLiteralExpression::new(ValueType::Int32(i)))
                );
            },
            ValueType::Bool(b) => {
                return BoundNode::BoundLiteralExpression(
                    Box::new(BoundLiteralExpression::new(ValueType::Bool(b)))
                );
            },
            ValueType::String(s) => {
                return BoundNode::BoundLiteralExpression(
                    Box::new(BoundLiteralExpression::new(ValueType::String(s)))
                );
            },
            ValueType::Null => {
                return BoundNode::BoundLiteralExpression(
                    Box::new(BoundLiteralExpression::new(ValueType::Null))
                );
            }
        }
    }

    fn BindNameExpression(&mut self, syntax: NameExpressionSyntax) -> BoundNode {
        let name = syntax.IdentifierToken.Text.clone();

        if syntax.IdentifierToken.IsMissing() {
            return BoundNode::BoundErrorExpression(
                Box::new(BoundErrorExpression::new())
            );
        }
        
        match self.scope.TryLookupVariable(name.clone().unwrap()) {
            Ok(variable) => {
                self.scope.TryDeclareVariable(variable.clone());        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                return BoundNode::BoundVariableExpression(
                    Box::new(BoundVariableExpression::new(variable))
                );
            }
            Err(_) => {
                self.diagnostics.ReportUndefinedName(syntax.IdentifierToken.Span(), name.unwrap());
                return BoundNode::BoundErrorExpression(
                    Box::new(BoundErrorExpression::new())
                );
            }
        }
    }

    fn BindAssignmentExpression(&mut self, syntax: AssignmentExpressionSyntax) -> BoundNode {
        let name = syntax.IdentifierToken.Text.clone();
        let boundExpression = self.BindExpression(syntax.Expression.clone(), false);

        match self.scope.TryLookupVariable(name.clone().unwrap()) {
            Ok(variable) => {
                self.scope.TryDeclareVariable(variable.clone());        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                if variable.IsReadOnly {
                    self.diagnostics.ReportCannotAssign(syntax.EqualsToken.Span(), name.unwrap())
                }

                let boundExpression_type: TypeSymbol;
                match boundExpression.clone() {
                    BoundNode::BoundErrorExpression(e) => {
                        boundExpression_type = (*e).Type();
                    },
                    BoundNode::BoundLiteralExpression(l) => {
                        match (*l).Value {
                            ValueType::Int32(_) => boundExpression_type = TypeSymbol::Int,
                            ValueType::Bool(_) => boundExpression_type = TypeSymbol::Bool,
                            ValueType::String(_) => boundExpression_type = TypeSymbol::String,
                            ValueType::Null => boundExpression_type = TypeSymbol::Error,
                        }
                    },
                    BoundNode::BoundUnaryExpression(u) => {
                        boundExpression_type = (*u).Op.Type;
                    },
                    BoundNode::BoundBinaryExpression(b) => {
                        boundExpression_type = (*b).Op.Type;
                    },
                    BoundNode::BoundAssignmentExpression(a) => {
                        match (*a).Variable.Type {
                            TypeSymbol::Int => boundExpression_type = TypeSymbol::Int,
                            TypeSymbol::Bool => boundExpression_type = TypeSymbol::Bool,
                            TypeSymbol::String => boundExpression_type = TypeSymbol::String,
                            TypeSymbol::Error => boundExpression_type = TypeSymbol::Error,
                            TypeSymbol::Void => boundExpression_type = TypeSymbol::Void,
                        }
                    },
                    BoundNode::BoundVariableExpression(v) => {
                        match (*v).Variable.Type {
                            TypeSymbol::Int => boundExpression_type = TypeSymbol::Int,
                            TypeSymbol::Bool => boundExpression_type = TypeSymbol::Bool,
                            TypeSymbol::String => boundExpression_type = TypeSymbol::String,
                            TypeSymbol::Error => boundExpression_type = TypeSymbol::Error,
                            TypeSymbol::Void => boundExpression_type = TypeSymbol::Void,
                        }
                    },
                    BoundNode::BoundCallExpression(c) => {
                        boundExpression_type = (*c).Type();
                    },
                    BoundNode::BoundConversionExpression(c) => {
                        boundExpression_type = (*c).Type;
                    },
                }
                if boundExpression_type != variable.Type {
                    let syntax_span: TextSpan;
                    match syntax.Expression {
                        SyntaxNode::AssignmentExpressionSyntax(a) => 
                            syntax_span = a.Span(),
                        SyntaxNode::BinaryExpressionSyntax(b) => 
                            syntax_span = b.Span(),
                        SyntaxNode::NameExpressionSyntax(n) => 
                            syntax_span = n.Span(),
                        SyntaxNode::LiteralExpressionSyntax(l) => 
                            syntax_span = l.Span(),
                        SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                            syntax_span = p.Span(),
                        SyntaxNode::UnaryExpressionSyntax(u) => 
                            syntax_span = u.Span(),
                        SyntaxNode::CallExpressionSyntax(_) =>     // fake, not support for CallExpressionSyntax
                            syntax_span = TextSpan::new(0, 0)
                    }

                    self.diagnostics.ReportCannotConvert(syntax_span, boundExpression_type, variable.Type.clone())
                }

                return BoundNode::BoundAssignmentExpression(
                    Box::new(BoundAssignmentExpression::new(variable, boundExpression))
                );
            },
            Err(_) => {
                self.diagnostics.ReportUndefinedName(syntax.IdentifierToken.Span(), name.unwrap());
                return boundExpression;
            }
        }

        
    }

    fn BindUnaryExpression(&mut self, syntax: UnaryExpressionSyntax) -> BoundNode {
        let boundOperand = self.BindExpression(syntax.Operand, false);
        let boundOperand_type: TypeSymbol;
        match boundOperand.clone() {
            BoundNode::BoundErrorExpression(e) => {
                boundOperand_type = (*e).Type();
            },
            BoundNode::BoundLiteralExpression(l) => {
                match (*l).Value {
                    ValueType::Int32(_) => boundOperand_type = TypeSymbol::Int,
                    ValueType::Bool(_) => boundOperand_type = TypeSymbol::Bool,
                    ValueType::String(_) => boundOperand_type = TypeSymbol::String,
                    ValueType::Null => boundOperand_type = TypeSymbol::Error,
                }
            },
            BoundNode::BoundUnaryExpression(u) => {
                boundOperand_type = (*u).Op.Type;
            },
            BoundNode::BoundBinaryExpression(b) => {
                boundOperand_type = (*b).Op.Type;
            },
            BoundNode::BoundAssignmentExpression(a) => {
                match (*a).Variable.Type {
                    TypeSymbol::Int => boundOperand_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundOperand_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundOperand_type = TypeSymbol::String,
                    TypeSymbol::Error => boundOperand_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundOperand_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundVariableExpression(v) => {
                match (*v).Variable.Type {
                    TypeSymbol::Int => boundOperand_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundOperand_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundOperand_type = TypeSymbol::String,
                    TypeSymbol::Error => boundOperand_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundOperand_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundCallExpression(c) => {
                boundOperand_type = (*c).Type();
            },
            BoundNode::BoundConversionExpression(c) => {
                boundOperand_type = (*c).Type;
            },
        }
        
        let boundOperator = BoundUnaryOperator::Bind(syntax.OperatorToken.Kind, boundOperand_type.clone());

        match boundOperator {
            Ok(op) => {
                return BoundNode::BoundUnaryExpression(
                    Box::new(BoundUnaryExpression::new(op, boundOperand))
                );
            },
            Err(_) => {
                self.diagnostics.ReportUndefinedUnaryOperator(syntax.OperatorToken.Span(), syntax.OperatorToken.Text.unwrap(), boundOperand_type);
                return BoundNode::BoundErrorExpression(
                    Box::new(BoundErrorExpression::new())
                );
            },
        }
    }

    fn BindBinaryExpression(&mut self, syntax: BinaryExpressionSyntax) -> BoundNode {
        let boundLeft = self.BindExpression(syntax.Left, false);
        let boundRight = self.BindExpression(syntax.Right, false);

        let boundLeft_type: TypeSymbol;
        match boundLeft.clone() {
            BoundNode::BoundErrorExpression(e) => {
                boundLeft_type = (*e).Type();
            },
            BoundNode::BoundLiteralExpression(l) => {
                match (*l).Value {
                    ValueType::Int32(_) => boundLeft_type = TypeSymbol::Int,
                    ValueType::Bool(_) => boundLeft_type = TypeSymbol::Bool,
                    ValueType::String(_) => boundLeft_type = TypeSymbol::String,
                    ValueType::Null => boundLeft_type = TypeSymbol::Error,
                }
            },
            BoundNode::BoundUnaryExpression(u) => {
                boundLeft_type = (*u).Op.Type;
            },
            BoundNode::BoundBinaryExpression(b) => {
                boundLeft_type = (*b).Op.Type;
            },
            BoundNode::BoundAssignmentExpression(a) => {
                match (*a).Variable.Type {
                    TypeSymbol::Int => boundLeft_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundLeft_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundLeft_type = TypeSymbol::String,
                    TypeSymbol::Error => boundLeft_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundLeft_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundVariableExpression(v) => {
                match (*v).Variable.Type {
                    TypeSymbol::Int => boundLeft_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundLeft_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundLeft_type = TypeSymbol::String,
                    TypeSymbol::Error => boundLeft_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundLeft_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundCallExpression(c) => {
                boundLeft_type = (*c).Type();
            },
            BoundNode::BoundConversionExpression(c) => {
                boundLeft_type = (*c).Type;
            },
        }
        let boundRight_type: TypeSymbol;
        match boundRight.clone() {
            BoundNode::BoundErrorExpression(e) => {
                boundRight_type = (*e).Type();
            },
            BoundNode::BoundLiteralExpression(l) => {
                match (*l).Value {
                    ValueType::Int32(_) => boundRight_type = TypeSymbol::Int,
                    ValueType::Bool(_) => boundRight_type = TypeSymbol::Bool,
                    ValueType::String(_) => boundRight_type = TypeSymbol::String,
                    ValueType::Null => boundRight_type = TypeSymbol::Error,
                }
            },
            BoundNode::BoundUnaryExpression(u) => {
                boundRight_type = (*u).Op.Type;
            },
            BoundNode::BoundBinaryExpression(b) => {
                boundRight_type = (*b).Op.Type;
            },
            BoundNode::BoundAssignmentExpression(a) => {
                match (*a).Variable.Type {
                    TypeSymbol::Int => boundRight_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundRight_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundRight_type = TypeSymbol::String,
                    TypeSymbol::Error => boundRight_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundRight_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundVariableExpression(v) => {
                match (*v).Variable.Type {
                    TypeSymbol::Int => boundRight_type = TypeSymbol::Int,
                    TypeSymbol::Bool => boundRight_type = TypeSymbol::Bool,
                    TypeSymbol::String => boundRight_type = TypeSymbol::String,
                    TypeSymbol::Error => boundRight_type = TypeSymbol::Error,
                    TypeSymbol::Void => boundRight_type = TypeSymbol::Void,
                }
            },
            BoundNode::BoundCallExpression(c) => {
                boundRight_type = (*c).Type();
            },
            BoundNode::BoundConversionExpression(c) => {
                boundRight_type = (*c).Type;
            },
        }

        let boundOperator = BoundBinaryOperator::Bind(syntax.OperatorToken.Kind, boundLeft_type.clone(), boundRight_type.clone());

        match boundOperator.clone() {
            Ok(op) => {
                return BoundNode::BoundBinaryExpression(
                    Box::new(BoundBinaryExpression::new(boundLeft, op, boundRight))
                );
            },
            Err(_) => {
                self.diagnostics.ReportUndefinedBinaryOperator(syntax.OperatorToken.Span(), syntax.OperatorToken.Text.unwrap(), boundLeft_type, boundRight_type);
                return BoundNode::BoundErrorExpression(
                    Box::new(BoundErrorExpression::new())
                );
            },
        }
    }

    fn BindCallExpression(&mut self, syntax: CallExpressionSyntax) -> BoundNode {        
        if syntax.Arguments.Count() == 1 {
            match self.LookupType(syntax.Identifier.Text.clone().unwrap()) {
                Some(type_symbol) => {
                    match syntax.Arguments.Index(0) {
                        ListValueType::SyntaxNode(node) => return self.BindConversion(type_symbol, *node),
                        _ => {}
                    }
                },
                None => {}
            }
        }

        let mut boundArguments: Vec<BoundNode> = Vec::new();

        for argument in syntax.Arguments.nodesAndSeparators.iter() {
            match argument {
                ListValueType::SyntaxNode(argu) => {
                    let boundArgument = self.BindExpression(*argu.clone(), false);
                    boundArguments.push(boundArgument);
                }
                _ => {}
            }
        }

        let mut function: FunctionSymbol = FunctionSymbol::new(String::from("zz"), Vec::new(), TypeSymbol::Void);
        match self.scope.TryLookupFunction(syntax.Identifier.Text.clone().unwrap()) {
            Ok(func) => {
                let _ = function;
                function = func;
            }
            Err(_) => {
                self.diagnostics.ReportUndefinedFunction(syntax.Identifier.Span(), syntax.clone().Identifier.Text.unwrap());
                return BoundNode::BoundErrorExpression(
                    Box::new(BoundErrorExpression::new())
                );
            }
        }

        if syntax.Arguments.Count() != function.Parameter.len() as i32 {
            self.diagnostics.ReportWrongArgumentCount(syntax.Span(), function.Name, function.Parameter.len() as i32, syntax.Arguments.Count());
            return BoundNode::BoundErrorExpression(
                Box::new(BoundErrorExpression::new())
            );
        }

        for i in 0..syntax.Arguments.Count() {
            let argument = boundArguments[i as usize].clone();
            let parameter = function.Parameter[i as usize].clone();

            let argument_type: TypeSymbol;
            match argument {
                BoundNode::BoundErrorExpression(e) => {
                    argument_type = (*e).Type();
                },
                BoundNode::BoundAssignmentExpression(a) => {
                    argument_type = a.Type();
                }
                BoundNode::BoundBinaryExpression(b) => {
                    argument_type = b.Type();
                }
                BoundNode::BoundLiteralExpression(l) => {
                    argument_type = l.Type();
                }
                BoundNode::BoundUnaryExpression(u) => {
                    argument_type = u.Type();
                }
                BoundNode::BoundVariableExpression(v) => {
                    argument_type = v.Type();
                }
                BoundNode::BoundCallExpression(c) => {
                    argument_type = c.Type();
                }
                BoundNode::BoundConversionExpression(c) => {
                    argument_type = c.Type;
                }
            }
            if argument_type != parameter.Type
            {
                self.diagnostics.ReportWrongArgumentType(syntax.Span(), parameter.Name, parameter.Type, argument_type);
                return BoundNode::BoundErrorExpression(
                    Box::new(BoundErrorExpression::new())
                );
            }
        }

        return BoundNode::BoundCallExpression(
            Box::new(BoundCallExpression::new(function, boundArguments))
        ) ;
    }

    fn BindConversion(&mut self, type_symbol: TypeSymbol, syntax: SyntaxNode) -> BoundNode {
        let expression = self.BindExpression(syntax.clone(), false);

        let syntax_span: TextSpan;
        match syntax {
            SyntaxNode::AssignmentExpressionSyntax(a) => 
                syntax_span = a.Span(),
            SyntaxNode::BinaryExpressionSyntax(b) => 
                syntax_span = b.Span(),
            SyntaxNode::NameExpressionSyntax(n) => 
                syntax_span = n.Span(),
            SyntaxNode::LiteralExpressionSyntax(l) => 
                syntax_span = l.Span(),
            SyntaxNode::ParenthesizedExpressionSyntax(p) => 
                syntax_span = p.Span(),
            SyntaxNode::UnaryExpressionSyntax(u) => 
                syntax_span = u.Span(),
            SyntaxNode::CallExpressionSyntax(_) =>     // fake, not support for CallExpressionSyntax
                syntax_span = TextSpan::new(0, 0)
        }
        let expression_type;
        match expression.clone() {
            BoundNode::BoundErrorExpression(e) => {
                expression_type = (*e).Type();
            },
            BoundNode::BoundAssignmentExpression(a) => {
                expression_type = a.Type();
            }
            BoundNode::BoundBinaryExpression(b) => {
                expression_type = b.Type();
            }
            BoundNode::BoundLiteralExpression(l) => {
                expression_type = l.Type();
            }
            BoundNode::BoundUnaryExpression(u) => {
                expression_type = u.Type();
            }
            BoundNode::BoundVariableExpression(v) => {
                expression_type = v.Type();
            }
            BoundNode::BoundCallExpression(c) => {
                expression_type = c.Type();
            }
            BoundNode::BoundConversionExpression(c) => {
                expression_type = c.Type;
            }
        }

        let conversion = Conversion::Classify(expression_type.clone(), type_symbol.clone());
        if !conversion.Exists {
            self.diagnostics.ReportCannotConvert(syntax_span, expression_type, type_symbol);
            return BoundNode::BoundErrorExpression(
                Box::new(BoundErrorExpression::new())
            );
        }

        return BoundNode::BoundConversionExpression(
            Box::new(BoundConversionExpression::new(type_symbol, expression))
        );
    }

    fn BindVariable(&mut self, identifier: SyntaxToken, isReadOnly: bool, vari_type: TypeSymbol) -> VariableSymbol {
        let name: String;
        match identifier.Text.clone() {
            Some(text) => name = text,
            None => name = String::from("?")
        }
        let declare = !identifier.IsMissing();
        let variable = VariableSymbol::new(name.clone(), isReadOnly, vari_type);

        if declare && !self.scope.TryDeclareVariable(variable.clone()) {
            self.diagnostics.ReportVariableAlreadyDeclared(identifier.Span(), name);
        }

        return variable;
    }

    fn LookupType(&self, name: String) -> Option<TypeSymbol> {
        match name.as_str() {
            "bool" => {
                return Some(TypeSymbol::Bool);
            }
            "int" => {
                return Some(TypeSymbol::Int);
            }
            "string" => {
                return Some(TypeSymbol::String);
            }
            _ => {
                return None;
            }
        }
    }
}