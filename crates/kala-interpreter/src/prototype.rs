use crate::context::{EnvironmentRecord, CompletionSignal};
use crate::declare::declare_binding;
use crate::eval::Eval;
use crate::{value::JSValue, node::InterpreterF, context::InterpreterContext};
use kala_ast::ast;
use kala_context::evaluation_context::DeclarationKind;
use crate::node;

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
    environment: EnvironmentRecord,
    code: node::Function, // TODO: RefCell<lexical::Function>?
}

impl PrototypeFunction {
    pub fn new(
        env: EnvironmentRecord,
        code: node::Function,
    ) -> Self {
        PrototypeFunction {
            environment: env, // capture only neccessary environment
            code: code, // need to share the code with the runtime, change to reference
        }
    }

    pub fn call(&mut self, ctx: &mut InterpreterContext, args: Vec<JSValue>) -> Option<JSValue> {
        let mut local_ctx = InterpreterContext::from_environment(self.environment.clone()); // i really dont want to use clone here...

        for (i, arg) in args.iter().enumerate() {
            // TODO: handle argn mismatch
            declare_binding(&mut local_ctx, true, &self.code.function.params[i], &Some(arg.clone()));
        }

        Eval::new(&mut local_ctx).block(&mut self.code.function.body.block);

        println!("completion signal: {:?}", local_ctx.completion_signal());

        match local_ctx.completion_signal() {
            Some(CompletionSignal::ReturnValue(value)) => Some(value),
            Some(CompletionSignal::Return) => Some(JSValue::Undefined),
            Some(CompletionSignal::Throw(value)) => {
                ctx.termination_throw(&value);
                None
            },
            _ => None,
        }
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