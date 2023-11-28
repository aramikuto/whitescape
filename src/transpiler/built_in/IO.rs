use crate::{
    ast::Expression,
    specs::whitespace::{IOOperations, StackOperations, IMP},
    transpiler::{state::State, CodeOutput, VariableType},
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
            _ => {
                panic!("Only integer values are supported for now");
            }
        }
    } else {
        panic!("Unsupported argument");
    }
}
