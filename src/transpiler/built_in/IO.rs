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

        let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::PushNumber(addr)).gen();
        res.add(code, debug_code);

        match type_ {
            VariableType::Int => {
                let CodeOutput { code, debug_code } = IMP::IO(IOOperations::ReadAsNumber).gen();
                res.add(code, debug_code);
            }
            VariableType::String(_) => {
                let read_loop_start_label = state.get_label();
                let read_loop_end_label = state.get_label();
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(read_loop_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Duplicate).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Stack(StackOperations::Duplicate).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::IO(IOOperations::ReadAsChar).gen();
                res.add(code, debug_code);
                // Condition
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Retrieve).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(10)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Arithmetic(ArithmeticOperations::Subtract).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::JumpIfZero(read_loop_end_label)).gen();
                res.add(code, debug_code);
                // Advance pointer
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(HeapVariableSize::Char.size())).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Arithmetic(ArithmeticOperations::Add).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::Jump(read_loop_start_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::FlowControl(FlowControlOperations::SetLabel(read_loop_end_label)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } =
                    IMP::Stack(StackOperations::PushNumber(0)).gen();
                res.add(code, debug_code);
                let CodeOutput { code, debug_code } = IMP::Heap(HeapOperations::Store).gen();
                res.add(code, debug_code);
            }
            _ => {
                panic!("Only integer values are supported for now");
            }
        }
    } else {
        panic!("Unsupported argument");
    }
}
