use crate::{value::JSValue, literal::Literal, lexical::{Identifier}};
use kala_context::environment_record::EnvironmentRecord;
pub struct InterpreterContext {
    environment: EnvironmentRecord<JSValue>,
}

impl InterpreterContext {
    pub fn new() -> Self {
        InterpreterContext {
            environment: EnvironmentRecord::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        unimplemented!()
    }

    pub fn exit_scope(&mut self) {
        unimplemented!()
    }

    pub fn declare_mutable_binding(&mut self, binding: &Identifier, value: &JSValue) -> bool {
        unimplemented!()
    }

    pub fn declare_immutable_binding(&mut self, binding: &Identifier, value: &JSValue) -> bool {
        unimplemented!()
    }

    pub fn enter_for_scope(&mut self) {
        unimplemented!()
    }

    pub fn exit_for_scope(&mut self) {
        unimplemented!()
    }

    pub fn loop_scope(&mut self) {
        unimplemented!()
    }

    pub fn completion_signal(&mut self) -> Option<CompletionSignal> {
        unimplemented!()
    }

    pub fn termination_break(&mut self) {
        unimplemented!()
    }

    pub fn termination_continue(&mut self) {
        unimplemented!()
    }

    pub fn termination_return(&mut self, value: &Option<JSValue>) {
        unimplemented!()
    }

    pub fn termination_throw(&mut self, value: &JSValue) {
        unimplemented!()
    }

    pub fn clear_completion_signal(&mut self) {
        unimplemented!()
    }

    pub fn get_binding_value(&mut self, binding: &Identifier) -> Option<JSValue> {
        unimplemented!()
    }

    pub fn set_binding_value(&mut self, binding: &Identifier, value: &JSValue) -> bool {
        unimplemented!()
    }

    pub fn function_environment(&mut self, captures: Vec<Identifier>) -> Option<EnvironmentRecord<JSValue>> {
        self.environment.closure(captures)
    }
}

pub enum CompletionSignal {
    Break,
    Continue,
    Return,
    ReturnValue(JSValue),
    Throw(JSValue),
}