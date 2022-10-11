use kala_context::environment_record::EnvironmentRecord;
use crate::{value::JSValue, lexical::InterpreterF, context::InterpreterContext};
use kala_ast::ast;
use crate::lexical;

#[derive(Debug, Clone)]
pub enum Prototype {
    Function(PrototypeFunction), // function object
    Array(PrototypeArray), // object with array storage
    // TypedArray(Vec<u8>),
    // Error(ErrorType, String), // error object
    // Primitive(), // primitive value wrapper
    // ForeignFunction(Box<dyn Fn(Vec<JSValue>) -> Result<JSValue, String>>), // foreign function object
    // ForeignObject(Box<fdsafdsa>) // foreign object
    // Struct(), // known type struct object, inferred or from type annotation
}

#[derive(Debug, Clone)]
pub struct PrototypeFunction {
    environment: EnvironmentRecord<JSValue>,
    identifier: Option<String>,
    parameters: Vec<String>, // TODO: add binding pattern
    body: ast::Statement<InterpreterF>, // Either a block or a single expression
}

impl PrototypeFunction {
    pub fn new(
        env: EnvironmentRecord<JSValue>,
        code: lexical::Function,
    ) -> Self {
        unimplemented!()
    }

    pub fn call(&self, ctx: &mut InterpreterContext, args: Vec<JSValue>) -> Option<JSValue> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct PrototypeArray {
    pub elements: Vec<JSValue>
}

impl PrototypeArray {
    pub fn at(&self, index: usize) -> JSValue {
        self.elements.get(index).cloned().unwrap_or(JSValue::undefined())
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}