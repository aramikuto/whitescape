use crate::{
    ast::Expression,
    specs::whitespace::{
        ArithmeticOperations, FlowControlOperations, HeapOperations, StackOperations, IMP,
    },
    transpiler::{emitter::CodeEmitter, state::State, CodeOutput, HeapVariableSize, VariableType},
};

pub fn concat(state: &mut State, args: &Vec<Expression>, res: &mut CodeOutput) {
    res.add("".to_string(), "# concat".to_string());
    let target = args.get(0).unwrap();
    let source = args.get(1).unwrap();
    if let (Expression::Variable(target_name), Expression::Variable(source_name)) = (target, source)
    {
        let target_variable = state.heap_allocation_map.get(&target_name).unwrap();
        let source_variable = state.heap_allocation_map.get(&source_name).unwrap();
        let target_type = target_variable.type_();
        let source_type = source_variable.type_();
        let target_addr = target_variable.offset();
        let source_addr = source_variable.offset();

        match (target_type, source_type) {
            (VariableType::String(_), VariableType::String(_)) => {
                let seek_start_label = state.get_label();
                let seek_end_label = state.get_label();
                let copy_start_label = state.get_label();
                let copy_end_label = state.get_label();

                let mut emitter = CodeEmitter {};

                res.append(emitter.emit(vec![
                    // Set pointer to the end of the target string
                    IMP::Stack(StackOperations::PushNumber(target_addr)),
                    IMP::FlowControl(FlowControlOperations::SetLabel(seek_start_label)),
                    IMP::Stack(StackOperations::Duplicate),
                    IMP::Heap(HeapOperations::Retrieve),
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(seek_end_label)),
                    // Advance pointer
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())),
                    IMP::Arithmetic(ArithmeticOperations::Add),
                    IMP::FlowControl(FlowControlOperations::Jump(seek_start_label)),
                    IMP::FlowControl(FlowControlOperations::SetLabel(seek_end_label)),
                    // Copy source string
                    IMP::Stack(StackOperations::PushNumber(source_addr)),
                    IMP::FlowControl(FlowControlOperations::SetLabel(copy_start_label)),
                    IMP::Stack(StackOperations::Duplicate),
                    IMP::Heap(HeapOperations::Retrieve),
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(copy_end_label)),
                    // Copy char
                    IMP::Stack(StackOperations::Duplicate),
                    IMP::Heap(HeapOperations::Retrieve),
                    IMP::Stack(StackOperations::CopyNth(2)),
                    IMP::Stack(StackOperations::Swap),
                    IMP::Heap(HeapOperations::Store),
                    // Advance target pointer
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())),
                    IMP::Arithmetic(ArithmeticOperations::Add),
                    IMP::Stack(StackOperations::Swap),
                    // Advance source pointer
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())),
                    IMP::Arithmetic(ArithmeticOperations::Add),
                    IMP::Stack(StackOperations::Swap),
                    IMP::FlowControl(FlowControlOperations::Jump(copy_start_label)),
                    IMP::FlowControl(FlowControlOperations::SetLabel(copy_end_label)),
                    // Add null terminator
                    IMP::Stack(StackOperations::Swap),
                    IMP::Stack(StackOperations::PushNumber(0)),
                    IMP::Heap(HeapOperations::Store),
                    // Clean up
                    IMP::Stack(StackOperations::Discard),
                ]));
            }
            _ => {
                panic!("Only string values are supported for now");
            }
        }
    }
}
