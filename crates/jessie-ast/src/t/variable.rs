use crate::{LocalVariable, Variable};

pub fn _let_var(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Local(false, index))
}

pub fn _let_var_local(name: &str, index: u32) -> LocalVariable {
    LocalVariable::new(Variable::declared(name.into(), crate::VariableIndex::Local(false, index)))
}

pub fn _let_var_escaping(name: &str, index: u32) -> LocalVariable {
    LocalVariable::escaping(Variable::declared(name.into(), crate::VariableIndex::Local(false, index)))
}

pub fn _const_var(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Local(true, index))
}

pub fn _const_var_local(name: &str, index: u32) -> LocalVariable {
    LocalVariable::new(Variable::declared(name.into(), crate::VariableIndex::Local(true, index)))
}

pub fn _const_var_escaping(name: &str, index: u32) -> LocalVariable {
    LocalVariable::escaping(Variable::declared(name.into(), crate::VariableIndex::Local(true, index)))
}

pub fn _capture(name: &str, index: u32) -> Variable {
    Variable::declared(name.into(), crate::VariableIndex::Captured(index))
}