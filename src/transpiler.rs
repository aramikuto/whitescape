use std::collections::HashMap;

use crate::ast::Expression;
use crate::ast::Statement;

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

struct HeapVariable {
    offset: i32,
    type_: VariableType,
}

pub fn transpile(ast: Vec<Statement>) -> CodeOutput {
    let mut code = String::new();
    let mut debug_code: String = String::new();
    // Map contains name of variable and its offset in the heap
    let mut heap_allocation_map: HashMap<String, HeapVariable> = HashMap::new();
    let mut heap_offset: i32 = 0; // Value is in bytes
    for node in ast {
        match node {
            Statement::IntDeclaration(name) => {
                heap_allocation_map.insert(
                    name,
                    HeapVariable {
                        offset: heap_offset,
                        type_: VariableType::Int,
                    },
                );
                heap_offset += HeapVariableSize::Int.size();
            }
            Statement::Assignment(name, value) => match value {
                Expression::Integer(value) => {
                    let addr = heap_allocation_map.get(&name).unwrap().offset;
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Stack(StackOperations::PushNumber(addr)).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Stack(StackOperations::PushNumber(value)).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                    let CodeOutput {
                        code: c,
                        debug_code: dc,
                    } = IMP::Heap(HeapOperations::Store).gen();
                    code.push_str(&c);
                    debug_code.push_str(&format!("{};\n", dc));
                }
                _ => {
                    panic!("Only integer values are supported for now");
                }
            },
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
                    let HeapVariable { type_, offset } = heap_allocation_map.get(&name).unwrap();
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
        let CodeOutput { code, .. } = transpile(input);
        assert_eq!(util::unbleach(code), "sssttsssttntnstnnn");
    }
    #[test]
    fn heap_allocation() {
        let input = vec![
            Statement::IntDeclaration("m".to_string()),
            Statement::Assignment("m".to_string(), Expression::Integer(11)),
            Statement::Exit,
        ];
        let CodeOutput { code, .. } = transpile(input);
        assert_eq!(util::unbleach(code), "ssssnssststtnttsnnn");
    }
    #[test]
    fn print_from_heap() {
        let input = vec![
            Statement::IntDeclaration("m".to_string()),
            Statement::Assignment("m".to_string(), Expression::Integer(11)),
            Statement::Print(Expression::Variable("m".to_string())),
            Statement::Exit,
        ];
        let CodeOutput { code, .. } = transpile(input);
        assert_eq!(util::unbleach(code), "ssssnssststtnttsssssnttttnstnnn");
    }
}
