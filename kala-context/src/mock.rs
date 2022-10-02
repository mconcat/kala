use crate::evaluation_context as context;
#[derive(Clone, Debug, PartialEq)]
pub enum JSValue {
    Number(i64),
    Undefined,
}

impl Default for JSValue {
    fn default() -> Self {
        JSValue::Undefined
    }
} 

impl context::JSValue for JSValue {
    type Variable = context::BaseVariable<Self>;

    fn is_reference(&self) -> bool {
        false
    }
}

impl JSValue {
    pub fn number(v: i64) -> Self {
        JSValue::Number(v)
    }

    pub fn undefined() -> Self {
        JSValue::Undefined
    }
}