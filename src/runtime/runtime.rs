

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

use std::collections::HashSet;
use crate::ast;

pub type DeclarationKind = ast::DeclarationKind;



/*
impl<S: JSString, N: JSNumber> PropName<S, N> {
    pub fn to_string(&self) -> String {
        match self {
            PropName::String(s) => s.to_string(),
            PropName::Number(n) => n.to_string(),
        }
    }
}
*/
/*
pub trait JSProperty {
    type V: JSValue;
        
    fn get(&self) -> Self::V;
    fn set(&mut self, value: Self::V);
}
*/
/* 
pub trait JSReference {
    //type Context: JSContext;
//    type Iter: Iterator<Item=Self>;
    type V: JSValue;

    fn property(&self, name: 
        PropName<
            <Self::V as JSValue>::S,
            <Self::V as JSValue>::N,
        >
    ) -> <Self::V as JSValue>::P; 
    //fn call(&self, ctx: &mut Self::Context, args: &[<Self::Context as JSContext>::V]) -> <Self::Context as JSContext>::V;
    // fn set_method(&self, name: &Self::N, val: Self::M);
    
    // iterators
    // property_iter coerces self to an object and iterates over the properties
    // fn property_iter(&self) -> Self::Iter;
    // element_iter coerces self to an array and iterates over the elements
    // fn element_iter(&self) -> Self::Iter;
}
*/

/*
pub enum ObjectKind {
    Array,
    ArrayIterator(ArrayIterator),
    // ArrayBuffer(ArrayBuffer),
    // Map(OrderedMap<JsValue>),
    // MapIterator(MapIterator),
    BigInt(JsBigInt),
    Boolean(bool),
    // DataView(DataView),
    // ForInIterator(ForInIterator),
    Function(Function),
    // Set(OrderedSet<JsValue>),
    // SetIterator(SetIterator),
    String(JsString),
    StringIterator(StringIterator),
    Number(f64),
    Symbol(JsSymbol),
    Error,
    Ordinary,
    // Date(Date),
    NativeObject(Box<dyn NativeObject>),
    // IntegerIndexed(IntegerIndexed), // TypedArray
    #[cfg(feature = "intl")]
    // DateTimeFormat(Box<DateTimeFormat>),
    // Promise(Promise),
}
*/

/*
pub trait JSBoolean<V: JSValue> {
    fn wrap(self) -> V;
}

pub trait JSNumber<V: JSValue> {
    fn wrap(self) -> V;

 
    fn to_i64(&self) -> Option<i64>;
}

pub trait JSBigint<V: JSValue> {
    fn wrap(self) -> V;

    fn op_add(&mut self, other: &Self) -> &mut Self;
    fn op_sub(&mut self, other: &Self) -> &mut Self;
    fn op_mul(&mut self, other: &Self) -> &mut Self;
    fn op_div(&mut self, other: &Self) -> &mut Self;
    fn op_mod(&mut self, other: &Self) -> &mut Self;
   
    fn op_neg(&mut self) -> &mut Self;
    fn op_inc(&mut self) -> &mut Self;
    fn op_dec(&mut self) -> &mut Self;

    // TODO: bit operations
   
    fn op_lt(&self, other: &Self) -> bool;
    fn op_gt(&self, other: &Self) -> bool;
    fn op_lte(&self, other: &Self) -> bool;
    fn op_gte(&self, other: &Self) -> bool;
}

pub trait JSString<V: JSValue> {
    fn wrap(self) -> V;

    fn op_concat(&mut self, other: &Self) -> &mut Self;
}
*/
pub trait JSObject<V: JSValue> {
    fn wrap(self) -> V;

}
/*
// Function-like objects, Functions, Closures, Native functions, etc.
pub trait JSCallable<V: JSValue> {
    fn check(v: V) -> bool;
    fn cast(v: V) -> Option<Self>;
    fn coerce(v: V) -> Self;
    fn wrap(&self) -> V;

}

// Array-like objects, Array, TypedArray, Custom Iterators, String Iterators, etc.
pub trait JSIndexible<V: JSValue> {
    fn check(v: V) -> bool;
    fn cast(v: V) -> Option<Self>;
    fn coerce(v: V) -> Self;
    fn wrap(&self) -> V;

    fn get_index(&self, index: i64) -> V;
    fn set_index(&mut self, index: i64, value: V);
}
*/
pub trait JSError<V: JSValue> {
    fn wrap(self) -> V;

    // TODO
}



pub trait JSValue: Clone + Sized  {

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

    // error constructors
    fn error(s: String) -> Self;
    fn range_error(s: String) -> Self;
    fn reference_error(s: String) -> Self;
    fn type_error(s: String) -> Self;


    // objects(reference types) requires memory allocation
    // creation of them should be handled in context.
}

type Value<Context: JSContext> = <Context as JSContext>::V; 
struct InternalValue<V: JSValue>(V);

impl<V: JSValue> InternalValue<V> {
    fn wrap(v: V) -> Self {
        InternalValue(v)
    }

    fn unwrap(&self) -> V {
        self.0
    }
}

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
    fn object(&mut self, props: Vec<(String, Self::V)>) -> Self::V;
    fn function(&mut self, captures: Vec<String>, code: ast::FunctionExpression) -> Self::V;
    
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

    fn op_eq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V
    fn op_neq(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V
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
