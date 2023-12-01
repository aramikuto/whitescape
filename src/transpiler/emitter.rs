use crate::{
    ast::{Expression, Operation},
    specs::whitespace::*,
};

use super::{state, CodeOutput, DebugCodeBlock, HeapVariableSize, VariableType};

struct EvaluationResult {
    code: String,
    debug_code: DebugCodeBlock,
    type_: VariableType,
}

pub struct CodeEmitter {}

impl CodeEmitter {
    pub fn emit(&mut self, expression: Vec<IMP>) -> CodeOutput {
        let mut res = CodeOutput::new();
        for imp in expression {
            let CodeOutput { code, debug_code } = imp.gen();
            res.add(code, debug_code);
        }
        res
    }

    // TODO: Calculation can leave a value on the stack, which might not be desirable
    // Need to implement a way to keep stack clean
    pub fn evaluate_expression(
        &mut self,
        expression: &Expression,
        heap_allocation_map: &state::HeapAllocation,
        level: i8,
    ) -> (String, DebugCodeBlock, VariableType, i32) {
        let mut code = String::new();
        let mut debug_code = DebugCodeBlock::new(Some(expression.to_string()), level);
        match expression {
            Expression::Integer(value) => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Stack(StackOperations::PushNumber(*value)).gen();
                code.push_str(&c);
                debug_code.push(dc);
                (
                    code,
                    debug_code,
                    VariableType::Int,
                    HeapVariableSize::Int.size(),
                )
            }
            Expression::Variable(name) => {
                let variable = heap_allocation_map.get(name).unwrap();
                let type_ = variable.type_();
                let offset = variable.offset();
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Stack(StackOperations::PushNumber(offset)).gen();
                code.push_str(&c);
                debug_code.push(dc);
                match type_ {
                    VariableType::Int => {
                        let CodeOutput {
                            code: c2,
                            debug_code: dc2,
                        } = IMP::Heap(HeapOperations::Retrieve).gen();
                        code.push_str(&c2);
                        debug_code.push(dc2);
                        (
                            code,
                            debug_code,
                            VariableType::Int,
                            HeapVariableSize::Int.size(),
                        )
                    }
                    _ => {
                        panic!("Only integer values are supported for now");
                    }
                }
            }
            Expression::BinaryOp {
                operator,
                left,
                right,
            } => {
                let EvaluationResult {
                    code: c,
                    debug_code: dc,
                    ..
                } = self.evaluate_binary_op(operator, left, right, heap_allocation_map, level);
                code.push_str(&c);
                debug_code.append(dc);
                (
                    code,
                    debug_code,
                    VariableType::Int,
                    HeapVariableSize::Int.size(),
                )
            }
            _ => {
                panic!("Unsupported expression");
            }
        }
    }

    fn evaluate_binary_op(
        &mut self,
        operator: &Operation,
        left: &Expression,
        right: &Expression,
        heap_allocation_map: &state::HeapAllocation,
        level: i8,
    ) -> EvaluationResult {
        let mut code = String::new();
        let mut debug_code = DebugCodeBlock::new(None, level);

        let (left_code, left_debug_code, left_type, left_size) =
            self.evaluate_expression(left, heap_allocation_map, level + 1);
        let (right_code, right_debug_code, right_type, right_size) =
            self.evaluate_expression(right, heap_allocation_map, level + 1);
        code.push_str(&left_code);
        code.push_str(&right_code);
        debug_code.append(left_debug_code);
        debug_code.append(right_debug_code);

        match operator {
            Operation::Add => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Arithmetic(ArithmeticOperations::Add).gen();
                code.push_str(&c);
                debug_code.push(dc);
                EvaluationResult {
                    code,
                    debug_code,
                    type_: VariableType::Int,
                }
            }
            Operation::Sub => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                code.push_str(&c);
                debug_code.push(dc);
                EvaluationResult {
                    code,
                    debug_code,
                    type_: VariableType::Int,
                }
            }
            Operation::Mul => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Arithmetic(ArithmeticOperations::Multiply).gen();
                code.push_str(&c);
                debug_code.push(dc);
                EvaluationResult {
                    code,
                    debug_code,
                    type_: VariableType::Int,
                }
            }
            Operation::Div => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Arithmetic(ArithmeticOperations::DivideInteger).gen();
                code.push_str(&c);
                debug_code.push(dc);
                EvaluationResult {
                    code,
                    debug_code,
                    type_: VariableType::Int,
                }
            }
            Operation::Mod => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Arithmetic(ArithmeticOperations::Modulo).gen();
                code.push_str(&c);
                debug_code.push(dc);
                EvaluationResult {
                    code,
                    debug_code,
                    type_: VariableType::Int,
                }
            }
            _ => {
                panic!("Unsupported operator");
            }
        }
    }
}
