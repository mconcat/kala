use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr, Not, Neg};

use crate::context::InterpreterContext;
use crate::lexical::{self, Identifier};
use crate::lexical::InterpreterF;
use crate::literal::{Literal, BooleanLiteral, NumberLiteral, StringLiteral};
use crate::prototype::{PrototypeFunction, PrototypeArray};
use crate::prototype::Prototype;

use kala_context::environment_record::EnvironmentRecord;
use kala_context::evaluation_context::{self, EvaluationVariable};
use kala_ast::ast;

#[derive(Clone, Debug, PartialEq)]
pub struct JSBoolean(bool);

#[derive(Clone, Debug, PartialEq)]
pub struct JSString(pub String);

impl JSString {
    fn concat(&mut self, other: &Self) -> &mut Self {
        self.0.push_str(&other.0);
        self
    }
}


#[derive(Debug, Clone)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl PartialEq for JSObject {
    fn eq(&self, other: &Self) -> bool {
        return false // Objects are compared by reference always
    }
}

impl ToString for JSObject {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("{");
        for (k, v) in &self.properties {
            s.push_str(&format!("{}:{},", k, v.to_string()));
        }
        s.push_str("}");
        s
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum JSNumber {
    SMI(i32),
 //    Float(),
}

impl ToString for JSNumber {
    fn to_string(&self) -> String {
        match self {
            JSNumber::SMI(i) => i.to_string(),
        }
    }
}

impl JSNumber {
    pub fn equal(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a == b,
        }
    }

    pub fn add(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a += *b
        }
    }

    pub fn sub(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a -= *b
        }
    }

    pub fn mul(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a *= *b
        }
    }

    pub fn div(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a /= *b
        }
    }

    pub fn pow(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a = a.pow(*b as u32)
        }
    }

    pub fn modulo(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a %= *b
        }
    }

    pub fn bit_and(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a &= *b
        }
    }

    pub fn bit_or(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a |= *b
        }
    }

    pub fn bit_xor(&mut self, other: &mut Self) {
        unimplemented!()
        /*
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a ^= b
        }
        */
    }

    pub fn left_shift(&mut self, other: &mut Self) {
        unimplemented!()
        /*
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a <<= b
        }
        */
    }

    pub fn right_shift(&mut self, other: &mut Self) {
        unimplemented!()
        /*
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a >>= b
        }
        */
    }

    pub fn unsigned_right_shift(&mut self, other: &mut Self) {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => *a = (*a as u32 >> *b as u32) as i32
        }
    }

    pub fn strict_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a == b
        }
    }

    pub fn greater_than(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a > b
        }
    }

    pub fn greater_than_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a >= b
        }
    }

    pub fn less_than(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a < b
        }
    }

    pub fn less_than_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (JSNumber::SMI(a), JSNumber::SMI(b)) => a <= b
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JSValue  {
    Undefined,
    Boolean(JSBoolean),
    Number(JSNumber), // TODO: switch to SMINumber<Decimal64>
    String(JSString),
    Object(Rc<RefCell<JSObject>>),
}

impl ToString for JSValue {
    fn to_string(&self) -> String {
        match self {
            JSValue::Undefined => "undefined".to_string(),
            JSValue::Boolean(b) => b.0.to_string(),
            JSValue::Number(n) => match n {
                JSNumber::SMI(i) => i.to_string(),
            },
            JSValue::String(s) => format!("\"{}\"", s.0),
            JSValue::Object(o) => o.borrow().to_string(),
        }
    }
}

impl JSValue {
    pub fn undefined() -> Self {
        JSValue::Undefined
    }

    pub fn is_undefined(&self) -> bool {
        match self {
            JSValue::Undefined => true,
            _ => false,
        }
    }
/*
    pub fn null() -> Self {
        JSValue::Null
    }
*/
    pub fn boolean(b: bool) -> Self {
        JSValue::Boolean(JSBoolean(b))
    }

    pub fn as_boolean(&self) -> Option<&JSBoolean> {
        if let JSValue::Boolean(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn number(n: i32) -> Self {
        JSValue::Number(JSNumber::SMI(n))
    }

    pub fn as_number(&self) -> Option<&JSNumber> {
        if let JSValue::Number(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_mut_number(&mut self) -> Option<&mut JSNumber> {
        if let JSValue::Number(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn string(s: String) -> Self {
        JSValue::String(JSString(s))
    } 

    pub fn as_string(&self) -> Option<&JSString> {
        if let JSValue::String(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn array(elements: Vec<JSValue>) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Array(PrototypeArray{elements})),
            properties: BTreeMap::new(),
        })))
    }

    pub fn object(props: Vec<(Identifier, JSValue)>) -> Self {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert(k.id.clone(), v);
        }
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: None,
            properties: properties,
        })))
    }

    pub fn function(env: EnvironmentRecord<JSValue>, code: lexical::Function) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Function(PrototypeFunction::new(env, code.clone()))),
            properties: BTreeMap::new(),
        })))
    }

    pub fn equal(&self, other: &Self) -> bool {
        match (self, other) {
            (JSValue::Undefined, JSValue::Undefined) => true,
            (JSValue::Boolean(a), JSValue::Boolean(b)) => a == b,
            (JSValue::Number(a), JSValue::Number(b)) => a.equal(b),
            (JSValue::String(a), JSValue::String(b)) => a == b,
            (JSValue::Object(a), JSValue::Object(b)) => unimplemented!(),
            _ => false,
        }
    }
    
    pub fn get_property(&self, key: &lexical::Identifier) -> Option<JSValue> {
        match self {
            JSValue::Object(obj) => {
                let mut obj = obj.borrow_mut();
                obj.properties.get(&key.id).cloned()
            }
            _ => None,
        }
    }

    pub fn set_property(&mut self, key: &lexical::Identifier, value: JSValue) {
        match self {
            JSValue::Object(obj) => {
                let mut obj = obj.borrow_mut();
                obj.properties.insert(key.id.clone(), value); 
            }
            _ => (),
        }
    }

    pub fn get_index(&self, index: i64) -> Option<&JSValue> {
        unimplemented!()
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            JSValue::Undefined => false,
            JSValue::Boolean(b) => b.0,
            JSValue::Number(JSNumber::SMI(0)) => false,
            JSValue::Number(_) => true,
            JSValue::String(s) => s.0.len() > 0,
            JSValue::Object(obj) => true,
        }
    }

    pub fn call(&self, ctx: &mut InterpreterContext, args: Vec<JSValue>) -> Option<JSValue> {
        match self {
            JSValue::Object(obj) => {
                let obj = obj.borrow();
                if let Some(Prototype::Function(func)) = &obj.prototype {
                    func.call(ctx, args)
                } else {
                    None 
                }
            }
            _ => None, // proper error
        }
    }
}

impl Default for JSValue {
    fn default() -> Self {
        JSValue::Undefined
    }
}

impl PartialEq<Literal> for JSValue {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (JSValue::Undefined, Literal::Undefined) => true,
            (JSValue::Boolean(JSBoolean(b1)), Literal::Boolean(BooleanLiteral(b2))) => b1 == b2,
            (JSValue::Number(JSNumber::SMI(n1)), Literal::Number(NumberLiteral::SMI(n2))) => n1 == n2,
            (JSValue::String(JSString(s1)), Literal::String(StringLiteral(s2))) => s1 == s2,
            _ => false,
        }
    }
}

impl evaluation_context::JSValue for JSValue {
    type Variable = EvaluationVariable<Self>;

    fn is_reference(&self) -> bool {
        match self {
            JSValue::Object(_) => true,
            _ => false,
        }
    }
}