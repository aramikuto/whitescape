use crate::{
    ast::Expression,
    specs::whitespace::{
        ArithmeticOperations, FlowControlOperations, HeapOperations, StackOperations, IMP,
    },
    transpiler::{state::State, CodeOutput, HeapVariableSize, VariableType},
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
            (VariableType::String(target_length), VariableType::String(source_length)) => {
                let seek_start_label = state.get_label();
                let seek_end_label = state.get_label();
                // Set pointer to the end of the target string
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(target_addr)).gen();
                res.add(code, debug_code);

                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(seek_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Duplicate).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Retrieve).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(seek_end_label)).gen();
                res.add(code, debug_code);
                // Advance pointer
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Arithmetic(ArithmeticOperations::Add).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Jump(seek_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(seek_end_label)).gen();
                res.add(code, debug_code);

                // Copy source string
                let copy_start_label = state.get_label();
                let copy_end_label = state.get_label();
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(source_addr)).gen();
                res.add(code, debug_code);

                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(copy_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Duplicate).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Retrieve).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(copy_end_label)).gen();
                res.add(code, debug_code);

                // Copy char
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Duplicate).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Retrieve).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::CopyNth(2)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Swap).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Store).gen();
                res.add(code, debug_code);

                // Advance target pointer
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Arithmetic(ArithmeticOperations::Add).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Swap).gen();
                res.add(code, debug_code);
                // Advance source pointer
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Arithmetic(ArithmeticOperations::Add).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Swap).gen();
                res.add(code, debug_code);

                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Jump(copy_start_label)).gen();
                res.add(code, debug_code);

                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(copy_end_label)).gen();
                res.add(code, debug_code);
                // Add null terminator
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Swap).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(0)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Store).gen();
                res.add(code, debug_code);

                // Clean up
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Discard).gen();
                res.add(code, debug_code);
            }
            _ => {
                panic!("Only string values are supported for now");
            }
        }
    }
}
