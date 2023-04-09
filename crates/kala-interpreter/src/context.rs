use crate::{value::JSValue, node::{Identifier}};
use kala_context::environment_record;

pub type EnvironmentRecord = environment_record::EnvironmentRecord<Identifier, JSValue>;

pub struct InterpreterContext {
    environment: EnvironmentRecord,
    completion_signal: Option<CompletionSignal>,
}

// InterpreterContext should be passed as &mut always, 
// to ensure linear execution of the whole program and to avoid possible copying
impl InterpreterContext {
    pub fn new() -> Self {
        InterpreterContext {
            environment: EnvironmentRecord::new(),
            completion_signal: None,
        }
    }

    pub fn from_environment(environment: EnvironmentRecord) -> Self {
        InterpreterContext {
            environment: environment,
            completion_signal: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.environment.enter()
    }

    pub fn exit_scope(&mut self) {
        self.environment.exit()
    }

    pub fn declare_mutable_binding(&mut self, binding: &Identifier, value: &Option<JSValue>) -> Result<(), String> {
        self.environment.initialize_mutable_binding(binding, value.clone())
    }

    pub fn declare_immutable_binding(&mut self, binding: &Identifier, value: &JSValue) -> Result<(), String> {
        self.environment.initialize_immutable_binding(binding, value.clone())
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
        self.completion_signal.clone() 
    }

    pub fn termination_break(&mut self) {
        self.completion_signal = Some(CompletionSignal::Break);
    }

    pub fn termination_continue(&mut self) {
        self.completion_signal = Some(CompletionSignal::Continue);
    }

    pub fn termination_return(&mut self, value: &Option<JSValue>) {
        match value {
            Some(v) => self.completion_signal = Some(CompletionSignal::ReturnValue(v.clone())),
            None => self.completion_signal = Some(CompletionSignal::Return),
        }
    }

    pub fn termination_throw(&mut self, value: &JSValue) {
        self.completion_signal = Some(CompletionSignal::Throw(value.clone()));
    }

    pub fn clear_completion_signal(&mut self) {
        self.completion_signal = None;
    }

    pub fn get_binding_value(&mut self, binding: &Identifier) -> Option<JSValue> {
        self.environment.resolve_binding(binding)
    }

    pub fn set_binding_value(&mut self, binding: &Identifier, value: &JSValue) -> bool {
        unimplemented!()
    }

    pub fn function_environment(&mut self, _captures: Vec<Identifier>) -> Option<EnvironmentRecord> {
        // TODO: implement free variable analysis
        // for now we just capture the whole environment that is
        // accessible from the current scope
        // this will cause overhead proportional to the size of the environment
        // 
        // this overhead could be optimized by only capturing 
        // the variables that are actually used in the function, in JIT style.

        self.environment.closure_with_global_capture()
    }
}

#[derive(Clone, Debug)]
pub enum CompletionSignal {
    Break,
    Continue,
    Return,
    ReturnValue(JSValue),
    Throw(JSValue),
}