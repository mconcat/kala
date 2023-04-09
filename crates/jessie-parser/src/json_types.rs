use crate::{parser::{Node, ParserState}, json_parser::data_structure};

#[derive(Debug, PartialEq, Clone)]
pub enum DataLiteral {
    Null,
    False,
    True,
    Number(String),
    String(String),
}

impl ToString for DataLiteral {
    fn to_string(&self) -> String {
        match self {
            DataLiteral::Null => "null".to_string(),
            DataLiteral::False => "false".to_string(),
            DataLiteral::True => "true".to_string(),
            DataLiteral::Number(n) => n.to_string(),
            DataLiteral::String(s) => s.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataStructure {
    DataLiteral(DataLiteral),
    Array(Array),
    Record(Record),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array(pub Vec<Element>);

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Expr(DataStructure),
}


#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Vec<PropDef>);

#[derive(Debug, PartialEq, Clone)]
pub enum PropDef {
    KeyValue(PropName, DataStructure), 
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropName {
    String(String),
    Number(String),
    Ident(String),
}