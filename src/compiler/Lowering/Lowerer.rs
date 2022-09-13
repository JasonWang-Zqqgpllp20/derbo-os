#![allow(non_snake_case)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

use super::super::Binding::BoundTreeRewriter::BoundTreeRewriter;
use super::super::Binding::BoundLabel::BoundLabel;
use super::super::Binding::BoundStatement::{BoundStatement, BoundVariableDeclaration};
use super::super::Binding::BoundBlockStatement::BoundBlockStatement;
use super::super::Binding::BoundIfStatement::BoundIfStatement;
use super::super::Binding::BoundForStatement::BoundForStatement;
use super::super::Binding::BoundWhileStatement::BoundWhileStatement;
use super::super::Binding::BoundConditionalGotoStatement::BoundConditionalGotoStatement;
use super::super::Binding::BoundGotoStatement::BoundGotoStatement;
use super::super::Binding::BoundLabelStatement::BoundLabelStatement;
use super::super::Binding::BoundVariableExpression::BoundVariableExpression;
use super::super::Symbol::TypeSymbol::TypeSymbol;
use super::super::Binding::BoundBinaryExpression::BoundBinaryExpression;
use super::super::Syntax::SyntaxKind::SyntaxKind;
use super::super::Binding::BoundBinaryOperator::BoundBinaryOperator;
use super::super::Binding::BoundNode::BoundNode;
use super::super::Binding::BoundExpressionStatement::BoundExpressionStatement;
use super::super::Binding::BoundAssignmentExpression::BoundAssignmentExpression;
use super::super::Binding::BoundLiteralExpression::BoundLiteralExpression;
use super::super::Syntax::ValueType::ValueType;
use super::super::Binding::BoundUnaryExpression::BoundUnaryExpression;
use super::super::Binding::BoundErrorExpression::BoundErrorExpression;
use super::super::Binding::BoundCallExpression::BoundCallExpression;
use super::super::Binding::BoundConversionExpression::BoundConversionExpression;


#[derive(Clone, Debug, PartialEq)]
pub struct Lowerer {
    labelCount: u8,
    rewriter: BoundTreeRewriter,    // the same as abstract func in C#
}

impl Lowerer {
    pub fn new() -> Lowerer {
        Lowerer {
            labelCount: 0,
            rewriter: BoundTreeRewriter {},
        }
    }

    pub fn GenerateLabel(&mut self) -> BoundLabel {
        let mut name = String::from("Label");
        self.labelCount += 1;
        name.push((self.labelCount + 48) as char);     // int2str?????
        BoundLabel::new(name)
    }

    pub fn Lower(&self, statement: BoundStatement) -> BoundBlockStatement {
        let mut lowerer = Lowerer::new();
        let result = lowerer.RewriteStatement(statement);
        let flatten = self.Flatten(result);
        
        return flatten;
    }

    pub fn Flatten(&self,statement: BoundStatement) -> BoundBlockStatement {
        let mut builder: Vec<BoundStatement> = Vec::new();
        let mut stack: Vec<BoundStatement> = Vec::new();
        stack.push(statement);

        while stack.len() > 0 {
            let current = stack.pop().unwrap();
            
            match current {
                BoundStatement::BoundBlockStatement(block) => {
                    let len = block.Statements.len();
                    for i in 0..len {
                        stack.push(block.Statements[len - 1 - i].clone());      // correct???????????
                    }
                },
                _ => {
                    builder.push(current);
                }
            }
        }

        return BoundBlockStatement::new(builder);
    }
    
    pub fn RewriteStatement(&mut self, node: BoundStatement) -> BoundStatement {
        match node {
            BoundStatement::BoundBlockStatement(b) => {
                return self.RewriteBlockStatement(*b);
            }
            BoundStatement::BoundVariableDeclaration(v) => {
                return self.RewriteVariableDeclaration(*v);
            }
            BoundStatement::BoundIfStatement(i) => {
                return self.RewriteIfStatement(*i);
            }
            BoundStatement::BoundWhileStatement(w) => {
                return self.RewriteWhileStatement(*w);
            } 
            BoundStatement::BoundForStatement(f) => {
                return self.RewriteForStatement(*f);
            }
            BoundStatement::BoundLabelStatement(l) => {
                return self.RewriteLabelStatement(*l);
            }
            BoundStatement::BoundGotoStatement(g) => {
                return self.RewriteGotoStatement(*g);
            }  
            BoundStatement::BoundConditionalGotoStatement(c) => {
                return self.RewriteConditionalGotoStatement(*c);
            }  
            BoundStatement::BoundExpressionStatement(e) => {
                return self.RewriteExpressionStatement(*e);
            }
            // no need to suppor for BoundGlobalScope
            // _ => {
            //     return BoundStatement::BoundBlockStatement(Box::new(        // fake
            //         BoundBlockStatement {
            //             Statements: Vec::new(),
            //         }
            //     ))
            // } 
        }
    }

    fn RewriteIfStatement(&mut self, node: BoundIfStatement) -> BoundStatement {
        match node.ElseStatement {
            Some(else_statement) => {
                let elseLabel = self.GenerateLabel();
                let endLabel = self.GenerateLabel();

                let gotoFalse = BoundConditionalGotoStatement::new(elseLabel.clone(), node.Condition, true);
                let gotoEndStatement = BoundGotoStatement::new(endLabel.clone());
                let elseLabelStatement = BoundLabelStatement::new(elseLabel);
                let endLabelStatement = BoundLabelStatement::new(endLabel);

                let mut statements = Vec::new();
                statements.push(BoundStatement::BoundConditionalGotoStatement(
                    Box::new(gotoFalse)
                ));
                statements.push(node.ThenStatement);
                statements.push(BoundStatement::BoundGotoStatement(
                    Box::new(gotoEndStatement)
                ));
                statements.push(BoundStatement::BoundLabelStatement(
                    Box::new(elseLabelStatement)
                ));
                statements.push(else_statement);
                statements.push(BoundStatement::BoundLabelStatement(
                    Box::new(endLabelStatement)
                ));

                let result = BoundStatement::BoundBlockStatement(
                    Box::new(BoundBlockStatement::new(statements))
                );

                return self.RewriteStatement(result);
            }
            None => {
                let endLabel = self.GenerateLabel();
                let gotoFalse = BoundConditionalGotoStatement::new(endLabel.clone(), node.Condition, true);
                let endLabelStatement = BoundLabelStatement::new(endLabel);

                let mut statements: Vec<BoundStatement> = Vec::new();
                statements.push(BoundStatement::BoundConditionalGotoStatement(
                    Box::new(gotoFalse)
                ));
                statements.push(node.ThenStatement);
                statements.push(BoundStatement::BoundLabelStatement(
                    Box::new(endLabelStatement)
                ));

                let result = BoundStatement::BoundBlockStatement(
                    Box::new(BoundBlockStatement::new(statements))
                );

                return self.RewriteStatement(result);
            }
        }
    }

    fn RewriteWhileStatement(&mut self, node: BoundWhileStatement) -> BoundStatement {
        // while <condition>
        //      <bode>
        // ----->
        // goto check
        // continue:
        // <body>
        // check:
        // gotoTrue <condition> continue
        // end:
        let continueLabel = self.GenerateLabel();
        let checkLabel = self.GenerateLabel();
        let endLabel = self.GenerateLabel();

        let gotoCheck = BoundGotoStatement::new(checkLabel.clone());
        let continueLabelStatement = BoundLabelStatement::new(continueLabel.clone());
        let checkLabelStatement = BoundLabelStatement::new(checkLabel);
        let gotoTrue = BoundConditionalGotoStatement::new(continueLabel, node.Condition, false);
        let endLabelStatement = BoundLabelStatement::new(endLabel);

        let mut statements: Vec<BoundStatement> = Vec::new();
        statements.push(BoundStatement::BoundGotoStatement(
            Box::new(gotoCheck)
        ));
        statements.push(BoundStatement::BoundLabelStatement(
            Box::new(continueLabelStatement)
        ));
        statements.push(node.Body);
        statements.push(BoundStatement::BoundLabelStatement(
            Box::new(checkLabelStatement)
        ));
        statements.push(BoundStatement::BoundConditionalGotoStatement(
            Box::new(gotoTrue)
        ));
        statements.push(BoundStatement::BoundLabelStatement(
            Box::new(endLabelStatement)
        ));

        let result = BoundStatement::BoundBlockStatement(
            Box::new(BoundBlockStatement::new(statements))
        );

        return self.rewriter.RewriteStatement(result);
    }

    fn RewriteForStatement(&mut self, node: BoundForStatement) -> BoundStatement {
        // for <var> = <lower> to <upper>
        //      <body>
        // ---->
        // {
        //      var <var> = <lower>
        //      while (<var> <= <upper>)
        //      {
        //          <body>
        //          <var> = <var> + 1
        //      }   
        // }
        let variableDeclaration = BoundVariableDeclaration::new(node.Variable.clone(), node.LowerBound);
        let variableExpression = BoundVariableExpression::new(node.Variable.clone());
        let condition = BoundBinaryExpression::new(
            BoundNode::BoundVariableExpression(Box::new(variableExpression.clone())),
            BoundBinaryOperator::Bind(SyntaxKind::LessOrEqualsToken, TypeSymbol::Int, TypeSymbol::Int).unwrap(),
            node.UpperBound
        );

        let increment = BoundExpressionStatement::new(
            BoundNode::BoundAssignmentExpression(Box::new(
                BoundAssignmentExpression::new(
                    node.Variable,
                    BoundNode::BoundBinaryExpression(Box::new(
                        BoundBinaryExpression::new(
                            BoundNode::BoundVariableExpression(Box::new(variableExpression)),
                            BoundBinaryOperator::Bind(SyntaxKind::PlusToken, TypeSymbol::Int, TypeSymbol::Int).unwrap(),
                            BoundNode::BoundLiteralExpression(Box::new(
                                BoundLiteralExpression::new(ValueType::Int32(1))
                            )) 
                        )
                    )) 
                )
            )
            )
        );
        

        let mut statements: Vec<BoundStatement> = Vec::new();
        statements.push(node.Body);
        statements.push(BoundStatement::BoundExpressionStatement(
            Box::new(increment)
        ));
        let whileBody = BoundBlockStatement::new(statements);
        let whileStatement = BoundWhileStatement::new(
            BoundNode::BoundBinaryExpression(Box::new(
                condition
            )),
            BoundStatement::BoundBlockStatement(Box::new(
                whileBody
            ))
        );

        let mut statements: Vec<BoundStatement> = Vec::new();
        statements.push(BoundStatement::BoundVariableDeclaration(
            Box::new(variableDeclaration)
        ));
        statements.push(BoundStatement::BoundWhileStatement(
            Box::new(whileStatement)
        ));

        let result = BoundStatement::BoundBlockStatement(
            Box::new(BoundBlockStatement::new(statements))
        );

        return self.RewriteStatement(result);
    }

    fn RewriteBlockStatement(&mut self, node: BoundBlockStatement) -> BoundStatement {
        let mut builder: Option<Vec<BoundStatement>> = None;

        for i in 0..node.Statements.len() {
            let oldStatement = node.Statements[i].clone();
            let newStatement = self.RewriteStatement(oldStatement.clone());
            if newStatement != oldStatement {
                match builder {
                    Some(_) => {},
                    None => {
                        let mut v = Vec::new();
                        for j in 0..i {
                            v.push(node.Statements[j].clone());
                        } 
                        builder = Some(v);
                    }
                }
            }

            match builder.clone() {
                Some(mut b) => {
                    b.push(newStatement); 
                    builder = Some(b);
                },
                None => {}
            }
        }

        match builder {
            Some(_) => {},
            None => {
                return BoundStatement::BoundBlockStatement(
                    Box::new(node)
                );
            }
        }

        return BoundStatement::BoundBlockStatement(
            Box::new(BoundBlockStatement::new(builder.unwrap()))
        );
    }

    fn RewriteVariableDeclaration(&self, node: BoundVariableDeclaration) -> BoundStatement {
        let initializer = self.RewriteExpression(node.Initializer.clone());
        if initializer == node.Initializer {
            return BoundStatement::BoundVariableDeclaration(
                Box::new(node)
            );
        }

        return BoundStatement::BoundVariableDeclaration(
            Box::new(BoundVariableDeclaration::new(node.Variable, initializer))
        );
    }

    fn RewriteLabelStatement(&self, node: BoundLabelStatement) -> BoundStatement {
        return BoundStatement::BoundLabelStatement(
            Box::new(node)
        );
    }

    fn RewriteGotoStatement(&self, node: BoundGotoStatement) -> BoundStatement {
        return BoundStatement::BoundGotoStatement(
            Box::new(node)
        );
    }

    fn RewriteConditionalGotoStatement(&self, node: BoundConditionalGotoStatement) -> BoundStatement {
        let condition = self.RewriteExpression(node.Condition.clone());
        if condition == node.Condition {
            return BoundStatement::BoundConditionalGotoStatement(
                Box::new(node)
            );
        }
        
        return BoundStatement::BoundConditionalGotoStatement(
            Box::new(BoundConditionalGotoStatement::new(node.Label, condition, node.JumpIfFalse))
        );
    }

    fn RewriteExpressionStatement(&self, node: BoundExpressionStatement) -> BoundStatement {
        let expression = self.RewriteExpression(node.Expression.clone());
        if expression == node.Expression {
            return BoundStatement::BoundExpressionStatement(
                Box::new(node)
            );
        }
        
        return BoundStatement::BoundExpressionStatement(
            Box::new(BoundExpressionStatement::new(expression))
        );
    }

    pub fn RewriteExpression(&self, node: BoundNode) -> BoundNode {
        match node {
            BoundNode::BoundErrorExpression(e) => {
                return self.RewriteErrorExpression(*e);
            }
            BoundNode::BoundLiteralExpression(l) => {
                return self.RewriteLiteralExpression(*l);
            }
            BoundNode::BoundVariableExpression(v) => {
                return self.RewriteVariableExpression(*v);
            }
            BoundNode::BoundAssignmentExpression(a) => {
                return self.RewriteAssignmentExpression(*a);
            }
            BoundNode::BoundUnaryExpression(u) => {
                return self.RewriteUnaryExpression(*u);
            }
            BoundNode::BoundBinaryExpression(b) => {
                return self.RewriteBinaryExpression(*b);
            }
            BoundNode::BoundCallExpression(c) => {
                return self.RewriteCallExpression(*c);
            }
            BoundNode::BoundConversionExpression(c) => {
                return self.RewriteConversionExpression(*c);
            }
        }
    }

    fn RewriteErrorExpression(&self, node: BoundErrorExpression) -> BoundNode {
        return BoundNode::BoundErrorExpression(
            Box::new(node)
        )
    }

    fn RewriteLiteralExpression(&self, node: BoundLiteralExpression) -> BoundNode {
        return BoundNode::BoundLiteralExpression(
            Box::new(node)
        )
    }

    fn RewriteVariableExpression(&self, node: BoundVariableExpression) -> BoundNode {
        return BoundNode::BoundVariableExpression(
            Box::new(node)
        )
    }

    fn RewriteAssignmentExpression(&self, node: BoundAssignmentExpression) -> BoundNode {
        let expression = self.RewriteExpression(node.Expression.clone());
        if expression == node.Expression {
            return  BoundNode::BoundAssignmentExpression(
                Box::new(node)
            );
        }
        
        return  BoundNode::BoundAssignmentExpression(
            Box::new(BoundAssignmentExpression::new(node.Variable, expression))
        );
    }

    fn RewriteUnaryExpression(&self, node: BoundUnaryExpression) -> BoundNode {
        let operand = self.RewriteExpression(node.Operand.clone());
        if operand == node.Operand {
            return BoundNode::BoundUnaryExpression(
                Box::new(node)
            );
        }
        
        return BoundNode::BoundUnaryExpression(
            Box::new(BoundUnaryExpression::new(node.Op, operand))
        );
    }

    fn RewriteBinaryExpression(&self, node: BoundBinaryExpression) -> BoundNode {
        let left = self.RewriteExpression(node.Left.clone());
        let right = self.RewriteExpression(node.Right.clone());
        if left == node.Left && right == node.Right {
            return BoundNode::BoundBinaryExpression(
                Box::new(node)
            );
        }
        
        return BoundNode::BoundBinaryExpression(
            Box::new(BoundBinaryExpression::new(left, node.Op, right))
        );
    }

    fn RewriteCallExpression(&self, node: BoundCallExpression) -> BoundNode {
        let mut builder: Option<Vec<BoundNode>> = None;

        for i in 0..node.Arguments.len() {
            let oldArgument = node.Arguments[i].clone();
            let newArgument = self.RewriteExpression(oldArgument.clone());
            if newArgument != oldArgument {
                match builder {
                    Some(_) => {},
                    None => {
                        let mut v = Vec::new();
                        for j in 0..i {
                            v.push(node.Arguments[j].clone());
                        } 
                        builder = Some(v);
                    }
                }
            }

            match builder.clone() {
                Some(mut b) => {
                    b.push(newArgument); 
                    builder = Some(b);
                },
                None => {}
            }
        }

        match builder {
            Some(_) => {},
            None => {
                return BoundNode::BoundCallExpression(
                    Box::new(node)
                );
            }
        }

        return BoundNode::BoundCallExpression(
            Box::new(BoundCallExpression::new(node.Function, builder.unwrap()))
        );
    }

    fn RewriteConversionExpression(&self, node: BoundConversionExpression) -> BoundNode {
        let expression = self.RewriteExpression(node.Expression.clone());
        if expression == node.Expression {
            return BoundNode::BoundConversionExpression(
                Box::new(node)
            );
        }

        return BoundNode::BoundConversionExpression(
            Box::new(BoundConversionExpression::new(node.Type, expression))
        )
    }
}