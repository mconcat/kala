use std::{cell::{Cell, OnceCell}, rc::Rc};

use utils::SharedString;

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub name: SharedString,
    pub index: Rc<Cell<VariableIndex>>,
    pub block_index: u32,
}

impl Variable {
    pub fn new(name: SharedString, index: VariableIndex, block_index: u32) -> Self {
        Self {
            name,
            index: Rc::new(Cell::new(index)),
            block_index
        }
    }

    pub fn unknown(name: SharedString, block_index: u32) -> Self {
        Self {
            name,
            index: Rc::new(Cell::new(VariableIndex::Unknown)),
            block_index
        }
    }

    pub fn get_name(&self) -> &SharedString {
        &self.name
    }
    pub fn set_parameter_index(&self, index: u32) {
        self.index.set(VariableIndex::Parameter(index))
    }
    pub fn set_capture_index(&self, index: u32) {
        self.index.set(VariableIndex::Capture(index))
    }

    pub fn set_local_index(&self, index: u32) {
        self.index.set(VariableIndex::Local(index))
    }

    pub fn set_static_index(&self, index: u32) {
        self.index.set(VariableIndex::Static(index))
    }

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VariableIndex {
    Unknown,
    Parameter(u32), // points to the function scope's parameter_variables
    Capture(u32), // points to the function scope's captured_variables, which is a Vec<Variable> refering to the parent function's variables
    Local(u32), // points to the function scope's declared_variables
    Static(u32), // points to the module scope's static_variables
}

impl VariableIndex {
    pub fn unwrap_local(&self) -> u32 {
        match self {
            VariableIndex::Local(index) => *index,
            _ => panic!("unwrap_local called on {:?}", self)
        }
    }
}