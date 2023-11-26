use crate::transpiler::CodeOutput;
mod utils;

pub enum StackOperations {
    PushNumber(i32),
    //...
    Swap,
    //...
    Slide(i32),
}

impl StackOperations {
    const PREFIX: &'static str = " ";
    pub fn gen(&self) -> CodeOutput {
        match self {
            StackOperations::PushNumber(value) => CodeOutput {
                debug_code: format!("push {}", value),
                code: format!("{} {}", Self::PREFIX, utils::encode_number(*value)),
            },
            StackOperations::Swap => CodeOutput {
                debug_code: format!("swap"),
                code: format!("{}\n\t", Self::PREFIX),
            },
            StackOperations::Slide(n) => CodeOutput {
                debug_code: format!("slide {}", n),
                code: format!("{}\t\n{}", Self::PREFIX, utils::encode_number(*n)),
            },
        }
    }
}

pub enum ArithmeticOperations {
    Add,
    Subtract,
    Multiply,
    DivideInteger,
    Modulo,
}

impl ArithmeticOperations {
    const PREFIX: &'static str = "\t ";
    pub fn gen(&self) -> CodeOutput {
        match self {
            ArithmeticOperations::Add => CodeOutput {
                debug_code: format!("add"),
                code: format!("{}  ", Self::PREFIX),
            },
            ArithmeticOperations::Subtract => CodeOutput {
                debug_code: format!("subtract"),
                code: format!("{} \t", Self::PREFIX),
            },
            ArithmeticOperations::Multiply => CodeOutput {
                debug_code: format!("multiply"),
                code: format!("{} \n", Self::PREFIX),
            },
            ArithmeticOperations::DivideInteger => CodeOutput {
                debug_code: format!("divide"),
                code: format!("{}\t ", Self::PREFIX),
            },
            ArithmeticOperations::Modulo => CodeOutput {
                debug_code: format!("modulo"),
                code: format!("{}\t\t", Self::PREFIX),
            },
        }
    }
}

pub enum HeapOperations {
    Store,
    Retrieve,
}

impl HeapOperations {
    const PREFIX: &'static str = "\t\t";
    pub fn gen(&self) -> CodeOutput {
        match self {
            HeapOperations::Store => CodeOutput {
                debug_code: format!("store"),
                code: format!("{} ", Self::PREFIX),
            },
            HeapOperations::Retrieve => CodeOutput {
                debug_code: format!("retrieve"),
                code: format!("{}\t", Self::PREFIX),
            },
        }
    }
}

pub enum IOOperations {
    PrintAsChar,
    PrintAsNumber,
}

impl IOOperations {
    const PREFIX: &'static str = "\t\n";
    pub fn gen(&self) -> CodeOutput {
        match self {
            IOOperations::PrintAsChar => CodeOutput {
                debug_code: format!("print_stack_top_as_char"),
                code: format!("{}  ", Self::PREFIX),
            },
            IOOperations::PrintAsNumber => CodeOutput {
                debug_code: format!("print_stack_top_as_number"),
                code: format!("{} \t", Self::PREFIX),
            },
        }
    }
}

pub enum FlowControlOperations {
    SetLabel(i32),
    // ...
    Jump(i32),
    JumpIfZero(i32),
    JumpIfNegative(i32),
    Exit,
}

impl FlowControlOperations {
    const PREFIX: &'static str = "\n";
    pub fn gen(&self) -> CodeOutput {
        match self {
            FlowControlOperations::SetLabel(label) => CodeOutput {
                debug_code: format!("{}:", label),
                code: format!("{}  {}", Self::PREFIX, utils::number_to_label(label)),
            },
            FlowControlOperations::Jump(label) => CodeOutput {
                debug_code: format!("jump {}", label),
                code: format!("{} \n{}", Self::PREFIX, utils::number_to_label(label)),
            },
            FlowControlOperations::JumpIfZero(label) => CodeOutput {
                debug_code: format!("jump_if_zero {}", label),
                code: format!("{}\t {}", Self::PREFIX, utils::number_to_label(label)),
            },
            FlowControlOperations::JumpIfNegative(label) => CodeOutput {
                debug_code: format!("jump_if_negative {}", label),
                code: format!("{}\t\t{}", Self::PREFIX, utils::number_to_label(label)),
            },
            FlowControlOperations::Exit => CodeOutput {
                debug_code: format!("exit"),
                code: format!("{}\n\n", Self::PREFIX),
            },
        }
    }
}

pub enum IMP {
    Stack(StackOperations),
    Arithmetic(ArithmeticOperations),
    Heap(HeapOperations),
    FlowControl(FlowControlOperations),
    IO(IOOperations),
}

impl IMP {
    pub fn gen(&self) -> CodeOutput {
        match self {
            IMP::Stack(operation) => operation.gen(),
            IMP::Arithmetic(operation) => operation.gen(),
            IMP::Heap(operation) => operation.gen(),
            IMP::IO(operation) => operation.gen(),
            IMP::FlowControl(operation) => operation.gen(),
        }
    }
}
