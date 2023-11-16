use utils::{FxMap, Map, SharedString};

use crate::{Expr, ExprDiscriminant, Record, Field, AssignOp, LValue, PropertyAccess, DeclarationIndex, VariablePointer};

// Pattern is a subset of Expr
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Rest(Box<Pattern>) = ExprDiscriminant::Spread as u8,
    Optional(Box<OptionalPattern>) = ExprDiscriminant::Assignment as u8,
    ArrayPattern(Box<ArrayPattern>) = ExprDiscriminant::Array as u8, // only Vec<Param> form is valid
    RecordPattern(Box<RecordPattern>) = ExprDiscriminant::Record as u8,
    Variable(Box<VariablePointer>) = ExprDiscriminant::Variable as u8,
}

impl Pattern {
    pub fn visit(&self, index: DeclarationIndex, f: &mut impl PatternVisitor) -> Option<()> {
        let mut access = vec![];
        self.visit_internal(index, &mut access, f)
    }

    pub(crate) fn visit_internal(&self, index: DeclarationIndex, property_access: &mut Vec<PropertyAccess>, f: &mut impl PatternVisitor) -> Option<()> {
        match self {
            Self::Rest(pat) => unimplemented!("rest pattern"),
            Self::Optional(pat) => unimplemented!("optional"),
            Self::ArrayPattern(pat) => {
                for (i, elem) in (&pat.0).iter().enumerate() {
                    property_access.push(PropertyAccess::Element(i as u32));
                    elem.visit_internal(index, property_access, f)?;
                    property_access.pop();
                }
                Some(())
            }
            Self::RecordPattern(pat) => {
                for prop in &pat.0 {
                    match prop {
                        PropParam::KeyValue(k, v) => {
                            property_access.push(PropertyAccess::Property(k.clone()));
                            v.visit_internal(index, property_access, f)?;
                            property_access.pop();
                        },
                        PropParam::Rest(v) => {
                            unimplemented!("rest")
                        },
                        PropParam::Shorthand(k, v) => {
                            property_access.push(PropertyAccess::Property(k.clone()));
                            f.visit(index, v.name.clone(), property_access.clone())?;
                            property_access.pop();
                        }, 
                    }
                }
                Some(())
            },
            Self::Variable(x) => {
                f.visit(index, x.name.clone(), property_access.clone())
            }
        }
    }
}

impl Pattern {
    pub fn optional(lvalue: VariablePointer, expr: Expr) -> Self {
        Pattern::Optional(Box::new(OptionalPattern(OptionalOp::Optional, LValueOptional::Variable(Box::new(lvalue)), expr)))
    }
}

pub trait PatternVisitor {
    fn visit(&mut self, index: DeclarationIndex, name: SharedString, property_access: Vec<PropertyAccess>) -> Option<()>;
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct OptionalPattern(pub OptionalOp, pub LValueOptional, pub Expr);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum OptionalOp {
    Optional = AssignOp::Assign as u8,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum LValueOptional {
    Variable(Box<VariablePointer>) = 12, // LValue::Variable
}

// ArrayPattern is a subset of Expr::Array
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayPattern(pub Vec<Pattern>);

// RecordPattern is a subset of Expr::Record
#[repr(transparent)]
#[derive(Debug, PartialEq, Clone)]
pub struct RecordPattern(pub Vec<PropParam>);

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum PropParam {
    KeyValue(Box<Field>, Pattern),
    Shorthand(Box<Field>, Box<VariablePointer>),
    Rest(Box<VariablePointer>),
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