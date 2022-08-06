use core::panic;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env::VarError;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use std::collections::HashSet;
//use crate::lexical;
use crate::ast;
use crate::runtime;

/*
// Entry point
fn eval(expr: ast::Expression) -> JSValue {
    lexical::eval_expression(&JSContext::new(), &expr)
}
*/

#[derive(PartialEq, Eq, Debug)]
pub enum JSValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(JSNumber),
    // Bigint(i64, i64), // TODO
    String(JSString),
    
    Reference(JSReference),

//    External(JSExternal),
}

// JSExternal corresponds to WASM externals, interface for native functions.
#[derive(PartialEq, Eq, Debug)]
pub struct JSExternal {
    pub name: String,
}

impl JSValue {
    pub fn new_undefined() -> JSValue {
        JSValue::Undefined
    }

    pub fn new_null() -> JSValue {
        JSValue::Null
    }

    pub fn new_number(value: i64) -> JSValue {
        JSValue::Number(JSNumber::Integer(value))
    }

    pub fn new_string(value: &str) -> JSValue {
        JSValue::String(JSString::new(value.to_string()))
    }


    pub fn is_truthy(&self) -> bool {
        match self {
            JSValue::Undefined => false,
            JSValue::Null => false,
            JSValue::Boolean(b) => *b,
            JSValue::Number(JSNumber::NaN) => false,
            JSValue::Number(JSNumber::Integer(0)) => false,
            JSValue::Number(_) => true,
            JSValue::String(s) => !s.is_empty(),
            JSValue::Reference(r) => true,
        }
    }
}

impl runtime::JSValue for JSValue {
    type N = JSNumber;
    // type B = Bigint;
    type S = JSString;
    type R = JSReference;
/*
    fn type_match<T>(&self, 
            if_null: T,
            if_undefined: T,
            if_boolean: impl Fn(bool) -> T,
            if_number: impl Fn(Self::N) -> T,
            // if_bigint: dyn Fn(&Self) -> &'a T,
            if_string: impl Fn(Self::S) -> T,
            if_object: impl Fn(Self::R) -> T,
        ) -> T {
        match self {
            JSValue::Null => if_null,
            JSValue::Undefined => if_undefined,
            JSValue::Boolean(b) => if_boolean(*b),
            JSValue::Number(n) => if_number(*n),
            // JSValue::Bigint(n, _) => if_bigint(n),
            JSValue::String(s) => if_string(*s),
            JSValue::Reference(r) => if_object(*r),
        }
    }
*/
    fn as_boolean(&self) -> Option<bool> {
        match self {
            JSValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    fn as_number(&self) -> Option<&Self::N> {
        match self {
            JSValue::Number(n) => Some(n),
            _ => None,
        }
    }

    fn as_string(&self) -> Option<&Self::S> {
        match self {
            JSValue::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_reference(&self) -> Option<&Self::R> {
        match self {
            JSValue::Reference(r) => Some(r),
            _ => None,
        }
    }

    fn to_boolean(&self) -> bool {
        self.is_truthy()
    }

    fn to_number(&self) -> Self::N {
        match self {
            JSValue::Undefined => JSNumber::NaN,
            JSValue::Null => JSNumber::Integer(0),
            JSValue::Boolean(b) => JSNumber::Integer(if *b { 1 } else { 0 }),
            JSValue::Number(n) => *n,
            JSValue::String(_) => panic!("Cannot convert to integer"),
            JSValue::Reference(_) => panic!("Cannot convert to integer"),
        }
    }
/*
    fn to_string(&self) -> Self::S {
        match self {
            JSValue::Undefined => JSString::new("undefined".to_string()),
            JSValue::Null => JSString::new_short("null"),
            JSValue::Boolean(b) => JSString::new_short(if *b { "true" } else { "false" }),
            JSValue::Number(n) => JSString::new(n.to_string()),
            JSValue::String(s) => *s,
            JSValue::Reference(r) => panic!("Cannot convert to string"),
        }
    }
    

    fn to_object(&self) -> Self::R {
        match self {
            JSValue::Reference(r) => *r,
            _ => unimplemented!("Type conversion to object"),
        }
    }
*/
    fn dup(&self) -> &Self {
        self
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum JSNumber {
    NaN,
    Infinity(bool), // true = positive, false = negative
    Integer(i64),
}

use JSNumber::NaN;
use JSNumber::Infinity;
use JSNumber::Integer;

impl JSNumber {
    #[inline]
    fn new(value: i64) -> JSNumber {
        JSNumber::Integer(value)
    }
 
    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Integer(i) => i as i32, // TODO
            _ => 0,
        }
    }

    #[inline]
    fn to_string(&self) -> String {
        match self {
            &Integer(i) => i.to_string(),
            &NaN => "NaN".to_string(),
            &Infinity(true) => "Infinity".to_string(),
            &Infinity(false) => "-Infinity".to_string(),
        }
    }


    const MAX_SAFE_INTEGER: i64 = 9007199254740991;
    const MIN_SAFE_INTEGER: i64 = -9007199254740991;

    fn check_overflow(&mut self) -> &mut Self {
        match self {
            Integer(i) => {
                if *i > Self::MAX_SAFE_INTEGER  {
                    *i = Self::MAX_SAFE_INTEGER;
                }
            }
            _ => {}
        }
        self
    }

    fn check_underflow(&mut self) -> &mut Self {
        match self {
            Integer(i) => {
                if *i < Self::MIN_SAFE_INTEGER  {
                    *i = Self::MIN_SAFE_INTEGER;
                }
            }
            _ => {}
        }
        self
    }
}

impl runtime::JSNumeric for JSNumber {
    #[inline]
    fn op_add(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x != *y {
                        *self = NaN;
                    }
                }
                _ => {}, // ignore other cases
            }
            Integer(x) => match other {
                Infinity(_) => { *self = *other },
                Integer(y) => { *x += *y; self.check_overflow(); },
                NaN => { *self = NaN },
            },
        };

        self
    }

    #[inline]
    fn op_sub(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x == *y {
                        *self = NaN
                    }
                }
                _ => {}, // ignore other cases
            }
            Integer(x) => match other {
                Infinity(y) => { *y = !*y },
                Integer(y) => { *x -= *y; self.check_underflow(); },
                NaN => { *self = NaN },
            },
        };

        self
    }

    #[inline]
    fn op_mul(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => match other {
                Infinity(y) => { *x = *x == *y },
                Integer(0) => { *self = NaN },
                Integer(y) => { *x = *x == (*y>=0) },
                NaN => { *self = NaN },
            }
            Integer(0) => match other {
                Infinity(_) => { *self = NaN },
                Integer(_) => {},
                NaN => { *self = NaN }, 
            },
            Integer(x) => match other {
                Infinity(y) => { *self = Infinity((*x>=0) == *y) },
                Integer(y) => { *x = *x * *y; Ok(self) },
                NaN => { *self = NaN },
                _ => panic!("should not reach here"), 
            },
        };

        self
    }

    #[inline]
    fn op_div(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => Ok(self),
            Infinity(x) => match other {
                Infinity(y) => self.assign(NaN),
                Integer(y) => { *x = *x == (*y>=0); Ok(self) },
                NaN => self.assign(NaN),
            }
            Integer(0) => match other {
                Integer(0) => self.assign(NaN),
                NaN => self.assign(NaN),
                _ => Ok(self),
            },
            Integer(x) => match other {
                Integer(0) => self.assign(Infinity(*x>=0)),
                Infinity(y) => { *x = 0; Ok(self) },
                Integer(y) => { *x = *x / *y; Ok(self) },
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
        }
    }

    #[inline]
    fn op_modulo(&mut self, other: &Self) -> Result<&Self, String> {
        match self {
            NaN => Ok(self),
            Infinity(x) => self.assign(NaN),
            Integer(x) => match other {
                Infinity(y) => Ok(self),
                Integer(0) => self.assign(NaN),
                Integer(y) => { *x = *x % *y; Ok(self) },
                NaN => self.assign(NaN),
                _ => self.type_error(), 
            },
        }
    }

    #[inline]
    fn op_pow(&mut self, other: &Self) -> Result<&Self, String> {
        unimplemented!("asdf")
    } // TODO XXX

    #[inline]
    fn op_bitand(&mut self, other: &Self) -> Result<&Self, String> {
        let v = self.to_int32() & other.to_int32();
        match self {
            Integer(x) => { *x = v as i64; Ok(self) },
            _ => self.assign(Integer(v as i64)),
        }  
    }

    #[inline]
    fn op_bitor(&mut self, other: &Self) -> Result<&Self, String> {
        let v = self.to_int32() | other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        } 
    }

    #[inline]
    fn op_bitxor(&mut self, other: &Self) -> Result<&Self, String> {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            Integer(x) => { *x = v as i64; Ok(self) },
            _ => self.assign(Integer(v as i64)),
        }    
    }

    #[inline]
    fn op_bitnot(&mut self) -> Result<&Self, String> {
        let v = self.to_int32() ^ 0xFFFFFFFF;
        match self {
            Integer(x) => { *x = v as i64; Ok(self) },
            _ => self.assign(Integer(v as i64)),
        }
    }

    #[inline]
    fn op_lshift(&mut self, other: &Self) -> Result<&Self, String> {
        let v = self.to_int32() << other.to_int32();
        match self {
            Integer(x) => { *x = v as i64; Ok(self) }, 
            _ => self.assign(Integer(v as i64)),
        } 
    }

    #[inline]
    fn op_rshift(&mut self, other: &Self) -> Result<&Self, String> {
        let v = self.to_int32() >> other.to_int32();
        match self {
            Integer(x) => { *x = v as i64; Ok(self) },
            _ => self.assign(Integer(v as i64)),
        }   
    }

    #[inline]
    fn op_urshift(&mut self, other: &Self) -> Result<&Self, String> {
        unimplemented!("asdfasdf")
    }

    #[inline]
    fn op_equal(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => true,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x == *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x == *y,
                _ => false,
            },
        }
    }

    #[inline]
    fn op_not_equal(&self, other: &Self) -> bool {
        !self.op_equal(other)
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_lt(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x < *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x < *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_lte(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x <= *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x <= *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_gt(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x > *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x > *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_gte(&self, other: &Self) -> bool {
        match self {
            NaN => match other {
                NaN => false,
                _ => false,
            },
            Infinity(x) => match other {
                Infinity(y) => *x >= *y,
                _ => false,
            },
            Integer(x) => match other {
                Integer(y) => *x >= *y,
                _ => false,
            },
        }
    }

    #[inline]
    fn op_neg(&mut self) -> &Self {
        match self {
            NaN => {},
            Infinity(x) => *x = !*x,
            Integer(x) => *x = -*x,
        };

        self
    }

    #[inline]
    fn op_inc(&mut self) -> &Self {
        match self {
            NaN => self,
            Infinity(x) => self,
            Integer(x) => { *x += 1; self },
        }.check_overflow()
    }

    #[inline]
    fn op_dec(&mut self) -> &Self {
        match self {
            NaN => self,
            Infinity(x) => self,
            Integer(x) => { *x -= 1; self },
        }.check_underflow()
    }
}


impl runtime::JSNumber for JSNumber {
    type V = JSValue;

    fn to_value(&self) -> Self::V {
        JSValue::Number(*self)
    }
}
/* 
pub enum Bigint {
    V128(bool, [u32; 4]),
}

use Bigint::V128;

impl Bigint {
    #[inline]
    fn ok(&self, value: ()) -> Result<&Self, String> {
        Ok(self)
    }

    #[inline]
    fn assign(&mut self, value: Self) -> Result<&Self, String> {
        *self = value;
        Ok(self)
    }

    #[inline]
    fn type_error(&self) -> Result<&Self, String> {
        Err("Type error")
    }

    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Integer(i) => i as i32, // TODO
            _ => 0,
        }
    }
}


impl runtime::Bigint for Bigint {
    #[inline]
    fn add(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            V128(xsign, x) => match other {
                V128(ysign, y) => {
                    let val = x[0] as i64;
                    let carry = 0;
                    val += y[0] * ((xsign != ysign) as i32 * -1);
                    if val < 0 {
                        carry = -1;
                        val = u32::MAX - val;
                    } else if val > u32::MAX {
                        carry = 1;
                        val = val - u32::MAX;
                    }
                    x[0] = val as u32;

                    let val = x[1] as i64;
                    let carry = 0;
                    val += y[0] * ((xsign != ysign) as i32 * -1);
                    if val < 0 {
                        carry = -1;
                        val = u32::MAX - val;
                    } else if val > u32::MAX {
                        carry = 1;
                        val = val - u32::MAX;
                    }
                    x[1] = val as u32;
                }
            },
        }
        self.ok(())
    }
}
*/

#[derive(PartialEq, Eq, Debug)]
pub enum JSString {
    Short(u64), // short ascii String, maximum length is 8 bytes
    Normal(String), // heap allocated arbitrary length UTF-8 String
}

impl JSString {
    #[inline]
    fn new(s: String) -> Self {
        JSString::Normal(s)
    }

    #[inline]
    fn new_short(s: &str) -> Self {
        JSString::Short(u64::from_be_bytes(s.as_bytes().try_into().unwrap()))
    }

    fn to_normal(&mut self) -> &mut Self {
        match self {
            &mut JSString::Short(x) => {
                let bytes: [u8; 8] = x.to_be_bytes();
                *self = JSString::Normal(std::str::from_utf8(bytes.as_ref()).unwrap().to_string());
            }
            _ => {}
        }
        self
    }

    fn as_normal(&self) -> &Self {
        match self {
            JSString::Short(x) => {
                let bytes: [u8; 8] = x.to_be_bytes();
                &JSString::Normal(String::from_utf8(bytes.to_vec()).unwrap())
            }
            _ => self
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            JSString::Short(x) => *x == 0,
            JSString::Normal(x) => x.is_empty(),
        }
    }

    #[inline]
    fn to_string(&self) -> String {
        let JSString::Normal(s) = self.as_normal();
        *s
    }

}

impl runtime::JSString for JSString {
    #[inline]
    fn concat(&mut self, other: &Self) -> &mut Self {
        let JSString::Normal(self_) = self.to_normal();
        for c in other.to_string().chars() {
            self_.push(c);
        }
        self
    }
}

/*
enum Prototype {
    ObjectPrototype,
    FunctionPrototype,
    ArrayPrototype,
    StringPrototype,
    BooleanPrototype,
    NumberPrototype,
    DatePrototype,
    RegExpPrototype,
    // HostPrototype,
    ErrorPrototype,
    RangeErrorPrototype,
    ReferenceErrorPrototype,
    SyntaxErrorPrototype,
    TypeErrorPrototype,
    // ArrayBufferPrototype,
    // DataViewPrototype,
    // TypedArrayPrototype,
    // MapPrototype,
    // SetPrototype,
    // PromisePrototype,
    // ProxyPrototype,
}
*/

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorType {
    Error,
    RangeError,
    ReferenceError,
    SyntaxError,
    TypeError,
}

#[derive(Debug)]
// Jessie does not allow custom prototype definition,
// so we use a single enum for all predefined prototypes.
pub enum Prototype {
    Function(HashMap<String, JSValue>, ast::FunctionExpression), // function object
    Array(Vec<JSValue>), // object with array storage
    // TypedArray(Vec<u8>),
    Error(ErrorType, String), // error object
    // Primitive(),
    // ForeignFunction(Box<dyn Fn(Vec<JSValue>) -> Result<JSValue, String>>), // foreign function object
    // State(String), // state object
}

impl PartialEq for Prototype {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Prototype::Function(x, _) => false, // XXX
            Prototype::Array(x) => match other {
                Prototype::Array(y) => x == y,
                _ => false,
            },
            Prototype::Error(x, _) => match other {
                Prototype::Error(y, _) => x == y,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Eq for Prototype {

}

#[derive(PartialEq, Eq, Debug)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl JSObject {
    #[inline]
    fn new(props: Vec<(&ast::PropName, JSValue)>) -> Self {
        unimplemented!("insert properties");
        JSObject {
            prototype: None,
            properties: BTreeMap::new(),
        }
    }

    #[inline]
    fn new_array(elems: Vec<JSValue>) -> Self {
        JSObject {
            prototype: Some(Prototype::Array(elems)),
            properties: BTreeMap::new(),
        }
    }

    #[inline]
    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: ast::FunctionExpression, captures: Vec<&ScopedVariable>) -> Self {
        unimplemented!()
        /*
        JSObject {
            prototype: Some(Prototype::Function()),
            properties: BTreeMap::new(),
        }
        */
    }
}

impl JSObject {
    fn call(&self, ctx: JSContext, parameters: Vec<JSValue>) -> Option<JSValue> {
        unimplemented!()
        /*
        if let Prototype::Function(env, func) = &self.prototype {
            let scope = Scope::new(env);

            ctx.scope.enter();
            for (i, param) in func.parameters.iter().enumerate() {
                match param.body? {
                    ast::parameter_pattern::Body::Pattern(p) => {
                        match p {
                            ast::pattern::Pattern::Identifier(id) => {
                                ctx.initialize_binding(id.name, ast::DeclarationKind::Let, Some(parameters[i]));
                            }
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            for (name, capture) in env.iter() {
                ctx.initialize_binding(capture.to_string(), ast::DeclarationKind::Let, Some(parameters[i]));
            }

            lexical::eval_statement(&ctx, self.prototype);
            ctx.scope.exit()
        } else {
            panic!("Type error");
        }

        */
    }
}

#[derive(PartialEq, Eq, Debug)]
// Reference is a pointer to a value in the heap.
pub struct JSReference {
    value: Rc<RefCell<JSObject>>
}

impl JSReference {
    fn new(value: JSObject) -> Self {
        JSReference {
            value: Rc::new(RefCell::new(value))
        }
    }

    fn new_array(elems: Vec<JSValue>) -> Self {
        JSReference {
            value: Rc::new(RefCell::new(JSObject::new_array(elems)))
        }
    }

    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: ast::FunctionExpression, captures: Vec<&ScopedVariable>) -> Self {
        JSReference {
            value: Rc::new(RefCell::new(JSObject::new_function(identifier, parameters, body, captures)))
        }
    }
}

impl runtime::JSReference for JSReference {
    type V = JSValue;
    type P = JSProperty;
    type N = JSPropName;
//    type Iter = JSObjectIterator;

    fn property(&self, name: &Self::N) -> Self::P {
        JSProperty::new(*self, name.name)
    }

    fn call(&self, args: &[Self::V]) -> Self::V {
        unimplemented!()
    }
}

pub struct JSPropName {
    name: String,
}

impl runtime::JSPropName for JSPropName {
    fn new(name: String) -> Self {
        JSPropName {
            name,
        }
    }

    fn to_string(&self) -> String {
        self.name
    }
}

// JSProperty is meant to be ephemeral
pub struct JSProperty {
    value: Weak<RefCell<JSObject>>,
    propname: String,
}

impl JSProperty {
    fn new(refer: JSReference, name: String) -> Self {
        JSProperty {
            value: Rc::downgrade(&refer.value),
            propname: name,
        }
    }
}

impl runtime::JSProperty for JSProperty {
    type V = JSValue;

    fn get(&self) -> Self::V {
        match &*(unsafe { &*self.value.as_ptr() }).borrow() {
            JSObject { prototype, properties } => {
                *properties.get(&self.propname).unwrap_or(&JSValue::Undefined) 
            }
            _ => unimplemented!("ToObject"), // TODO
        }
    }

    fn set(&self, value: Self::V) {
        match &*(unsafe { &*self.value.as_ptr() }).borrow() {
            JSObject { prototype, properties } => {
                *properties.get_mut(&self.propname).unwrap() = value;
            }
            _ => unimplemented!("TypeError"), // TODO: return TypeError
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ScopedVariable {
    Stack(JSValue, ast::DeclarationKind, i16),
    Heap(Rc<RefCell<JSValue>>, ast::DeclarationKind, i16),
}

impl ScopedVariable {
    #[inline]
    pub fn new(value: JSValue, kind: ast::DeclarationKind, depth: i16) -> Self {
        Self::Stack(
            value,
            kind,
            depth
        )
    }
    
    #[inline]
    pub fn promote(&mut self) -> &Self {
        match self {
            &mut Self::Stack(value, kind, depth) => {
                *self = Self::Heap(
                    Rc::new(RefCell::new(value)),
                    kind,
                    depth
                )
            },
            _ => {},
        };
        self
    }

    pub fn value(&self) -> JSValue {
        match self {
            &Self::Stack(value, _, _) => value,
            &Self::Heap(value, _, _) => *value.borrow_mut(),
        }
    }

    pub fn set(&mut self, value: JSValue) {
        match self {
            Self::Stack(existing_value, _, _) => *existing_value = value,
            Self::Heap(existing_value, _, _) => *existing_value.borrow_mut() = value,
        }
    }

    pub fn kind(&self) -> ast::DeclarationKind {
        match self {
            &Self::Stack(_, kind, _) => kind,
            &Self::Heap(_, kind, _) => kind,
        }
    }

    pub fn depth(&self) -> i16 {
        match self {
            &Self::Stack(_, _, depth) => depth,
            &Self::Heap(_, _, depth) => depth,
        }
    }
}

// Binding: variables that are visible in the current scope.
// Recovery: shadowed parent scope variables
pub struct Scope {
    binding: HashMap<String, ScopedVariable>,
    recovery: Vec<BTreeMap<String, Option<ScopedVariable>>>,
}

impl Scope {
    pub fn new() -> Self {
        // XXX: construct binding from env
        Scope {
            binding: HashMap::new(),
            recovery: Vec::new(),
        }
    }

    #[inline]
    pub fn depth(&self) -> i16 {
        self.recovery.len() as i16
    }

    #[inline]
    pub fn current_recovery(&self) -> &BTreeMap<String, Option<ScopedVariable>> {
        self.recovery.last().unwrap()
    }

    #[inline]
    pub fn variable(&self, name: String) -> Result<&ScopedVariable, String> {
        self.binding.get(&name).ok_or(format!("ReferenceError: {} is not defined", name))
    }

    // variable_mut should be de
    #[inline]
    pub fn variable_mut(&mut self, name: String) -> Result<&mut ScopedVariable, String> {
        match self.binding.get_mut(&name) {
            Some(existing) => {
                if existing.kind() != ast::DeclarationKind::Let {
                    // cannot set to non-let variable
                    Err("TypeError: Assignment to constant variable.".to_string())
                } else {
                    Ok(existing)
                }
            },
            None => Err(format!("ReferenceError: assignment to undeclared variable \"{}\"", name)),
        }  
    }
    
    pub fn declare(&mut self, name: String, kind: ast::DeclarationKind, value: Option<JSValue>) {
        if kind == ast::DeclarationKind::Const && value.is_none() {
            panic!("const variable must be initialized");
        }

        match self.binding.get_mut(&name) {
            Some(existing) => {
                if existing.depth() < self.depth() {
                    // parent scope had declared the variable,
                    // shadow it by adding it to the recovery list
                    self.current_recovery().insert(name, Some(*existing));
                } else if existing.kind() == ast::DeclarationKind::Const {
                    // current scope had declared the variable as const,
                    // cannot override const variables
                    unimplemented!("SyntaxError: redeclaration of formal parameter \"{}\"", name)
                } else {
                    // current scope had declared the variable as let,
                    // by override the existing, the rust compiler will automatically drop the existing reference(possibly Rc).
                    // noop
                }
                *existing = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
            },
            None => {
                // there is no variable declared with this name
                // add the variable as discard to the recovery list
                // and insert the new variable to the binding
                self.current_recovery().insert(name, None);
                self.binding.insert(name, ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth()));
            }
        }
    }

    #[inline]
    pub fn enter(&mut self) {
        self.recovery.push(BTreeMap::new());
    }

    pub fn exit(&mut self) {
        let Some(recovery) = self.recovery.pop();
        for (key, entry) in recovery.iter() {
            match entry {
                Some(existing) => {
                    self.binding.insert(key.clone(), *existing.clone());
                }
                None => {
                    self.binding.remove(key);
                }
            }
        }
    }
}

pub struct JSContext {
    scope: Scope,
    completion: Option<runtime::Completion<JSValue>>,
}

impl JSContext {
    fn new() -> Self {
        JSContext {
            scope: Scope::new(),
            completion: None,
        }
    }

    fn set_completion(&mut self, completion: runtime::Completion<JSValue>) {
        self.completion = Some(completion);
    }
}

impl runtime::JSContext for JSContext {
    type V = JSValue;

    fn check_early_errors(&self) -> bool {
        false // TODO
    }



    fn block_scope(&self, hoisted_fns: Vec<(String, Self::V)>, body: impl Fn()) {
        self.scope.enter();
        body();
        self.scope.exit();
    }

    fn extract_free_variables(&self, vars: HashSet<String>) -> HashSet<String> {
        for var in vars.iter() {
            if self.scope.variable(*var).is_ok() {
                vars.remove(var);
            }
        }
        
        vars
    }

    #[inline]
    fn declare_const_variable(&mut self, name: String, v: Self::V) {
        self.scope.declare(name, ast::DeclarationKind::Const, Some(v))
    }

    #[inline]
    fn declare_let_variable(&mut self, name: String, v: Option<Self::V>) {
        self.scope.declare(name, ast::DeclarationKind::Let, v)
    }

    #[inline]
    fn control_loop(&mut self, test: impl Fn() -> Self::V, body: impl Fn()) {
        while test().is_truthy() {
            body();
        }
    }

    #[inline]
    fn control_branch(&mut self, test: impl Fn() -> Self::V, consequent: impl Fn(), alternate: impl Fn()) {
        if test().is_truthy() {
            consequent();
        } else {
            alternate();
        }
    }

    #[inline]
    fn control_branch_value(&mut self, test: impl Fn() -> Self::V, consequent: impl Fn() -> Self::V, alternate: impl Fn() -> Self::V) -> Self::V {
        if test().is_truthy() {
            consequent()
        } else {
            alternate()
        }
    }

    #[inline]
    fn control_switch(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn control_coalesce(&mut self, left: impl Fn() -> Self::V, right: impl Fn() -> Self::V) -> Self::V {
        let result = left();
        match result {
            JSValue::Undefined => right(),
            JSValue::Null => right(),
            _ => result,
        }
    }

    #[inline]
    fn complete_break(&mut self) {
        self.set_completion(runtime::Completion::Break);
    }

    #[inline]
    fn complete_continue(&mut self) {
        self.set_completion(runtime::Completion::Continue);
    }

    #[inline]
    fn complete_return(&mut self, v: Option<Self::V>) {
        self.set_completion(runtime::Completion::Return(v));
    }

    #[inline]
    fn complete_throw(&mut self, v: Self::V) {
        self.set_completion(runtime::Completion::Throw(v));
    }

    #[inline]
    fn completion(&self) -> Option<runtime::Completion<Self::V>> {
        self.completion
    }

    #[inline]
    fn new_undefined(&self) -> Self::V {
        JSValue::Undefined
    }

    #[inline]
    fn new_null(&self) -> Self::V {
        JSValue::Null
    }

    #[inline]
    fn new_boolean(&self, b: bool) -> Self::V {
        JSValue::Boolean(b)
    }

    #[inline]
    fn new_number(&self, n: i64) -> Self::V {
        JSValue::Number(JSNumber::new(n))
    }

    #[inline]
    fn wrap_number(&self, n: &<<Self as runtime::JSContext>::V as runtime::JSValue>::N) -> Self::V {
        JSValue::Number(*n)
    }

    #[inline]
    fn new_string(&self, s: String) -> Self::V {
        JSValue::String(JSString::new(s))
    }

    #[inline]
    fn wrap_string(&self, s: &<<Self as runtime::JSContext>::V as runtime::JSValue>::S) -> Self::V {
        JSValue::String(*s)
    }

    #[inline]
    fn new_array(&self, elements: Vec<Self::V>) -> Self::V {
        JSValue::Reference(JSReference::new_array(elements))
    }

    #[inline]
    fn new_object(&self, props: Vec<(&ast::PropName, Self::V)>) -> Self::V {
        JSValue::Reference(JSReference::new(JSObject::new(props)))
    }

    #[inline]
    fn new_function(&self, identifier: Option<String>, parameters: Vec<String>, body: ast::FunctionExpression, captures: Vec<String>) -> Self::V {
        let captured_variables = captures.iter().map(|v| self.scope.variable_mut(*v).unwrap().promote()).collect();
        JSValue::Reference(JSReference::new_function(identifier, parameters, body, captured_variables))
    }

    fn initialize_binding(&self, kind: ast::DeclarationKind, name: String, value: Option<Self::V>) {
        self.scope.declare(name, kind, value);
    }

    fn resolve_binding(&self, name: String) -> Result<JSValue, String> {
       self.scope.variable(name).map(|v| v.value())
    }

    fn set_binding(&mut self, name: String, value: Self::V) -> Result<(), String>{
        self.scope.variable_mut(name).map(|v| v.set(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::runtime::Scope;
    use crate::interpreter::runtime::JSValue;
    use crate::ast::DeclarationKind;
    
    #[test]
    fn scope_test_simple() {
        let scope = &mut Scope::new();

        let declare_let = |scope: &mut Scope, name: &str, value: Option<JSValue>| {
            scope.declare(name.to_string(), DeclarationKind::Let, value);
        };

        let declare_const = |scope: &mut Scope, name: &str, value: Option<JSValue>| {
            scope.declare(name.to_string(), DeclarationKind::Const, value);
        };

        let set_var = |scope: &mut Scope, name: &str, value: JSValue| {
            scope.variable_mut(name.to_string())
                .map(|v| v.set(value))
                .unwrap();
        };

        let assert_set_error = |scope: &mut Scope, name: &str| {
            assert!(scope.variable_mut(name.to_string()).is_err());
        };

        let assert_var = |scope: &mut Scope, name: &str, value: JSValue| {
            assert_eq!(scope.variable(name.to_string()).unwrap().value(), value);
        };

        /*
        {
            let a = 1;
            const b = 2;
            a = 3;
            // b = 4; // Error!
            let c;
            c = 4;
            {
                const a = 11;
                let b = 12;
                let x = 13;
                c = 14;
            }
            a = 5;
            // b = 6; // Error!
        }
        */

        declare_let(scope, "a", Some(JSValue::new_number(1)));
        assert_var(scope, "a", JSValue::new_number(1));

        declare_const(scope ,"b", Some(JSValue::new_number(2)));
        assert_var(scope, "b", JSValue::new_number(2));

        set_var(scope, "a", JSValue::new_number(3));
        assert_var(scope, "a", JSValue::new_number(3));

        assert_set_error(scope, "b"); // error on const variable set

        declare_let(scope, "c", None);
        assert_var(scope, "c", JSValue::new_undefined());

        set_var(scope, "c", JSValue::new_number(4));
        assert_var(scope, "c", JSValue::new_number(4));

        scope.enter();
        {
            assert_var(scope, "a", JSValue::new_number(3));
            declare_const(scope, "a", Some(JSValue::new_number(11)));
            assert_var(scope, "a", JSValue::new_number(11));

            assert_var(scope, "b", JSValue::new_number(2));
            declare_let(scope, "b", Some(JSValue::new_number(12)));
            assert_var(scope, "b", JSValue::new_number(12));

            assert_var(scope, "x", JSValue::new_undefined());
            declare_let(scope, "x", Some(JSValue::new_number(13)));
            assert_var(scope, "x", JSValue::new_number(13));

            assert_var(scope, "c", JSValue::new_number(4));
            set_var(scope, "c", JSValue::new_number(14));
            assert_var(scope, "c", JSValue::new_number(14));
        }; 
        scope.exit();

        assert_var(scope, "a", JSValue::new_number(3));
        assert_var(scope, "b", JSValue::new_number(2));
        assert_var(scope, "c", JSValue::new_number(14));
        assert_var(scope, "x", JSValue::new_undefined());

        set_var(scope, "a", JSValue::new_number(5));
        assert_var(scope, "a", JSValue::new_number(5));
        assert_set_error(scope, "b"); // error on const variable set
    }
}