use crate::Variable;

pub fn _local(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Local(false/*TODO */, index))
}

pub fn _capture(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Captured(index))
}