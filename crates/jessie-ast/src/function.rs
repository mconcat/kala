use crate::{Statement, Expr};
use std::cell::OnceCell;

#[derive(Debug, PartialEq, Clone)]
pub struct Function<'a> {
    pub name: Option<&'a str>,

    pub captures: &'a [Variable],

    pub declarations: &'a [VariableDeclaration],

    // block body
    pub statements: &'a [Statement<'a>],

    // arrow function expression body
    pub expression: Option<Expr<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableDeclarationType {
    Parameter,
    Local,
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: &'static str,
    pub block: u16,
    pub block_declaration_index: u16,
    pub variable_type: VariableDeclarationType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub declaration_index: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableCell<'a>(OnceCell<&'a Variable>);