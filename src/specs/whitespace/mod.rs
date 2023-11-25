use crate::transpiler::CodeOutput;
mod utils;

pub enum StackOperations {
    PushNumber(i32),
}

impl StackOperations {
    const PREFIX: &'static str = " ";
    pub fn gen(&self) -> CodeOutput {
        match self {
            StackOperations::PushNumber(value) => CodeOutput {
                debug_code: format!("push {}", value),
                code: format!("{} {}", Self::PREFIX, utils::encode_number(*value)),
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
    Exit,
}

impl FlowControlOperations {
    const PREFIX: &'static str = "\n";
    pub fn gen(&self) -> CodeOutput {
        match self {
            FlowControlOperations::Exit => CodeOutput {
                debug_code: format!("exit"),
                code: format!("{}\n\n", Self::PREFIX),
            },
        }
    }
}

pub enum IMP {
    Stack(StackOperations),
    // Arithmetic(ArithmeticOperations),
    Heap(HeapOperations),
    FlowControl(FlowControlOperations),
    IO(IOOperations),
}

impl IMP {
    pub fn gen(&self) -> CodeOutput {
        match self {
            IMP::Stack(operation) => operation.gen(),
            IMP::Heap(operation) => operation.gen(),
            IMP::IO(operation) => operation.gen(),
            IMP::FlowControl(operation) => operation.gen(),
        }
    }
}
