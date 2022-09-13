#![allow(non_snake_case)]

use alloc::vec::Vec;
use alloc::boxed::Box;

use super::BoundStatement::{BoundStatement, BoundVariableDeclaration};
use super::BoundNode::BoundNode;
use super::BoundBlockStatement::BoundBlockStatement;
use super::BoundIfStatement::BoundIfStatement;
use super::BoundWhileStatement::BoundWhileStatement;
use super::BoundForStatement::BoundForStatement;
use super::BoundLabelStatement::BoundLabelStatement;
use super::BoundGotoStatement::BoundGotoStatement;
use super::BoundConditionalGotoStatement::BoundConditionalGotoStatement;
use super::BoundExpressionStatement::BoundExpressionStatement;
use super::BoundLiteralExpression::BoundLiteralExpression;
use super::BoundVariableExpression::BoundVariableExpression;
use super::BoundAssignmentExpression::BoundAssignmentExpression;
use super::BoundUnaryExpression::BoundUnaryExpression;
use super::BoundBinaryExpression::BoundBinaryExpression;
use super::BoundErrorExpression::BoundErrorExpression;
use super::BoundCallExpression::BoundCallExpression;
use super::BoundConversionExpression::BoundConversionExpression;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundTreeRewriter {
}

impl BoundTreeRewriter {
    pub fn RewriteStatement(&self, node: BoundStatement) -> BoundStatement {
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

    fn RewriteBlockStatement(&self, node: BoundBlockStatement) -> BoundStatement {
        let mut builder: Vec<BoundStatement> = Vec::new();

        for i in 0..node.Statements.len() {
            let oldStatement = node.Statements[i].clone();
            let newStatement = self.RewriteStatement(oldStatement.clone());
            if newStatement != oldStatement {
                if builder.len() == 0 {
                    for j in 0..i {
                        builder.push(node.Statements[j].clone());
                    }
                }
            }

            if builder.len() != 0 {
                builder.push(newStatement);
            }
        }

        if builder.len() == 0 {
            return BoundStatement::BoundBlockStatement(
                Box::new(node)
            );
        }

        return BoundStatement::BoundBlockStatement(
            Box::new(BoundBlockStatement::new(builder))
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

    fn RewriteIfStatement(&self, node: BoundIfStatement) -> BoundStatement {
        let condition = self.RewriteExpression(node.Condition.clone());
        let thenStatement = self.RewriteStatement(node.ThenStatement.clone());

        let elseStatement: Option<BoundStatement>;
        match node.ElseStatement.clone() {
            Some(state) => elseStatement = Some(self.RewriteStatement(state)),
            None => elseStatement = None,
        }
        if condition == node.Condition && thenStatement == node.ThenStatement && elseStatement == node.ElseStatement {
            return BoundStatement::BoundIfStatement(
                Box::new(node)
            );
        }

        return BoundStatement::BoundIfStatement(
            Box::new(BoundIfStatement::new(condition, thenStatement, elseStatement))
        );
    }

    fn RewriteWhileStatement(&self, node: BoundWhileStatement) -> BoundStatement {
        let condition = self.RewriteExpression(node.Condition.clone());
        let body = self.RewriteStatement(node.Body.clone());
        if condition == node.Condition && body == node.Body {
            return BoundStatement::BoundWhileStatement(
                Box::new(node)
            );
        }

        return BoundStatement::BoundWhileStatement(
            Box::new(BoundWhileStatement::new(condition, body))
        );
    }

    fn RewriteForStatement(&self, node: BoundForStatement) -> BoundStatement {
        let lowerBound = self.RewriteExpression(node.LowerBound.clone());
        let upperBound = self.RewriteExpression(node.UpperBound.clone());
        let body = self.RewriteStatement(node.Body.clone());
        if lowerBound == node.LowerBound && upperBound == node.UpperBound && body == node.Body {
            return BoundStatement::BoundForStatement(
                Box::new(node)
            );
        }
        
        return BoundStatement::BoundForStatement(
            Box::new(BoundForStatement::new(node.Variable, lowerBound, upperBound, body))
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