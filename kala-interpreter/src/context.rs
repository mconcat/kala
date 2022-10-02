use crate::value::JSValue;

pub struct InterpreterContext;

impl context::JSContext for InterpreterContext {
    
}

pub enum CompletionSignal {
    Break,
    Continue,
    Return,
    ReturnValue(JSValue),
    Throw(JSValue),
}