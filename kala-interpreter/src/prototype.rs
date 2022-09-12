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

impl PartialEq for Prototype {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Prototype::Function(x, _) => false, // XXX
            Prototype::Array(x) => match other {
                Prototype::Array(y) => x == y,
                _ => false,
            },
            /*
            Prototype::Error(x, _) => match other {
                Prototype::Error(y, _) => x == y,
                _ => false,
            },
            */
            _ => false,
        }
    }
}

pub struct PrototypeFunction {
    environment: EnvironmentRecord<JSValue>,
    identifier: Option<String>,
    parameters: Vec<String>, // TODO: add binding pattern
    body: ast::Statement, // Either a block or a single expression
}

impl PrototypeFunction {
    fn new(
        env: EnvironmentRecord<JSValue>,
        code: ast::FunctionExpression,
    ) -> Self {
        unimplemented!()
    }
}

pub struct PrototypeArray {
    elements: Vec<JSValue>
}