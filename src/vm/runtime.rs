

/*
// traits do not have runtime overhead(unless &dyn), so we use trait wrappers.

pub trait Undefined {
    fn new() -> Self;
}

pub struct MockUndefined {

}

impl Undefined for MockUndefined {
    fn new() -> Self {
        MockUndefined {}
    }
}

pub trait Null {
    fn new() -> Self;
}

pub struct MockNull {

}

impl Null for MockNull {
    fn new() -> MockNull {
        MockNull{}
    }
}

// ECMA boolean value
pub trait Boolean {
    pub fn new(value: bool) -> Self;

    fn and(&mut self, other: &Self) -> OpResult;
    fn or(&mut self, other: &Self) -> OpResult;
    fn not(&mut self) -> OpResult;
    fn xor(&mut self, other: &Self) -> OpResult;
}

pub fn boolean(value: bool) -> Box<dyn Boolean> {
    Box::new(MockBoolean::new(value))
}

pub struct MockBoolean {
    value: bool,
}

impl Boolean for MockBoolean {
    fn new(value: bool) -> Self {
        MockBoolean { value }
    }

    fn and(&mut self, other: &Self) -> OpResult {
        ok(*self.value = self.value && other.value)
    }

    fn or(&mut self, other: &Self) -> OpResult {
        ok(*self.value = self.value || other.value)
    }

    fn not(&mut self) -> OpResult {
        ok(*self.value = !self.value)
    }

    fn xor(&mut self, other: &Self) -> OpResult {
        ok(*self.value = self.value ^ other.value)
    }
}

pub enum OpResult {
    Ok,
    TypeError,
}

fn done() -> OpResult {
    OpResult::Ok
}

#[inline]
fn ok(value: ()) -> OpResult {
    OpResult::Ok
}

#[inline]
fn type_error() -> OpResult {
    OpResult::TypeError
}

// ECMA number/bigint value
// Integer values are represented as a 53-bit integer, without any fractional part.
// Bigint values are represented as a vector of 64-bit integers with a sign flag.
pub trait Numeric {

}

pub enum MockNumeric {
    NaN,
    Infinity(bool),
    Integer(i64),
    Bigint(bool, Vec<i64>),
}

impl MockNumeric {
    #[inline]
    fn assign(&mut self, other: Self) -> OpResult {
        *self = other;
        Ok
    }

    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            MockNumeric::NaN => 0,
            MockNumeric::Infinity(_) => 0,
            MockNumeric::Integer(n) => {
                let i = *n % 4294967296;
                if i > 2147483648 {
                    i - 4294967296 
                } else {
                    i
                }
            },
            _ => 0, // TODO XXX
        }
    }

    #[inline]
    fn to_uint32(&self) -> u32 {
        match self {
            MockNumeric::NaN => 0,
            MockNumeric::Infinity(_) => 0,
            MockNumeric::Integer(n) => {
                *n % 4294967296;
            },
            _ => 0, // TODO XXX
        }
    }
}

impl Numeric for MockNumeric {
 
}





/* 
V128(i64, i64),
V256(i64, i64, i64, i64),
// VInf(bool, u8, Vec<i64>),
}
*/

pub trait String {
    fn concat(&mut self, other: &Self) -> OpResult;
}

pub struct MockString {
    value: str
}

impl String for MockString {
    #[inline]
    fn concat(&mut self, other: &Self) -> OpResult {
        self.value.push_str(other.value);
        done()
    }
}

/*
enum String {
    Short(i64), // ascii string less than length 8
    Vector(Vec<u16>), // UTF-16 string
    // GlobalConst(i64),
}
*/

pub trait Property {
    fn get(&self) -> &Value;
    fn set(&self, val: &Value);
}

pub trait Class {
    fn add_property(&mut self, name: &str, prop: Box<Property>);
}

pub trait Array {
    fn get(&self, index: i64) -> &Value;
    fn set(&self, index: i64, val: &Value);
}

pub trait Object {
    //fn class(&self) -> &Class; // hiddenclass
    
    fn property(&self, name: &str) -> Option<&Property>;
    //fn property_known(&self, id: &i32) -> Option<&Property>;
    fn has_property(&self, name: &str) -> bool;
    //fn has_property_known(&self, id: &i32) -> bool;
    fn delete_property(&self, name: &str) -> bool;
    //fn delete_property_known(&self, id: &i32) -> bool;
    
    fn array(&self) -> Option<&Array>;
}

pub struct MockProperty {
    value: &mut Value
}

impl Property for MockProperty {
    #[inline]
    fn get(&self) -> &Value {
        self.value
    }

    #[inline]
    fn set(&self, val: &Value) {
        *self.value = val
    }
}

pub struct MockObject {
    properties: HashMap<str, MockValue>,
    array: Vec<MockValue>,
}

impl Object for MockObject {
    #[inline]
    fn property(&self, name: &str) -> Option<&Property> {
        self.properties.get(name)
    }
    
    #[inline]
    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
    
    #[inline]
    fn delete_property(&self, name: &str) -> bool {
        self.properties.remove(name).is_some()
    }
    
    #[inline]
    fn array(&self) -> Option<&Array> {
        self
    }
}

pub trait Closure {
    fn call(&self, args: &[Value]) -> Value;
}

pub struct MockClosure {

}

pub enum MockValue {
    Null,
    Undefined,
    Boolean(MockBoolean),
    Number(MockNumeric),
    String(MockString),
    Object(MockObject),
    Closure(MockClosure),
}

/*
// Runtime represented value. 
pub enum Value<
    B: Boolean,
    N: Numeric, 
    S: String,
    O: Object,
    P: Property,
    C: Closure,
> {
    Undefined,
    Null,
    Boolean(B),
    Integer(N),
    String(S),
    Object(O),
    Property(P),
    Closure(C),
}
*/
*/

#[path = "../gen/nessie.ast.rs"]
mod ast;

use std::collections::HashMap;
use std::collections::HashSet;

pub type DeclarationKind = ast::DeclarationKind;

pub trait JSBoolean {
    fn to_bool(&self) -> bool;
}

pub trait JSNumeric {
    fn add(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn sub(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn mul(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn div(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn modulo(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn pow(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn bitand(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn bitor(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn bitxor(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn bitnot(&mut self) -> Result<&mut Self, String>;
    fn lshift(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn rshift(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn urshift(&mut self, other: &Self) -> Result<&mut Self, String>;
    fn equal(&self, other: &Self) -> bool;
    fn not_equal(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
}

pub trait JSNumber: JSNumeric {
}

pub trait JSBigint: JSNumeric {
}

pub trait JSString {
    fn concat(&mut self, other: &Self) -> &mut Self;
}

pub trait JSPropName {
    fn new(name: String) -> Self;

    fn to_string(&self) -> String;
}

pub trait JSProperty {
    type V: JSValue;
        
    fn get(&self) -> Self::V;
    fn set(&self, value: Self::V);
}

pub trait JSReference {
    type V: JSValue;
    type N: JSPropName;
    type P: JSProperty;
//    type Iter: Iterator<Item=Self>;

    fn property(&self, name: &Self::N) -> Self::P; 
    fn call(&self, args: &[Self::V]) -> Self::V;
    // fn set_method(&self, name: &Self::N, val: Self::M);
    
    // iterators
    // property_iter coerces self to an object and iterates over the properties
    // fn property_iter(&self) -> Self::Iter;
    // element_iter coerces self to an array and iterates over the elements
    // fn element_iter(&self) -> Self::Iter;
}

pub trait JSClosure {
    type V: JSValue;

    fn call(&self, args: &[Self::V]) -> Self::V;
}

pub trait JSValue {
    type N: JSNumber;
    // type B: Bigint;
    type S: JSString;
    type R: JSReference;

    // Type switch
    // I am not sure if this is the best way to do this.
    // use it when really needed
    fn type_match<T>(&self, 
        if_null: T,
        if_undefined: T,
        if_boolean: &dyn Fn(bool) -> T,
        if_number: &dyn Fn(&Self::N) -> T,
        // if_bigint: dyn Fn(&Self) -> &'a T,
        if_string: &dyn Fn(&Self::S) -> T,
        if_object: &dyn Fn(&Self::R) -> T,
    ) -> T;

    // Type cast
    // Returns None if the value is not of the given type.
    fn as_boolean(&self) -> Option<bool>;
    fn as_number(&self) -> Option<&Self::N>;
    // fn as_bigint(&self) -> Option<&Self::B>;
    fn as_string(&self) -> Option<&Self::S>;

    fn as_reference(&self) -> Option<&Self::R>;

    // Type coersion
    // Type coersion as defined in https://262.ecma-international.org/9.0/#sec-type-conversion
    // panics if the value is not coercible to the given type.
    fn to_boolean(&self) -> bool;
    fn to_integer(&self) -> &Self::N;
    fn to_string(&self) -> &Self::S;
    fn to_object(&self) -> &Self::R;
}

pub enum Completion<V: JSValue> {
    Continue,
    Break,
    Return(Option<V>),
    Throw(V),
}

// equivalent to Fn(params: Vec<impl Value>) -> Completion<impl Value>
pub struct DelayedEvaulation<Ctx: JSContext> {
    env: HashMap<String, Ctx::V>, // TODO: generalized over arbitrary map
    func: &dyn Fn(Ctx, Vec<Ctx::V>) -> Completion<Ctx::V>,
}

impl<Ctx> DelayedEvaulation<Ctx> {

}

pub trait JSContext {
    type V: JSValue;

    ///////////////////////////////
    // Statements

    // Block scope.
    // 1. Holds a reference to the parent scope
    // 2. Constructs a new scope for the current execution context
    // 3. Hoist all the function declarations in the current execution context using parameter hoist
    // 4. Recover the parent scope after the execution context has finished
    fn block_scope(&self, hoisted_fns: Vec<(String, Self::V)>, body: &dyn Fn());

    fn extract_free_variables(&self, vars: HashSet<String>) -> HashSet<String>;

    // Variable declaration
    // Declare a new variable in the current scope
    fn declare_const_variable(&mut self, name: String, v: Self::V);
    fn declare_let_variable(&mut self, name: String, v: Option<Self::V>);

    // Control flow
    fn control_loop(&mut self, test: &dyn Fn() -> Self::V, body: &dyn Fn());
    // control_branch checks the truthy/falsy value of the condition and branches accordingly
    fn control_branch(&mut self, test: &dyn Fn() -> Self::V, consequent: &dyn Fn(), alternate: &dyn Fn());
    fn control_branch_value(&mut self, test: &dyn Fn() -> Self::V, consequent: &dyn Fn() -> Self::V, alternate: &dyn Fn() -> Self::V) -> Self::V;
    fn control_switch(&mut self); // TODO
    // fn control_try(&mut self, body: &ast::Block, catch: &ast::Block, finally: &ast::Block);
    fn control_coalesce(&mut self, left: &dyn Fn() -> Self::V, right: &dyn Fn() -> Self::V) -> Self::V;

    // Terminators
    fn complete_break(&mut self);
    fn complete_continue(&mut self);
    fn complete_return(&mut self, val: Option<Self::V>);
    fn complete_throw(&mut self, val: Self::V);
    fn completion(&self) -> Option<Completion<Self::V>>;

    ///////////////////////////////
    // Expression

    // Literal value creation
    // XS_CODE_UNDEFINED
    fn new_undefined() -> Self::V;
    // XS_CODE_NULL
    fn new_null() -> Self::V;
    // XS_CODE_TRUE
    // XS_CODE_FALSE
    fn new_boolean(b: bool) -> Self::V;
    // XS_CODE_NUMBER
    fn new_number(n: i64) -> Self::V;
    // XS_CODE_BIGINT
    // fn new_bigint(n: &[i32]) -> Self::V;
    // XS_CODE_STRING
    fn new_string(s: String) -> Self::V;

    // XS_CODE_ARRAY
    fn new_array(elements: &[Self::V]) -> Self::V;

    // Object value creation
    // XS_CODE_OBJECT
    fn new_object() -> Self::V;

    // Function value creation
    fn new_function(&self, identifier: Option<String>, parameters: Vec<String>, body: ast::FunctionExpression, captures: Vec<String>) -> Self::V;

    // variable access
    fn initialize_binding(&self, kind: ast::DeclarationKind, name: String, v: Option<Self::V>);
    fn resolve_binding(&self, name: String) -> Option<Self::V>; 
    fn set_binding(&mut self, name: String, v: Self::V) -> Result<(), &str>;
}