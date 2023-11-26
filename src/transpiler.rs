use std::collections::HashMap;

use crate::ast::Expression;
use crate::ast::Operation;
use crate::ast::Statement;

use crate::specs::whitespace::ArithmeticOperations;
use crate::specs::whitespace::FlowControlOperations;
use crate::specs::whitespace::HeapOperations;
use crate::specs::whitespace::IOOperations;
use crate::specs::whitespace::StackOperations;
use crate::specs::whitespace::IMP;

pub struct CodeOutput {
    pub code: String,
    pub debug_code: String,
}

#[derive()]
enum HeapVariableSize {
    Int,
    Float,
    Char,
    Bool,
}

impl HeapVariableSize {
    fn size(&self) -> i32 {
        match self {
            HeapVariableSize::Int => 4,
            HeapVariableSize::Float => 4,
            HeapVariableSize::Char => 1,
            HeapVariableSize::Bool => 1,
        }
    }
}

#[derive(Clone, Copy)]
enum VariableType {
    Int,
    Float,
    Char,
    Bool,
}

impl std::fmt::Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Int => write!(f, "i32"),
            VariableType::Float => write!(f, "f32"),
            VariableType::Char => write!(f, "char"),
            VariableType::Bool => write!(f, "bool"),
        }
    }
}

// TODO: Calculation can leave a value on the stack, which might not be desirable
// Need to implement a way to keep stack clean
fn evaluate_expression_to_stack(
    expression: &Expression,
    heap_allocation_map: &HeapAllocation,
) -> (String, String, VariableType, i32) {
    match expression {
        Expression::Integer(value) => {
            let CodeOutput {
                code: c,
                debug_code: dc,
            } = IMP::Stack(StackOperations::PushNumber(*value)).gen();
            (c, dc, VariableType::Int, 4)
        }
        Expression::Variable(name) => {
            let HeapVariable { type_, offset } = heap_allocation_map.get(name).unwrap();
            let CodeOutput {
                code: c,
                debug_code: dc,
            } = IMP::Stack(StackOperations::PushNumber(*offset)).gen();
            match type_ {
                VariableType::Int => {
                    let CodeOutput {
                        code: c2,
                        debug_code: dc2,
                    } = IMP::Heap(HeapOperations::Retrieve).gen();
                    (
                        c + &c2,
                        format!("{}; {};\n", dc, &dc2),
                        VariableType::Int,
                        4,
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
            let (left_code, left_debug_code, left_type, left_size) =
                evaluate_expression_to_stack(left, heap_allocation_map);
            let (right_code, right_debug_code, right_type, right_size) =
                evaluate_expression_to_stack(right, heap_allocation_map);
            let mut code = String::new();
            let mut debug_code = String::new();
            code.push_str(&left_code);
            code.push_str(&right_code);
            debug_code.push_str(&format!("{};\n", left_debug_code));
            debug_code.push_str(&format!("{};\n", right_debug_code));
            match operator {
                crate::ast::Operation::Add => {
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Arithmetic(ArithmeticOperations::Add).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                }
                crate::ast::Operation::Sub => {
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                }
                _ => {
                    panic!("Unsupported operator");
                } // crate::ast::Operation::Multiply => {
                  //     code.push_str(&IMP::Arithmetic(ArithmeticOperations::Multiply).gen().code);
                  //     debug_code.push_str(
                  //         &IMP::Arithmetic(ArithmeticOperations::Multiply)
                  //             .gen()
                  //             .debug_code,
                  //     );
                  // }
                  // crate::ast::Operation::Divide => {
                  //     code.push_str(&IMP::Arithmetic(ArithmeticOperations::Divide).gen().code);
                  //     debug_code.push_str(
                  //         &IMP::Arithmetic(ArithmeticOperations::Divide)
                  //             .gen()
                  //             .debug_code,
                  //     );
                  // }
                  // crate::ast::Operation::Modulo => {
                  //     code.push_str(&IMP::Arithmetic(ArithmeticOperations::Modulo).gen().code);
                  //     debug_code.push_str(
                  //         &IMP::Arithmetic(ArithmeticOperations::Modulo)
                  //             .gen()
                  //             .debug_code,
                  //     );
                  // }
                  // crate::ast::Operation::CompareEquals => {
                  //     code.push_str(&IMP::Arithmetic(ArithmeticOperations::Subtract).gen().code);
                  //     debug_code.push_str(
                  //         &IMP::Arithmetic(ArithmeticOperations::Subtract)
                  //             .gen()
                  //             .debug_code,
                  //     );
                  //     code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::JumpIfZero(0))
                  //             .gen()
                  //             .code,
                  //     );
                  //     debug_code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::JumpIfZero(0))
                  //             .gen()
                  //             .debug_code,
                  //     );
                  //     code.push_str(&IMP::Stack(StackOperations::PushNumber(0)).gen().code);
                  //     debug_code
                  //         .push_str(&IMP::Stack(StackOperations::PushNumber(0)).gen().debug_code);
                  //     code.push_str(&IMP::FlowControl(FlowControlOperations::Jump(0)).gen().code);
                  //     debug_code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::Jump(0))
                  //             .gen()
                  //             .debug_code,
                  //     );
                  //     code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::SetLabel(0))
                  //             .gen()
                  //             .code,
                  //     );
                  //     debug_code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::SetLabel(0))
                  //             .gen()
                  //             .debug_code,
                  //     );
                  //     code.push_str(&IMP::Stack(StackOperations::PushNumber(1)).gen().code);
                  //     debug_code
                  //         .push_str(&IMP::Stack(StackOperations::PushNumber(1)).gen().debug_code);
                  //     code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::SetLabel(0))
                  //             .gen()
                  //             .code,
                  //     );
                  //     debug_code.push_str(
                  //         &IMP::FlowControl(FlowControlOperations::SetLabel(0))
                  //             .gen()
                  //             .debug_code,
                  //     );
                  // }
            }
            (
                code,
                debug_code,
                VariableType::Int,
                std::cmp::max(left_size, right_size),
            )
        }
        _ => {
            panic!("Unsupported expression");
        }
    }
}

#[derive(Clone)]
struct HeapVariable {
    offset: i32,
    type_: VariableType,
}

struct HeapAllocation {
    map: HashMap<String, HeapVariable>,
    offset: i32,
}

impl Clone for HeapAllocation {
    fn clone(&self) -> Self {
        let mut map: HashMap<String, HeapVariable> = HashMap::new();
        for (key, value) in &self.map {
            map.insert(key.clone(), value.clone());
        }
        HeapAllocation {
            map,
            offset: self.offset,
        }
    }
}

impl HeapAllocation {
    pub fn new() -> Self {
        HeapAllocation {
            map: HashMap::new(),
            offset: 0,
        }
    }

    pub fn allocate(&mut self, name: String, type_: VariableType) -> i32 {
        let offset = self.offset;
        match type_ {
            VariableType::Int => {
                self.offset += HeapVariableSize::Int.size();
            }
            _ => {
                panic!("Only integer values are supported for now");
            }
        }
        self.map.insert(
            name,
            HeapVariable {
                offset: offset,
                type_,
            },
        );
        offset
    }

    pub fn get(&self, name: &String) -> Option<&HeapVariable> {
        self.map.get(name)
    }
}

#[derive(Clone)]
pub struct State {
    heap_allocation_map: HeapAllocation,
    current_label_n: i32,
}

impl State {
    pub fn new() -> Self {
        State {
            heap_allocation_map: HeapAllocation::new(),
            current_label_n: 0,
        }
    }

    pub fn get_label(&mut self) -> i32 {
        let label = self.current_label_n;
        self.current_label_n += 1;
        label
    }
}

pub fn transpile(ast: Vec<Statement>, state: Option<State>) -> CodeOutput {
    let mut code = String::new();
    let mut debug_code: String = String::new();

    let mut state: State = state.unwrap_or(State::new());

    for node in ast {
        match node {
            Statement::IntDeclaration(name) => {
                state.heap_allocation_map.allocate(name, VariableType::Int);
            }
            Statement::Assignment(name, value) => {
                match value {
                    Expression::Integer(value) => {
                        let CodeOutput {
                            code: c,
                            debug_code: dc,
                        } = IMP::Stack(StackOperations::PushNumber(value)).gen();
                        code.push_str(&c);
                        debug_code.push_str(&format!("{};\n", dc));
                    }
                    Expression::BinaryOp {
                        operator,
                        left,
                        right,
                    } => {
                        let left_result =
                            evaluate_expression_to_stack(&left, &state.heap_allocation_map);
                        let right_result =
                            evaluate_expression_to_stack(&right, &state.heap_allocation_map);
                        let (left_code, left_debug_code, left_type, left_size) = left_result;
                        let (right_code, right_debug_code, right_type, right_size) = right_result;
                        code.push_str(&left_code);
                        code.push_str(&right_code);
                        debug_code.push_str(&format!("{}\n", left_debug_code));
                        debug_code.push_str(&format!("{}\n", right_debug_code));
                        match operator {
                            Operation::Add => {
                                let CodeOutput {
                                    code: c,
                                    debug_code: dc,
                                } = IMP::Arithmetic(ArithmeticOperations::Add).gen();
                                code.push_str(&c);
                                debug_code.push_str(&format!("{};\n", dc));
                            }
                            Operation::Sub => {
                                let CodeOutput {
                                    code: c,
                                    debug_code: dc,
                                } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                                code.push_str(&c);
                                debug_code.push_str(&format!("{};\n", dc));
                            }
                            _ => {
                                panic!("Unsupported operator");
                            }
                        }
                    }
                    _ => {
                        panic!("Unsupported expression");
                    }
                }
                let addr = state.heap_allocation_map.get(&name).unwrap().offset;
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Stack(StackOperations::PushNumber(addr)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Stack(StackOperations::Swap).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::Heap(HeapOperations::Store).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
            }
            Statement::Print(expression) => match expression {
                Expression::Integer(value) => {
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Stack(StackOperations::PushNumber(value)).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));

                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::IO(IOOperations::PrintAsNumber).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                }
                Expression::Variable(name) => {
                    let HeapVariable { type_, offset } =
                        state.heap_allocation_map.get(&name).unwrap();
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Stack(StackOperations::PushNumber(*offset)).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Heap(HeapOperations::Retrieve).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                    match type_ {
                        VariableType::Int => {
                            let CodeOutput {
                                code: c,
                                debug_code: dc,
                            } = IMP::IO(IOOperations::PrintAsNumber).gen();
                            code.push_str(&c);
                            debug_code.push_str(&format!("{};\n", dc));
                        }
                        _ => {
                            panic!("Only integer values are supported for now");
                        }
                    }
                }
                _ => {
                    panic!("Only integer values are supported for now");
                }
            },
            Statement::Exit => {
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::Exit).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
            }
            // Statement::WhileLoop { condition, body } => {
            //     code.push_str(&format!("while ({}) {{\n", condition));
            //     code.push_str(&transpile(body));
            //     code.push_str("}\n");
            // }
            Statement::WhileLoop { condition, body } => {
                let loop_start_label = state.get_label();
                let loop_body_start_label = state.get_label();
                let loop_end_label = state.get_label();
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::SetLabel(loop_start_label)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{}\n", dc));
                match *condition {
                    Expression::BinaryOp {
                        operator,
                        left,
                        right,
                    } => match operator {
                        Operation::CompareEquals => {
                            let left_result =
                                evaluate_expression_to_stack(&left, &state.heap_allocation_map);
                            let right_result =
                                evaluate_expression_to_stack(&right, &state.heap_allocation_map);
                            let (left_code, left_debug_code, left_type, left_size) = left_result;
                            let (right_code, right_debug_code, right_type, right_size) =
                                right_result;
                            code.push_str(&left_code);
                            code.push_str(&right_code);
                            debug_code.push_str(&format!("{}\n", left_debug_code));
                            debug_code.push_str(&format!("{}\n", right_debug_code));
                            match left_type {
                                VariableType::Int => {
                                    let CodeOutput {
                                        code: c,
                                        debug_code: dc,
                                    } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                                    code.push_str(&c);
                                    debug_code.push_str(&format!("{};\n", dc));
                                }
                                _ => {
                                    panic!("Only integer values are supported for now");
                                }
                            }
                            let c2 = IMP::FlowControl(FlowControlOperations::JumpIfZero(
                                loop_body_start_label,
                            ))
                            .gen();
                            code.push_str(&c2.code);
                            debug_code.push_str(&format!("{};\n", c2.debug_code));
                        }
                        Operation::CompareLessThan => {
                            let left_result =
                                evaluate_expression_to_stack(&left, &state.heap_allocation_map);
                            let right_result =
                                evaluate_expression_to_stack(&right, &state.heap_allocation_map);
                            let (left_code, left_debug_code, left_type, left_size) = left_result;
                            let (right_code, right_debug_code, right_type, right_size) =
                                right_result;
                            code.push_str(&left_code);
                            code.push_str(&right_code);
                            debug_code.push_str(&format!("{}\n", left_debug_code));
                            debug_code.push_str(&format!("{}\n", right_debug_code));
                            match left_type {
                                VariableType::Int => {
                                    let CodeOutput {
                                        code: c,
                                        debug_code: dc,
                                    } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                                    code.push_str(&c);
                                    debug_code.push_str(&format!("{};\n", dc));
                                }
                                _ => {
                                    panic!("Only integer values are supported for now");
                                }
                            }
                            let c2 = IMP::FlowControl(FlowControlOperations::JumpIfNegative(
                                loop_body_start_label,
                            ))
                            .gen();
                            code.push_str(&c2.code);
                            debug_code.push_str(&format!("{};\n", c2.debug_code));
                        }
                        _ => {
                            panic!("Unsupported operator");
                        }
                    },
                    _ => {
                        panic!("Unsupported condition");
                    }
                }
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::Jump(loop_end_label)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::SetLabel(loop_body_start_label)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{}\n", dc));
                match *body {
                    Statement::Block(statements) => {
                        let CodeOutput {
                            code: c,
                            debug_code: dc,
                        } = transpile(statements, Some(state.clone()));
                        code.push_str(&c);
                        debug_code.push_str(&format!("{}\n", dc));
                    }
                    _ => {
                        panic!("Unsupported statement");
                    }
                }
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::Jump(loop_start_label)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{};\n", dc));
                let CodeOutput {
                    code: c,
                    debug_code: dc,
                } = IMP::FlowControl(FlowControlOperations::SetLabel(loop_end_label)).gen();
                code.push_str(&c);
                debug_code.push_str(&format!("{}\n", dc));
            }
            _ => {
                panic!("Unsupported statement");
            }
        }
    }
    CodeOutput { code, debug_code }
}

#[cfg(test)]
mod test {
    use crate::util;

    use super::*;

    #[test]
    fn print() {
        let input = vec![Statement::Print(Expression::Integer(99)), Statement::Exit];
        let CodeOutput { code, .. } = transpile(input, None);
        assert_eq!(util::unbleach(code), "sssttsssttntnstnnn");
    }
    #[test]
    fn heap_allocation() {
        let input = vec![
            Statement::IntDeclaration("m".to_string()),
            Statement::Assignment("m".to_string(), Expression::Integer(11)),
            Statement::Exit,
        ];
        let CodeOutput { code, .. } = transpile(input, None);
        assert_eq!(util::unbleach(code), "ssststtnssssnsntttsnnn");
    }
    #[test]
    fn print_from_heap() {
        let input = vec![
            Statement::IntDeclaration("m".to_string()),
            Statement::Assignment("m".to_string(), Expression::Integer(11)),
            Statement::Print(Expression::Variable("m".to_string())),
            Statement::Exit,
        ];
        let CodeOutput { code, .. } = transpile(input, None);
        assert_eq!(util::unbleach(code), "ssststtnssssnsntttsssssnttttnstnnn");
    }
    #[test]
    fn while_less_than() {
        let input = vec![
            Statement::IntDeclaration("m".to_string()),
            Statement::Assignment("m".to_string(), Expression::Integer(8)),
            Statement::WhileLoop {
                condition: Box::new(Expression::BinaryOp {
                    operator: Operation::CompareLessThan,
                    left: Box::new(Expression::Variable("m".to_string())),
                    right: Box::new(Expression::Integer(11)),
                }),
                body: Box::new(Statement::Block(vec![
                    Statement::Print(Expression::Variable("m".to_string())),
                    Statement::Assignment(
                        "m".to_string(),
                        Expression::BinaryOp {
                            operator: Operation::Add,
                            left: Box::new(Expression::Variable("m".to_string())),
                            right: Box::new(Expression::Integer(1)),
                        },
                    ),
                ])),
            },
            Statement::Exit,
        ];
        let CodeOutput { code, .. } = transpile(input, None);
        assert_eq!(
            util::unbleach(code),
            "ssstsssnssssnsntttsnssnssssntttssststtntsstntttnnsntsnnsstnssssnttttnstssssntttssstntsssssssnsntttsnsnnnsstsnnnn"
        );
    }
}
