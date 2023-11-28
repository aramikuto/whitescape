mod state;
mod tests;

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

impl CodeOutput {
    pub fn new() -> Self {
        CodeOutput {
            code: String::new(),
            debug_code: String::new(),
        }
    }

    pub fn add(&mut self, code: String, debug_code: String) {
        self.code.push_str(&code);
        self.debug_code.push_str(&format!("{}\n", debug_code));
    }
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
pub enum VariableType {
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

pub enum DebugCodeEntry {
    Code(String),
    Block(DebugCodeBlock),
}

pub struct DebugCodeBlock {
    name: Option<String>,
    code: Vec<DebugCodeEntry>,
    level: i8,
}

impl DebugCodeBlock {
    pub fn new(name: Option<String>, level: i8) -> Self {
        DebugCodeBlock {
            name,
            code: Vec::new(),
            level,
        }
    }

    pub fn push(&mut self, code: String) {
        self.code.push(DebugCodeEntry::Code(code));
    }

    pub fn append(&mut self, block: DebugCodeBlock) {
        self.code.push(DebugCodeEntry::Block(block));
    }

    pub fn render(&self) -> String {
        let mut result = String::new();
        if let Some(name) = &self.name {
            result.push_str(&format!(
                "{:indent$}{} {{\n",
                "",
                name,
                indent = (self.level * 2) as usize,
            ));
        }
        for entry in &self.code {
            match entry {
                DebugCodeEntry::Code(code) => {
                    result.push_str(&format!(
                        "{:indent$}{};\n",
                        "",
                        code,
                        indent = ((self.level + 1) * 2) as usize,
                    ));
                }
                DebugCodeEntry::Block(block) => {
                    result.push_str(&block.render());
                }
            }
        }
        if let Some(_) = &self.name {
            result.push_str(&format!(
                "{:indent$}}}\n",
                "",
                indent = (self.level * 2) as usize,
            ));
        }
        result
    }
}

// TODO: Calculation can leave a value on the stack, which might not be desirable
// Need to implement a way to keep stack clean
fn evaluate_expression_to_stack(
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
            } = evaluate_binary_op(operator, left, right, heap_allocation_map, level);
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

struct EvaluationResult {
    code: String,
    debug_code: DebugCodeBlock,
    type_: VariableType,
}

fn evaluate_binary_op(
    operator: &Operation,
    left: &Expression,
    right: &Expression,
    heap_allocation_map: &state::HeapAllocation,
    level: i8,
) -> EvaluationResult {
    let mut code = String::new();
    let mut debug_code = DebugCodeBlock::new(None, level);

    let (left_code, left_debug_code, left_type, left_size) =
        evaluate_expression_to_stack(left, heap_allocation_map, level + 1);
    let (right_code, right_debug_code, right_type, right_size) =
        evaluate_expression_to_stack(right, heap_allocation_map, level + 1);
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

pub fn transpile(ast: Vec<Statement>, state: Option<state::State>) -> CodeOutput {
    let mut res: CodeOutput = CodeOutput::new();
    let mut state: state::State = state.unwrap_or(state::State::new());

    for node in ast {
        match node {
            Statement::IntDeclaration(name) => {
                state.heap_allocation_map.allocate(name, VariableType::Int);
            }
            Statement::Assignment(name, value) => {
                match value {
                    Expression::Integer(value) => {
                        let CodeOutput { code, debug_code } =
                            IMP::Stack(StackOperations::PushNumber(value)).gen();
                        res.add(code, debug_code);
                    }
                    Expression::BinaryOp {
                        operator,
                        left,
                        right,
                    } => {
                        let (code, debug_code, ..) = evaluate_expression_to_stack(
                            &Expression::BinaryOp {
                                operator,
                                left,
                                right,
                            },
                            &state.heap_allocation_map,
                            0,
                        );
                        res.add(code, debug_code.render());
                    }
                    _ => {
                        panic!("Unsupported expression");
                    }
                }
                let addr = state.heap_allocation_map.get(&name).unwrap().offset();
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(addr)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Swap).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Store).gen();
                res.add(code, debug_code);
            }
            Statement::Print(expression) => match expression {
                Expression::Literal(value) => {
                    for ch in value.chars() {
                        let CodeOutput { code, debug_code } =
                            IMP::Stack(StackOperations::PushNumber(ch as i32)).gen();
                        res.add(code, debug_code);
                        let CodeOutput { code, debug_code } =
                            IMP::IO(IOOperations::PrintAsChar).gen();
                        res.add(code, debug_code);
                    }
                }
                Expression::Integer(value) => {
                    let CodeOutput { code, debug_code } =
                        IMP::Stack(StackOperations::PushNumber(value)).gen();
                    res.add(code, debug_code);

                    let CodeOutput { code, debug_code } =
                        IMP::IO(IOOperations::PrintAsNumber).gen();
                    res.add(code, debug_code);
                }
                Expression::Variable(name) => {
                    let variable = state.heap_allocation_map.get(&name).unwrap();
                    let type_ = variable.type_();
                    let offset = variable.offset();
                    let CodeOutput { code, debug_code } =
                        IMP::Stack(StackOperations::PushNumber(offset)).gen();
                    res.add(code, debug_code);
                    let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Retrieve).gen();
                    res.add(code, debug_code);
                    match type_ {
                        VariableType::Int => {
                            let CodeOutput { code, debug_code } =
                                IMP::IO(IOOperations::PrintAsNumber).gen();
                            res.add(code, debug_code);
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
                    let (code, debug_code, ..) = evaluate_expression_to_stack(
                        &Expression::BinaryOp {
                            operator,
                            left,
                            right,
                        },
                        &state.heap_allocation_map,
                        0,
                    );
                    res.add(code, debug_code.render());
                    let CodeOutput { code, debug_code } =
                        IMP::IO(IOOperations::PrintAsNumber).gen();
                    res.add(code, debug_code);
                }
                _ => {
                    panic!("Unsupported expression");
                }
            },
            Statement::Exit => {
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Exit).gen();
                res.add(code, debug_code);
            }
            Statement::WhileLoop { condition, body } => {
                let loop_start_label = state.get_label();
                let loop_body_start_label = state.get_label();
                let loop_end_label = state.get_label();
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(loop_start_label)).gen();
                res.add(code, debug_code);
                match *condition {
                    Expression::BinaryOp {
                        operator,
                        left,
                        right,
                    } => match operator {
                        Operation::CompareEquals => {
                            let left_result =
                                evaluate_expression_to_stack(&left, &state.heap_allocation_map, 0);
                            let right_result =
                                evaluate_expression_to_stack(&right, &state.heap_allocation_map, 0);
                            let (left_code, left_debug_code, left_type, left_size) = left_result;
                            let (right_code, right_debug_code, right_type, right_size) =
                                right_result;
                            res.add(left_code, left_debug_code.render());
                            res.add(right_code, right_debug_code.render());
                            match left_type {
                                VariableType::Int => {
                                    let CodeOutput {
                                        code: c,
                                        debug_code: dc,
                                    } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                                    res.add(c, dc);
                                }
                                _ => {
                                    panic!("Only integer values are supported for now");
                                }
                            }
                            let c2 = IMP::FlowControl(FlowControlOperations::JumpIfZero(
                                loop_body_start_label,
                            ))
                            .gen();
                            res.add(c2.code, c2.debug_code);
                        }
                        Operation::CompareLessThan => {
                            let left_result =
                                evaluate_expression_to_stack(&left, &state.heap_allocation_map, 0);
                            let right_result =
                                evaluate_expression_to_stack(&right, &state.heap_allocation_map, 0);
                            let (left_code, left_debug_code, left_type, left_size) = left_result;
                            let (right_code, right_debug_code, right_type, right_size) =
                                right_result;
                            res.add(left_code, left_debug_code.render());
                            res.add(right_code, right_debug_code.render());
                            match left_type {
                                VariableType::Int => {
                                    let CodeOutput {
                                        code: c,
                                        debug_code: dc,
                                    } = IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                                    res.add(c, dc);
                                }
                                _ => {
                                    panic!("Only integer values are supported for now");
                                }
                            }
                            let c2 = IMP::FlowControl(FlowControlOperations::JumpIfNegative(
                                loop_body_start_label,
                            ))
                            .gen();
                            res.add(c2.code, c2.debug_code);
                        }
                        _ => {
                            panic!("Unsupported operator");
                        }
                    },
                    _ => {
                        panic!("Unsupported condition");
                    }
                }
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Jump(loop_end_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(loop_body_start_label)).gen();
                res.add(code, debug_code);
                match *body {
                    Statement::Block(statements) => {
                        let CodeOutput { code, debug_code } =
                            transpile(statements, Some(state.clone()));
                        res.add(code, debug_code);
                    }
                    _ => {
                        panic!("Unsupported statement");
                    }
                }
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Jump(loop_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(loop_end_label)).gen();
                res.add(code, debug_code);
            }
            _ => {
                panic!("Unsupported statement");
            }
        }
    }
    CodeOutput {
        code: res.code,
        debug_code: res.debug_code,
    }
}
