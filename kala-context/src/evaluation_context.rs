pub trait JSVariable<V: JSValue>: Clone {
    fn new(value: V, mutable: bool) -> Self;

    fn capture(&mut self);

    fn get(&self) -> V;
    fn modify(&mut self, f: impl Fn(&mut V));
    fn set(&mut self, value: V);

    fn is_mutable(&self) -> bool;
  //  fn is_value(&self) -> bool;
  //  fn is_capture(&self) -> bool;
}

use std::arch::aarch64::ST;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use kala_ast::ast::{self, WhileStatement, SwitchCase, TryStatement, BreakStatement, ContinueStatement, ReturnStatement};

pub type DeclarationKind = kala_ast::common::DeclarationKind;

#[derive(Clone, Debug)]
pub struct Variable<V: JSValue> {
    value: V,
    mutable: bool,
}

impl<V: JSValue> Variable<V> {
    fn new(value: V, mutable: bool) -> Self {
        Variable { value, mutable }
    }
}

// To avoid unneccesary memory allocation, variables are first initialized as LocalVariable(which is just a Variable struct), and promoted to regular variable only when they are shared. 
// Variables are promoted when
// - the variable is referenced by an escaping object
// - the variable is captured by an escaping closure
// - the variable is passed to function call, and the variable is promoted
// TODO: add ownership inference 
#[derive(Clone, Debug)]
pub enum EvaluationVariable<V: JSValue> {
   //  LocalVariable(Variable<V>),
    Variable(Rc<RefCell<Variable<V>>>),
}

impl<V: JSValue> EvaluationVariable<V> {
    fn new(value: V, mutable: bool) -> Self {
        EvaluationVariable::Variable(Rc::new(RefCell::new(Variable::new(value, mutable))))
    }

    fn capture(&mut self) {
        // noop
        // TODO: implement capturing for LocalVariable
    }

    fn get(&self) -> V {
        match self {
            EvaluationVariable::Variable(var) => var.borrow().value.clone()
        }
    }

    fn modify(&mut self, f: impl Fn(&mut V)) {
        match self {
            EvaluationVariable::Variable(var) => {
                let mut var = var.borrow_mut();
                if !var.mutable {
                    panic!("Cannot modify immutable variable");
                }
                f(&mut var.value);
            }
        }
    }

    fn set(&mut self, new_value: V) {
        match self {
            EvaluationVariable::Variable(var) => {
                let mut var = var.borrow_mut();
                if !var.mutable {
                    panic!("Cannot modify immutable variable");
                }
                var.value = new_value;
            }
        }
    }

    fn is_mutable(&self) -> bool {
        match self {
            EvaluationVariable::Variable(var) => var.borrow().mutable,
        }
    }
}

pub trait JSValue: Clone + Sized + Default  {
    type Variable: JSVariable<Self>;

    // value constructors
    //fn undefined() -> Self;

//    fn null() -> Self;

//    fn as_boolean(&self) -> Option<Self::Boolean>;
//    fn boolean(b: bool) -> Self;

//    fn as_number(&self) -> Option<Self::Number>;
//    fn number(n: i64) -> Self;

//    fn as_bigint(&self) -> Option<Self::Bigint>;
//    fn bigint(n: i64) -> Self;

    //fn as_string(&self) -> Option<Self::String>;
//    fn string(s: String) -> Self;

    fn is_reference(&self) -> bool;
/* 
    // error constructors
    fn error(s: String) -> Self;
    fn range_error(s: String) -> Self;
    fn reference_error(s: String) -> Self;
    fn type_error(s: String) -> Self;
*/

    // objects(reference types) requires memory allocation
    // creation of them should be handled in context.
}
/*
pub trait EvaluationContext {
    type F: ast::NodeF;

    // statements
    fn eval_statement(&mut self, s: &mut <Self::F as ast::NodeF>::Statement);

    fn eval_variable_declaration(&mut self, s: &mut ast::VariableDeclaration<Self::F>);
    fn eval_function_declaration(&mut self, s: &mut ast::FunctionDeclaration<Self::F>);
    fn eval_block(&mut self, s: &mut <Self::F as ast::NodeF>::Block);
    fn eval_if(&mut self, s: &mut ast::IfStatement<Self::F>);
    fn eval_for(&mut self, s: &mut ast::ForStatement<Self::F>);
    fn eval_for_of(&mut self, s: &mut ast::ForOfStatement<Self::F>);
    fn eval_while(&mut self, s: &mut ast::WhileStatement<Self::F>);
    fn eval_switch(&mut self, s: &mut ast::SwitchStatement<Self::F>);
    fn eval_try(&mut self, s: &mut ast::TryStatement<Self::F>);
    fn eval_break(&mut self, s: &mut ast::BreakStatement);
    fn eval_continue(&mut self, s: &mut ast::ContinueStatement);
    fn eval_return(&mut self, s: &mut ast::ReturnStatement<Self::F>);
    fn eval_throw(&mut self, s: &mut ast::ThrowStatement<Self::F>);

    // expressions 
    fn eval_expression(&mut self, s: &mut <Self::F as ast::NodeF>::Expression);

    fn eval_literal(&mut self, s: &mut ast::Literal);
    fn eval_array(&mut self, s: &mut ast::ArrayExpression<Self::F>);
    fn eval_object(&mut self, s: &mut ast::ObjectExpression<Self::F>);
    fn eval_function(&mut self, s: &mut <Self::F as ast::NodeF>::Function);
    fn eval_unary(&mut self, s: &mut ast::UnaryExpression<Self::F>);
    fn eval_binary(&mut self, s: &mut ast::BinaryExpression<Self::F>);
    fn eval_logical(&mut self, s: &mut ast::LogicalExpression<Self::F>);
    fn eval_conditional(&mut self, s: &mut ast::ConditionalExpression<Self::F>);
    fn eval_update(&mut self, s: &mut ast::UpdateExpression<Self::F>);
    fn eval_variable(&mut self, s: &mut ast::VariableExpression<Self::F>);
    fn eval_assignment(&mut self, s: &mut ast::AssignmentExpression<Self::F>);
    fn eval_member(&mut self, s: &mut ast::MemberExpression<Self::F>);
    fn eval_call(&mut self, s: &mut ast::CallExpression<Self::F>);
}
*/