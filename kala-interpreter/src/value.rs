use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr, Not, Neg};

use crate::prototype::PrototypeFunction;
use crate::prototype::Prototype;

use kala_context::environment_record::EnvironmentRecord

#[derive(Clone, Debug, PartialEq)]
pub struct JSBoolean(bool);

#[derive(Clone, Debug, PartialEq)]
pub struct JSString(String);

impl JSString {
    fn concat(&mut self, other: &Self) -> &mut Self {
        self.0.push_str(&other.0);
        self
    }
}


#[derive(PartialEq, Debug, Clone)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
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

pub trait JSNumber: Add + Sub + Mul + Div + Rem + Neg + PartialEq + PartialOrd + ToString + Copy + Clone {

}

#[derive(Clone, Debug, PartialEq)]
pub enum JSValue  {
    Undefined,
    Null,
    Boolean(JSBoolean),
    Number(i64), // TODO: switch to SMINumber<Decimal64>
    String(JSString),
    Object(Rc<RefCell<JSObject>>),
}

impl<N> JSValue<N> {
    pub fn undefined() -> Self {
        JSValue::Undefined
    }

    pub fn null() -> Self {
        JSValue::Null
    }

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

    pub fn number(n: i64) -> Self {
        JSValue::Number(JSNumber::Integer(n))
    }

    pub fn as_number(&self) -> Option<&JSNumber> {
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

    pub fn array(elems: Vec<JSValue>) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Array(elems)),
            properties: BTreeMap::new(),
        })))
    }

    pub fn object(props: Vec<(String, JSValue)>) -> Self {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert(k.clone(), v);
        }
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: None,
            properties: properties,
        })))
    }

    pub fn function(env: EnvironmentRecord<JSValue>, code: ast::FunctionExpression) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Function(PrototypeFunction::new(env, code.clone()))),
            properties: BTreeMap::new(),
        })))
    }


    
    pub fn get_property(&self, key: JSValue) -> Option<&JSValue> {
        match self {
            JSValue::Object(obj) => {
                let mut obj = obj.borrow_mut();
                match key {
                    JSValue::Undefined => return obj.properties.get("undefined"),
                    JSValue::String(s) => return obj.properties.get(&s.0),
                    JSValue::Boolean(JSBoolean(b)) => return obj.properties.get(&b.to_string()),
                    JSValue::Number(n) => {
                        if let Some(Prototype::Array(elems)) = obj.prototype {
                            if let Some(index) = n.to_i64() {
                                if 0 <= index && index < elems.len() as i64 {
                                    return Some(&elems[index as usize])
                                } 
                            }
                        }
                        return obj.properties.get(&n.to_string())
                    }
                    _ => unimplemented!("object key")
                }
            }
            _ => None,
        }
    }

    pub fn set_property(&mut self, key: &JSValue, value: JSValue) {
        match self {
            JSValue::Object(obj) => {
                match key {
                    JSValue::String(s) => {
                        let mut obj = obj.borrow_mut();
                        obj.properties.insert(s.0, value);
                    }
                    _ => {
                        panic!("set_property: key must be a string");
                    }
                }
            }
            _ => (),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            JSValue::Undefined => false,
            JSValue::Null => false,
            JSValue::Boolean(b) => b.0,
            JSValue::Number(JSNumber::NaN) => false,
            JSValue::Number(JSNumber::Integer(0)) => false,
            JSValue::Number(_) => true,
            JSValue::String(s) => s.0.len() > 0,
            JSValue::Object(obj) => true,
        }
    }
}

impl Default for JSValue {
    fn default() -> Self {
        JSValue::Undefined
    }
}

impl runtime::JSValue for JSValue {
    fn error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn range_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn reference_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn type_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn is_reference(&self) -> bool {
        match self {
            JSValue::Object(_) => true,
            _ => false,
        }
    }
}