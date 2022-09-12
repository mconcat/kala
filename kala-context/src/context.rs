use kala_ast::ast;

use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub type DeclarationKind = ast::DeclarationKind;
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

#[derive(Clone, Debug)]
pub enum BaseVariable<V: JSValue> {
    Variable(Rc<RefCell<(V, bool)>>),
}

impl <V: JSValue> JSVariable<V> for BaseVariable<V> {
    fn new(value: V, mutable: bool) -> Self {
        BaseVariable::Variable(Rc::new(RefCell::new((value, mutable))))
    }

    fn capture(&mut self) {
        // noop
    }

    fn get(&self) -> V {
        match self {
            BaseVariable::Variable(var) => var.borrow().0.clone()
        }
    }

    fn modify(&mut self, f: impl Fn(&mut V)) {
        match self {
            BaseVariable::Variable(var) => {
                let mut var = var.borrow_mut();
                if !var.1 {
                    panic!("Cannot modify immutable variable");
                }
                f(&mut var.0);
            }
        }
    }

    fn set(&mut self, new_value: V) {
        match self {
            BaseVariable::Variable(var) => {
                let mut var = var.borrow_mut();
                if !var.1 {
                    panic!("Cannot modify immutable variable");
                }
                var.0 = new_value;
            }
        }
    }

    fn is_mutable(&self) -> bool {
        match self {
            BaseVariable::Variable(var) => var.borrow().1,
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

pub type Value<Context> = <Context as JSContext>::V; 

pub trait JSContext {
    type V: JSValue;

    // Primitive value constructors
    fn undefined() -> Self::V;

    fn null() -> Self::V;

    fn coerce_boolean(&mut self, v: &Self::V) -> Self::V; // "truthy"
    fn boolean(b: bool) -> Self::V;

    fn coerce_number(&mut self, v: &Self::V) -> Self::V; // ToNumber
    fn number(n: i64) -> Self::V;

    fn bigint(n: i64) -> Self::V;

    fn coerce_string(&mut self, v: &Self::V) -> Self::V; // ToString
    fn string(s: String) -> Self::V;
    
    // Reference value constructors

    // fn coerce_reference(v: Self::V) -> ObjectValue<Self>; // TODO
    fn array(&mut self, elems: Vec<Self::V>) -> Self::V;
    fn object(&mut self, props: Vec<(&String, Self::V)>) -> Self::V;
    fn function(&mut self, captures: Vec<String>, code: &ast::FunctionExpression) -> Self::V;
    
    // Lexical environment
    
    fn block_scope(&mut self, hoisted_funcs: Vec<(String, Self::V)>, body: impl Fn(&mut Self));

    fn call(&mut self, func: &Self::V, args: Vec<Self::V>) -> Result<Self::V, String>;

    // Control flow

    fn control_loop(&mut self, test: impl Fn(&mut Self) -> Self::V, body: impl Fn(&mut Self));

    fn control_branch(&mut self, test: impl Fn(&mut Self) -> Self::V, consequent: impl Fn(&mut Self), alternate: impl Fn(&mut Self));
    fn control_branch_value(&mut self, test: impl Fn(&mut Self) -> Self::V, consequent: impl Fn(&mut Self) -> Self::V, alternate: impl Fn(&mut Self) -> Self::V) -> Self::V;

    fn control_switch(&mut self); // TODO

    fn control_coalesce(&mut self, left: impl Fn(&mut Self) -> Self::V, right: impl Fn(&mut Self) -> Self::V) -> Self::V;



    // Terminators
    
    fn complete_break(&mut self);
    fn is_break(&self) -> bool;

    fn complete_continue(&mut self);
    fn is_continue(&self) -> bool;

    fn complete_return(&mut self);
    fn complete_return_value(&mut self, val: Self::V);
    fn is_return(&self) -> bool;
    fn consume_return(&mut self) -> Option<Self::V>;

    fn complete_throw(&mut self, val: Self::V);
    fn is_throw(&self) -> bool;
    fn consume_throw(&mut self) -> Self::V; 

    // variable access
    fn initialize_mutable_binding(&mut self, name: &String, v: &Option<Self::V>) -> Result<(), String>;
    fn initialize_immutable_binding(&mut self, name: &String, v: &Self::V) -> Result<(), String>;
    fn resolve_binding(&mut self, name: &String) -> Result<Self::V, String>; 
    fn set_binding(&mut self, name: &String, v: Self::V) -> Result<(), String>;

    // memory manipulation
    fn dup(&mut self, val: &Self::V) -> &Self::V;
    fn dup_mut(&mut self, val: &mut Self::V) -> &mut Self::V;

    // boolean operations
    fn op_and(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_or(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_not(&mut self, left: &mut Self::V) -> &mut Self::V;

    // operations 
    // Mutates and stores to the left arguments.
    fn op_add(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_sub(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_mul(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_div(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_mod(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_pow(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;

    fn op_neg(&mut self, left: &mut Self::V) -> &mut Self::V;
    fn op_inc(&mut self, left: &mut Self::V) -> &mut Self::V;
    fn op_dec(&mut self, left: &mut Self::V) -> &mut Self::V;
   
    fn op_bitand(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_bitor(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_bitxor(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;

    fn op_bitnot(&mut self, left: &mut Self::V) -> &mut Self::V;

    fn op_lshift(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_rshift(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_urshift(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;

    fn op_eq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_neq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_lt(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_gt(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_lte(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_gte(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;

    // equality operations
    fn op_strict_eq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;
    fn op_strict_neq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V;

    // object access
    fn object_property(&mut self, obj: &Self::V, property: &String) -> Result<Self::V, String>;
    fn object_property_computed(&mut self, obj: &Self::V, property: &Self::V) -> Result<Self::V, String>;
}
