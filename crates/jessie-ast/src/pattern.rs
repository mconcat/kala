use crate::{Expr, VariableCell};

// BindingPattern, Param, Pattern are all collapsed into single Pattern type
// Be careful to not mess with parsing orders - struct types and parsing might not correspond
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern<'a> {
    Rest(&'a Pattern<'a>),
    Optional(&'a VariableCell<'a>, &'a Expr<'a>),
    ArrayPattern(&'a [Pattern<'a>]), // only Vec<Param> form is valid
    RecordPattern(&'a [PropParam<'a>]),
    Variable(&'a VariableCell<'a>),
}

impl<'a> Pattern<'a> {
    pub fn rest(pattern: &'a Self) -> Self {
        Pattern::Rest(pattern)
    }

    pub fn optional(name: &'a VariableCell<'a>, expr: &'a Expr) -> Self {
        Pattern::Optional(name, expr)
    }

    pub fn array(patterns: &'a [Self]) -> Self {
        Pattern::ArrayPattern(patterns)
    }

    pub fn record(props: &'a [PropParam]) -> Self {
        Pattern::RecordPattern(props)
    }

    pub fn variable(name: &'a VariableCell<'a>) -> Self {
        Pattern::Variable(name)
    }
}

impl<'a> From<Expr<'a>> for Pattern<'a> {
    fn from(value: Expr) -> Self {
        // Expression can be converted to pattern only if it is 
        // - a variable
        // - an assignment to a variable
        // - array compatible with destructuring
        // - object compatible with destructuring
        match value {
            Expr::Variable(name) => Pattern::Variable(name.into()),
            Expr::Assignment(assign) => unimplemented!("optional"),
            Expr::Array(arr) => unimplemented!("array"),
            Expr::Record(rec) => unimplemented!("record"), 
            _ => panic!("Cannot convert expr to pattern"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropParam<'a> {
    Rest(Pattern<'a>),
    KeyValue(&'a VariableCell<'a>, Pattern<'a>),
    Optional(&'a VariableCell<'a>, Expr<'a>),
    Shorthand(&'a VariableCell<'a>),
}

