use crate::transpiler::emitter::CodeEmitter;

use crate::{
    ast::Expression,
    specs::whitespace::{
        ArithmeticOperations, FlowControlOperations, HeapOperations, IOOperations, StackOperations,
        IMP,
    },
    transpiler::{state::State, CodeOutput, HeapVariableSize, VariableType},
};

pub fn read(state: &mut State, args: &Vec<Expression>, res: &mut CodeOutput) {
    let target = args.get(0).unwrap();
    if let Expression::Variable(name) = target {
        let variable = state.heap_allocation_map.get(&name).unwrap();
        let type_ = variable.type_();
        let addr = variable.offset();
        let mut emitter = CodeEmitter {};

        res.append(emitter.emit(vec![IMP::Stack(StackOperations::PushNumber(addr))]));

        match type_ {
            VariableType::Int => {
                res.append(emitter.emit(vec![IMP::IO(IOOperations::ReadAsNumber)]))
            }
            VariableType::String(_) => {
                let read_loop_start_label = state.get_label();
                let read_loop_end_label = state.get_label();
                res.append(emitter.emit(vec![
                    IMP::FlowControl(FlowControlOperations::SetLabel(read_loop_start_label)),
                    IMP::Stack(StackOperations::Duplicate),
                    IMP::Stack(StackOperations::Duplicate),
                    IMP::IO(IOOperations::ReadAsChar),
                    // Condition
                    IMP::Heap(HeapOperations::Retrieve),
                    IMP::Stack(StackOperations::PushNumber(10)),
                    IMP::Arithmetic(ArithmeticOperations::Subtract),
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(read_loop_end_label)),
                    // Advance pointer
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())),
                    IMP::Arithmetic(ArithmeticOperations::Add),
                    IMP::FlowControl(FlowControlOperations::Jump(read_loop_start_label)),
                    IMP::FlowControl(FlowControlOperations::SetLabel(read_loop_end_label)),
                    IMP::Stack(StackOperations::PushNumber(0)),
                    IMP::Heap(HeapOperations::Store),
                ]));
            }
            _ => {
                panic!("Only integer and strings values are supported for now");
            }
        }
    } else {
        panic!("Unsupported argument");
    }
}

pub fn print(state: &mut State, args: &Vec<Expression>, res: &mut CodeOutput) {
    let expression = args.get(0).unwrap();
    let mut emitter = CodeEmitter {};

    match expression {
        Expression::Literal(value) => {
            for ch in value.chars() {
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(ch as i32)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::IO(IOOperations::PrintAsChar).gen();
                res.add(code, debug_code);
            }
        }
        Expression::Integer(value) => {
            let CodeOutput { code, debug_code } =
                IMP::Stack(StackOperations::PushNumber(*value)).gen();
            res.add(code, debug_code);

            let CodeOutput { code, debug_code } = IMP::IO(IOOperations::PrintAsNumber).gen();
            res.add(code, debug_code);
        }
        Expression::Variable(name) => {
            let variable = state.heap_allocation_map.get(&name).unwrap();
            let type_ = variable.type_();
            let offset = variable.offset();

            let CodeOutput { code, debug_code } =
                IMP::Stack(StackOperations::PushNumber(offset)).gen();
            res.add(code, debug_code);
            match type_ {
                VariableType::Int => {
                    res.append(emitter.emit(vec![IMP::Heap(HeapOperations::Retrieve)]));
                    res.append(emitter.emit(vec![IMP::IO(IOOperations::PrintAsNumber)]));
                }
                VariableType::String(_) => {
                    let print_loop_start_label = state.get_label();
                    let print_loop_end_label = state.get_label();
                    res.append(emitter.emit(vec![
                        IMP::FlowControl(FlowControlOperations::SetLabel(print_loop_start_label)),
                        IMP::Stack(StackOperations::Duplicate),
                        IMP::Stack(StackOperations::Duplicate),
                        IMP::Heap(HeapOperations::Retrieve),
                        // Condition
                        IMP::FlowControl(FlowControlOperations::JumpIfZero(print_loop_end_label)),
                        // Print
                        IMP::Heap(HeapOperations::Retrieve),
                        IMP::IO(IOOperations::PrintAsChar),
                        // Advance pointer
                        IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())),
                        IMP::Arithmetic(ArithmeticOperations::Add),
                        IMP::FlowControl(FlowControlOperations::Jump(print_loop_start_label)),
                        IMP::FlowControl(FlowControlOperations::SetLabel(print_loop_end_label)),
                        IMP::Stack(StackOperations::Discard),
                    ]));
                }
                _ => {
                    panic!("Only integer and string values are supported for now");
                }
            }
        }
        Expression::BinaryOp { .. } => {
            let mut emitter = CodeEmitter {};
            let (code, debug_code, ..) =
                emitter.evaluate_expression(expression, &state.heap_allocation_map, 0);
            res.add(code, debug_code.render());
            let CodeOutput { code, debug_code } = IMP::IO(IOOperations::PrintAsNumber).gen();
            res.add(code, debug_code);
        }
        _ => {
            panic!("Unsupported expression");
        }
    }
}
