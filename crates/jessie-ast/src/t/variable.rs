use crate::Variable;

pub fn _let_var(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Local(false, index))
}

pub fn _const_var(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Local(true, index))
}

pub fn _capture(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Captured(index))
}