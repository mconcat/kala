use crate::{Expr};

// BindingPattern, Param, Pattern are all collapsed into single Pattern type
// Be careful to not mess with parsing orders - struct types and parsing might not correspond
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>),
    Optional(String, Expr),
    ArrayPattern(Vec<Pattern>), // only Vec<Param> form is valid
    RecordPattern(Vec<PropParam>),
    Variable(String),
}
/* 
impl Pattern {
    pub fn rest(pattern: &'a Self) -> Self {
        Pattern::Rest(pattern)
    }

    pub fn optional(name: &'a VariableCell, expr: &'a Expr) -> Self {
        Pattern::Optional(name, expr)
    }

    pub fn array(patterns: &'a [Self]) -> Self {
        Pattern::ArrayPattern(patterns)
    }

    pub fn record(props: &'a [PropParam]) -> Self {
        Pattern::RecordPattern(props)
    }

    pub fn variable(name: &'a VariableCell) -> Self {
        Pattern::Variable(name)
    }
}

impl From<Expr> for Pattern {
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
*/
#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    Rest(Pattern),
    KeyValue(String, Pattern),
    Optional(String, Expr),
    Shorthand(String),
}

