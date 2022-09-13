#![allow(non_snake_case)]
#[warn(unused_imports)]

use crate::alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use super::Binding::BoundUnaryOperatorKind::BoundUnaryOperatorKind;
use super::Binding::BoundBinaryOperatorKind::BoundBinaryOperatorKind;
use super::Syntax::ValueType::ValueType;
use super::Binding::BoundNode::BoundNode;
use super::Symbol::VariableSymbol::{VariableSymbol, VariableSymbolDictionary};
use super::Binding::BoundLiteralExpression::BoundLiteralExpression;
use super::Binding::BoundAssignmentExpression::BoundAssignmentExpression;
use super::Binding::BoundBinaryExpression::BoundBinaryExpression;
use super::Binding::BoundVariableExpression::BoundVariableExpression;
use super::Binding::BoundUnaryExpression::BoundUnaryExpression;
use super::Binding::BoundStatement::{BoundStatement, BoundVariableDeclaration};
use super::Binding::BoundBlockStatement::BoundBlockStatement;
use super::Binding::BoundExpressionStatement::BoundExpressionStatement;
use super::Symbol::TypeSymbol::TypeSymbol;
use super::Binding::BoundLabel::BoundLabel;
use super::Binding::BoundCallExpression::BoundCallExpression;
use super::Binding::BoundConversionExpression::BoundConversionExpression;
use super::Symbol::BuiltinFunctions::BuiltinFunctions;

#[derive(Clone, Debug, PartialEq)]
pub struct Evaluator {
    root: BoundBlockStatement,
    variables: Vec<VariableSymbolDictionary>,
    lastValue: ValueType,
}

impl Evaluator {
    pub fn new(node: BoundBlockStatement, variables: Vec<VariableSymbolDictionary>) -> Evaluator {
        Evaluator {
            root: node,
            variables: variables,
            lastValue: ValueType::Null,
        }
    }

    pub fn Evaluate(&mut self) -> (Result<ValueType, ()>, Vec<VariableSymbolDictionary>) {
        let mut labelToIndex: Vec<(BoundLabel, i32)> = Vec::new();

        for i in 0..self.root.Statements.len() {
            match self.root.Statements[i].clone() {
                BoundStatement::BoundLabelStatement(l) => {
                    labelToIndex.push((l.Label, i as i32 + 1))
                }
                _ => {}
            }
        }

        let mut index = 0;

        while index < self.root.Statements.len() {
            let s = self.root.Statements[index].clone();

            match s {
                BoundStatement::BoundVariableDeclaration(v) => {
                    self.EvaluateVariableDeclaration(*v);
                    index += 1;
                }
                BoundStatement::BoundExpressionStatement(e) => {
                    self.EvaluateExpressionStatement(*e);
                    index += 1;
                } 
                BoundStatement::BoundGotoStatement(g) => {
                    let gs = *g;
                    for label in &labelToIndex {
                        if label.0 == gs.Label {
                            index = label.1 as usize;
                        }
                    }
                }
                BoundStatement::BoundConditionalGotoStatement(c) => {
                    let cgs = *c;
                    let cond = self.EvaluateExpression(cgs.Condition).unwrap();
                    let mut condition: bool = true;
                    match cond {
                        ValueType::Bool(b) => condition = b,
                        _ => {}
                    }
                    if condition && !cgs.JumpIfFalse || !condition && cgs.JumpIfFalse {
                        for label in &labelToIndex {
                            if label.0 == cgs.Label {
                                index = label.1 as usize;
                            }
                        }
                    } else {
                        index += 1;
                    }
                }
                BoundStatement::BoundLabelStatement(_) => {
                    index += 1;
                }
                _ => {
                    
                }
            }

        }
        // (Err(()), Vec::new())
        (Ok(self.lastValue.clone()), self.variables.clone())
    }

    pub fn EvaluateVariableDeclaration(&mut self, node: BoundVariableDeclaration) {
        let value = self.EvaluateExpression(node.Initializer);
        let mut find = -1;
        let mut symbol: VariableSymbol =  VariableSymbol {
            Name: String::from(node.Variable.Name.clone()),
            IsReadOnly: false,
            Type: TypeSymbol::Error,
        };
        for (i, vari) in self.variables.iter().enumerate() {
            if vari.Key == node.Variable {
                find = i as i32;
                symbol = node.Variable;
                break;
            }
        }

        if find == -1 {
            self.variables.push(
                VariableSymbolDictionary::new(symbol, value.clone().unwrap())
            );
        }

        self.lastValue = value.unwrap();
    }
    
    pub fn EvaluateExpressionStatement(&mut self, node: BoundExpressionStatement) {
        match self.EvaluateExpression(node.Expression) {
            Ok(v) => self.lastValue = v,
            Err(_) => self.lastValue = ValueType::Null,
        }
    }

    pub fn EvaluateExpression(&mut self, node: BoundNode) -> Result<ValueType, ()> {
        match node {
            BoundNode::BoundLiteralExpression(l) => {
                return self.EvaluateLiteralExpression(*l);
            },
            BoundNode::BoundVariableExpression(v) => {
                return self.EvaluateVariableExpression(*v);
            },
            BoundNode::BoundAssignmentExpression(a) => {
                return self.EvaluateAssignmentExpression(*a);
            },
            BoundNode::BoundUnaryExpression(u) => {
                return self.EvaluateUnaryExpression(*u);
            },
            BoundNode::BoundBinaryExpression(b) => {
                return self.EvaluateBinaryExpression(*b);
            },
            BoundNode::BoundCallExpression(c) => {
                return self.EvaluateCallExpression(*c);
            },
            BoundNode::BoundConversionExpression(c) => {
                return self.EvaluateConversionExpression(*c);
            },
            _ => {
                Err(())     // BoundErrorExpression
            }
        }
    }

    fn EvaluateLiteralExpression(&self, l: BoundLiteralExpression) -> Result<ValueType, ()> {
        match l.Value {
            ValueType::Int32(i) => return Ok(ValueType::Int32(i)),
            ValueType::Bool(b) => return Ok(ValueType::Bool(b)),
            ValueType::String(s) => return Ok(ValueType::String(s)),
            _ => return Err(())
        }
    }

    fn EvaluateVariableExpression(&self, v: BoundVariableExpression) -> Result<ValueType, ()> {
        for vari in self.variables.clone() {
            if v.Variable.Name == vari.Key.Name {
                return Ok(vari.Value)
            }
        }
        return Err(())
    }

    fn EvaluateAssignmentExpression(&mut self, a: BoundAssignmentExpression) -> Result<ValueType, ()> {
        let value = self.EvaluateExpression(a.Expression);
        let vd = VariableSymbolDictionary { Key: a.Variable.clone(), Value: value.clone().unwrap() };
        
        for (i, vari) in self.variables.clone().iter().enumerate() {
            if a.Variable.Name == vari.Key.Name {
                self.variables[i] = vd.clone();
                return Ok(vd.Value);
            }
        }
        // no exsiting variable has the same name
        self.variables.push(vd);
        return value;
    }

    fn EvaluateUnaryExpression(&mut self, u: BoundUnaryExpression) -> Result<ValueType, ()> {
        let operand = self.EvaluateExpression(u.Operand).unwrap();
        let operator = u.Op;
        match operator.Kind {
            BoundUnaryOperatorKind::Identity => {
                match operand {
                    ValueType::Int32(i) => return Ok(ValueType::Int32(i)),
                    _ => return Err(())
                }
            },
            BoundUnaryOperatorKind::Negation => {
                match operand {
                    ValueType::Int32(i) => return Ok(ValueType::Int32(-i)),
                    _ => return Err(())
                }
            },
            BoundUnaryOperatorKind::LogicalNegation => {
                match operand {
                    ValueType::Bool(b) => return Ok(ValueType::Bool(!b)),
                    _ => return Err(())
                }
            },
            BoundUnaryOperatorKind::OnesComplement => {
                match operand {
                    ValueType::Int32(b) => return Ok(ValueType::Int32(!b)),
                    _ => return Err(())
                }
            },
            // _ => return Err(())
        }
    }

    fn EvaluateBinaryExpression(&mut self, b: BoundBinaryExpression) -> Result<ValueType, ()> {
        let left = self.EvaluateExpression(b.Left).unwrap();
        let right = self.EvaluateExpression(b.Right).unwrap();
        let operator = b.Op;

        match operator.Kind {
            BoundBinaryOperatorKind::Addition => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l + r)),
                            _ => Err(())
                        }
                    },
                    ValueType::String(l) => {
                        match right {
                            ValueType::String(r) => return Ok(ValueType::String({
                                    let mut s = l;
                                    s.push_str(r.as_str());
                                    s
                                })),
                            _ => Err(())
                        }
                    }
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Subtraction => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l - r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Multiplication => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l * r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Division => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l / r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::LogicalAnd => {
                match left {
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l && r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::LogicalOr => {
                match left {
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l || r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Equals => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l == r)),
                            _ => Err(())
                        }
                    },
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l == r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::NotEquals => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l == r)),
                            _ => Err(())
                        }
                    },
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l != r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Less => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l < r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::LessOrEquals => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l <= r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::Greater => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l > r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::GreaterOrEquals => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Bool(l >= r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            },
            BoundBinaryOperatorKind::BitwiseAnd => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l & r)),
                            _ => Err(())
                        }
                    },
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l & r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            }
            BoundBinaryOperatorKind::BitwiseOr => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l | r)),
                            _ => Err(())
                        }
                    },
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l | r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            }
            BoundBinaryOperatorKind::BitwiseXor => {
                match left {
                    ValueType::Int32(l) => {
                        match right {
                            ValueType::Int32(r) => return Ok(ValueType::Int32(l ^ r)),
                            _ => Err(())
                        }
                    },
                    ValueType::Bool(l) => {
                        match right {
                            ValueType::Bool(r) => return Ok(ValueType::Bool(l ^ r)),
                            _ => Err(())
                        }
                    },
                    _ => return Err(())
                }
            }
            // _ => return Err(())
        }
    }
    
    fn EvaluateCallExpression(&mut self, node: BoundCallExpression) -> Result<ValueType, ()> {
        if node.Function == BuiltinFunctions::Input() {
            
        } else if node.Function == BuiltinFunctions::Print() {
            let para0 = self.EvaluateExpression(node.Arguments[0].clone()).unwrap();
            match para0 {
                ValueType::String(m) => {
                    // use crate::println;
                    // println!("{}", m);
                    use crate::terminal::task1;
                    task1::add_command((String::from("println"), m));
                }
                _ => {}
            }

            return Ok(ValueType::Null);
        } else if node.Function == BuiltinFunctions::Rand() {
            let rand = self.EvaluateExpression(node.Arguments[0].clone()).unwrap();
            use crate::interrupts::TIMER_COUNT;
            unsafe {
                match rand {
                    ValueType::Int32(r) => {
                        let rand_number = TIMER_COUNT % (r as u64 + 1);

                        return Ok(ValueType::Int32(rand_number as i32));
                    }
                    _ => {}
                }
            }
        } else if node.Function == BuiltinFunctions::Sleep() {
            let time = self.EvaluateExpression(node.Arguments[0].clone()).unwrap();
            match time {
                ValueType::Int32(sleep_time) => {
                    // use crate::api::sleep1s;
                    // sleep1s(sleep_time as u64);
                    use crate::terminal::task1;
                    let time = format!("{}", sleep_time);
                    task1::add_command((String::from("sleep"), time));
                }
                _ => {}
            }

            return Ok(ValueType::Null)
        } else if node.Function == BuiltinFunctions::Breakpoint() {
            x86_64::instructions::interrupts::int3();

            // use crate::terminal::task1;
            // for vari in &self.variables {
            //     match &vari.Value {
            //         ValueType::Int32(i) => {
            //             let text = format!("{}: {}", vari.Key.Name, *i);
            //             task1::add_command((String::from("println"), text));
            //         }
            //         ValueType::Bool(b) => {
            //             let text = format!("{}: {}", vari.Key.Name, *b);
            //             task1::add_command((String::from("println"), text));
            //         }
            //         ValueType::String(s) => {
            //             let text = format!("{}: {}", vari.Key.Name, *s);
            //             task1::add_command((String::from("println"), text));
            //         }
            //         ValueType::Null => {
            //             let text = format!("{}: {}", vari.Key.Name, "Null");
            //             task1::add_command((String::from("println"), text));
            //         }
            //         _ => return Err(())
            //     }
            // }
            return Ok(ValueType::Null)
        } else {
            return Err(())
        }

        return Err(())
    }
    
    fn EvaluateConversionExpression(&mut self, node: BoundConversionExpression) -> Result<ValueType, ()> {
        let value = self.EvaluateExpression(node.Expression);
        if node.Type == TypeSymbol::Bool {
            match value.unwrap() {
                ValueType::Bool(b) => {
                    return Ok(ValueType::Bool(b));
                }
                ValueType::String(s) => {
                    if s == String::from("true") {
                        return Ok(ValueType::Bool(true))
                    } else {
                        return Ok(ValueType::Bool(false))

                    }
                }
                _ => {}
            }
        } else if node.Type == TypeSymbol::Int {
            match value.unwrap() {
                ValueType::Int32(i) => {
                    return Ok(ValueType::Int32(i));
                }
                ValueType::String(s) => {
                    let char_vec = s.as_bytes();
                    let mut sum: u64 = 0;
                    let len = char_vec.len();

                    for c in char_vec.iter() {
                        if *c < 48 || *c > 57 {     // not number 0~9.
                            return Err(())
                        }
                    }

                    fn n_e(n: u64) -> u64 {     // calculate n^10
                        let mut ret = 1;
                        for _ in 0..n {
                            ret *= 10;
                        }
                        return ret;
                    }

                    for (i, c) in char_vec.iter().enumerate() {
                        sum += (*c as u64 - 48) * n_e( (len-i-1) as u64);
                    }

                    return Ok(ValueType::Int32(sum as i32));
                }
                _ => {}
            }
        } else if node.Type == TypeSymbol::String {
            match value.unwrap() {
                ValueType::Int32(i) => {
                    return Ok(ValueType::String(i.to_string()));    // alloc::string::ToString
                }
                ValueType::Bool(b) => {
                    if b == true {
                        return Ok(ValueType::String(String::from("true")));
                    } else {
                        return Ok(ValueType::String(String::from("false")));
                    }
                }
                ValueType::String(s) => {
                    return Ok(ValueType::String(s))
                }
                _ => {}
            }
        }
        return Err(())
    }
}