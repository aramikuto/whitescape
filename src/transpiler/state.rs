use std::collections::HashMap;

use super::{HeapVariableSize, VariableType};

#[derive(Clone)]
pub struct HeapVariable {
    offset: i32,
    type_: VariableType,
}

impl HeapVariable {
    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn type_(&self) -> VariableType {
        self.type_
    }
}

pub struct HeapAllocation {
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
    pub heap_allocation_map: HeapAllocation,
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
