use core::panic;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;
use std::collections::HashSet;

#[path="./runtime.rs"]
mod runtime;

#[path = "../gen/nessie.ast.rs"]
pub mod ast;

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

    fn type_match<T>(&self, 
            if_null: T,
            if_undefined: T,
            if_boolean: &dyn Fn(bool) -> T,
            if_number: &dyn Fn(&Self::N) -> T,
            // if_bigint: dyn Fn(&Self) -> &'a T,
            if_string: &dyn Fn(&Self::S) -> T,
            if_object: &dyn Fn(&Self::R) -> T,
        ) -> T {
        match self {
            JSValue::Null => if_null,
            JSValue::Undefined => if_undefined,
            JSValue::Boolean(b) => if_boolean(*b),
            JSValue::Number(n) => if_number(n),
            // JSValue::Bigint(n, _) => if_bigint(n),
            JSValue::String(s) => if_string(s),
            JSValue::Reference(r) => if_object(r),
        }
    }

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
            JSValue::Reference(r) => Some(r.clone()),
            _ => None,
        }
    }

    fn to_boolean(&self) -> bool {
        self.is_truthy()
    }

    fn to_integer(&self) -> &Self::N {
        match self {
            JSValue::Undefined => &JSNumber::NaN,
            JSValue::Null => &JSNumber::Integer(0),
            JSValue::Boolean(b) => &JSNumber::Integer(if *b { 1 } else { 0 }),
            JSValue::Number(n) => self,
            JSValue::String(_) => panic!("Cannot convert to integer"),
            JSValue::Reference(_) => panic!("Cannot convert to integer"),
        }
    }

    fn to_string(&self) -> &Self::S {
        match self {
            JSValue::Undefined => &JSString::new("undefined".to_string()),
            JSValue::Null => &JSString::new_short("null"),
            JSValue::Boolean(b) => &JSString::new_short(if *b { "true" } else { "false" }),
            JSValue::Number(n) => &JSString::new(n.to_string()),
            JSValue::String(s) => s,
            JSValue::Reference(r) => panic!("Cannot convert to string"),
        }
    }

    fn to_object(&self) -> &Self::R {
        match self {
            JSValue::Reference(r) => r,
            _ => unimplemented!("Type conversion to object"),
        }
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
    fn ok(&mut self, value: ()) -> Result<&mut Self, String> {
        Ok(self)
    }

    #[inline]
    fn assign(&mut self, value: Self) -> Result<&mut Self, String> {
        *self = value;
        Ok(self)
    }

    #[inline]
    fn type_error(&mut self) -> Result<&mut Self, String> {
        Err("Type error".to_string())
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
}

impl runtime::JSNumeric for JSNumber {
    #[inline]
    fn add(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x == *y {
                        self.ok(())
                    } else {
                        self.assign(Self::NaN)
                    }
                }
                _ => self.ok(()) // ignore other cases
            }
            Integer(x) => match other {
                Infinity(_) => self.assign(*other),
                Integer(y) => self.ok(*x = *x + *y),
                NaN => self.assign(NaN),
            },
        }
    }

    #[inline]
    fn sub(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => {
                    if *x != *y {
                        self.ok(())
                    } else {
                        self.assign(NaN)
                    }
                }
                _ => self.ok(()), // ignore other cases
            }
            Integer(x) => match other {
                Infinity(y) => self.ok(*y = !*y),
                Integer(y) => self.ok(*x = *x - *y),
                NaN => self.assign(NaN),
            },
        }
    }

    #[inline]
    fn mul(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => self.ok(*x = *x == *y),
                Integer(0) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x == (*y>=0)),
            }
            Integer(0) => match other {
                Infinity(_) => self.assign(NaN),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
            Integer(x) => match other {
                Infinity(y) => self.assign(Infinity((*x>=0) == *y)),
                Integer(y) => self.ok(*x = *x * *y),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
        }
    }

    #[inline]
    fn div(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => match other {
                Infinity(y) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x == (*y>=0)),
                NaN => self.assign(NaN),
            }
            Integer(0) => match other {
                Integer(0) => self.assign(NaN),
                NaN => self.assign(NaN),
                _ => self.ok(()),
            },
            Integer(x) => match other {
                Integer(0) => self.assign(Infinity(*x>=0)),
                Infinity(y) => self.ok(*x = 0),
                Integer(y) => self.ok(*x = *x / *y),
                NaN => self.assign(NaN),
                _ => self.type_error(),
            },
        }
    }

    #[inline]
    fn modulo(&mut self, other: &Self) -> Result<&mut Self, String> {
        match self {
            NaN => self.ok(()),
            Infinity(x) => self.assign(NaN),
            Integer(x) => match other {
                Infinity(y) => self.ok(()),
                Integer(0) => self.assign(NaN),
                Integer(y) => self.ok(*x = *x % *y),
                NaN => self.assign(NaN),
                _ => self.type_error(), 
            },
        }
    }

    #[inline]
    fn pow(&mut self, other: &Self) -> Result<&mut Self, String> {
        unimplemented!("asdf")
    } // TODO XXX

    #[inline]
    fn bitand(&mut self, other: &Self) -> Result<&mut Self, String> {
        let v = self.to_int32() & other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        }  
    }

    #[inline]
    fn bitor(&mut self, other: &Self) -> Result<&mut Self, String> {
        let v = self.to_int32() | other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        } 
    }

    #[inline]
    fn bitxor(&mut self, other: &Self) -> Result<&mut Self, String> {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        }    
    }

    #[inline]
    fn bitnot(&mut self) -> Result<&mut Self, String> {
        let v = self.to_int32() ^ 0xFFFFFFFF;
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        }
    }

    #[inline]
    fn lshift(&mut self, other: &Self) -> Result<&mut Self, String> {
        let v = self.to_int32() << other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64), 
            _ => self.assign(Integer(v as i64)),
        } 
    }

    #[inline]
    fn rshift(&mut self, other: &Self) -> Result<&mut Self, String> {
        let v = self.to_int32() >> other.to_int32();
        match self {
            Integer(x) => self.ok(*x = v as i64),
            _ => self.assign(Integer(v as i64)),
        }   
    }

    #[inline]
    fn urshift(&mut self, other: &Self) -> Result<&mut Self, String> {
        unimplemented!("asdfasdf")
    }

    #[inline]
    fn equal(&self, other: &Self) -> bool {
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
    fn not_equal(&self, other: &Self) -> bool {
        !self.equal(other)
    }

    // Copilot wrote, need to check
    #[inline]
    fn lt(&self, other: &Self) -> bool {
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
    fn le(&self, other: &Self) -> bool {
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
    fn gt(&self, other: &Self) -> bool {
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
    fn ge(&self, other: &Self) -> bool {
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
}

impl runtime::JSNumber for JSNumber {

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
                *self = JSString::Normal(std::str::from_utf8(bytes.as_ref())?.to_string());
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

#[derive(PartialEq, Eq, Debug)]
// Jessie does not allow custom prototype definition,
// so we use a single enum for all predefined prototypes.
pub enum Prototype {
    Function(HashMap<String, JSValue>, ast::FunctionExpression), // function object
    Array(Vec<JSValue>), // object with array storage
    // TypedArray(Vec<u8>),
    Error(ErrorType, String), // error object
    // Primitive(),
}

#[derive(PartialEq, Eq, Debug)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl JSObject {
    #[inline]
    fn new() -> Self {
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
    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: &dyn Fn(&Self) -> Option<JSValue>, captures: Vec<&JSValue>) -> Self {
        JSObject {
            prototype: Some(Prototype::Function()),
            properties: BTreeMap::new(),
        }
    }
}

impl runtime::JSObject for JSObject {
    fn call(&self, ctx: &JSContext, parameters: Vec<JSValue>) -> Option<JSValue> {
        if let Prototype::Function(env, func) = &self.prototype {
            
        } else {
            panic!("Type error");
        }

        lexical::eval_statement(ctx, self.prototype)
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

    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: dyn Fn(&Self) -> &JSValue, captures: Vec<&JSValue>) -> Self {
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
        JSProperty::new(self, name.name)
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
}

// JSProperty is meant to be ephemeral
pub struct JSProperty {
    value: Weak<RefCell<JSObject>>,
    propname: String,
}

impl JSProperty {
    fn new(refer: JSReference, name: String) -> Self {
        JSProperty {
            value: refer.value.downgrade(),
            propname: name,
        }
    }   
}

impl runtime::JSProperty for JSProperty {
    type V = JSValue;

    fn get(&self) -> Self::V {
        match *self.value.value.borrow() {
            JSObject { prototype, properties } => {
                *properties.get(&self.propname).unwrap_or(&JSValue::Undefined) 
            }
            _ => unimplemented!("ToObject"), // TODO
        }
    }

    fn set(&self, value: Self::V) {
        match *self.value.value.borrow() {
            JSObject { prototype, properties } => {
                *properties.get_mut(&self.propname).unwrap() = value;
            }
            _ => unimplemented!("TypeError"), // TODO: return TypeError
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ScopedVariable {
    value: JSValue,
    kind: ast::DeclarationKind,
    depth: i16
}

impl ScopedVariable {
    #[inline]
    pub fn new(value: JSValue, kind: ast::DeclarationKind, depth: i16) -> Self {
        ScopedVariable {
            value,
            kind,
            depth
        }
    }
}

pub struct Scope {
    binding: HashMap<String, ScopedVariable>,
    recovery: Vec<BTreeMap<String, Option<ScopedVariable>>>,
}

impl Scope {
    pub fn new() -> Self {
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
    pub fn get(&self, name: String) -> Option<JSValue> {
        match self.binding.get(&name) {
            Some(var) => Some(var.value),
            None => None,
        }
    }
    
    pub fn declare(&mut self, name: String, kind: ast::DeclarationKind, value: Option<JSValue>) {
        if kind == ast::DeclarationKind::Const && value.is_none() {
            panic!("const variable must be initialized");
        }

        match self.binding.get_mut(&name) {
            Some(existing) => {
                if existing.depth < self.depth() {
                    // parent scope had declared the variable,
                    // shadow it by adding it to the recovery list
                    self.current_recovery().insert(name, Some(existing.clone()));
                } else if existing.kind == ast::DeclarationKind::Let {
                    // TODO: support overriding let variables
                    panic!("cannot override let variable")
                } else {
                    // cannot override const variables
                    panic!("cannot override const variable")
                }

                *existing = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
            },
            None => {
                self.current_recovery().insert(name, None);
                self.binding.insert(name, ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth()));
            }
        }
    }

    pub fn set(&mut self, name: String, value: &JSValue) -> Result<(), &str> {
        match self.binding.get_mut(&name) {
            Some(existing) => {
                if existing.depth != self.depth() {
                    // if self.declare() had been called, this should not happen
                    return Err("cannot set variable from a different scope")
                } else if existing.kind != ast::DeclarationKind::Let {
                    // cannot set to non-let variable
                    // TODO: throw error instead of panic
                    return Err("Cannot set to non-let variable");
                }
                *existing = ScopedVariable::new(*value, existing.kind, existing.depth);
                Ok(())
            }
            None => {
                panic!("Cannot set to non-declared variable");
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
    fn set_completion(&mut self, completion: runtime::Completion<JSValue>) {
        self.completion = Some(completion);
    }
}

impl runtime::JSContext for JSContext {
    type V = JSValue;

    fn block_scope(&self, body: &dyn Fn()) {
        self.scope.enter();
        body();
        self.scope.exit();
    }

    fn extract_free_variables(&self, vars: HashSet<String>) -> HashSet<String> {
        for var in vars.iter() {
            if self.scope.get(var).is_some() {
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
    fn control_loop(&mut self, test: &dyn Fn() -> Self::V, body: &dyn Fn()) {
        while test().is_truthy() {
            body();
        }
    }

    #[inline]
    fn control_branch(&mut self, test: &dyn Fn() -> Self::V, consequent: &dyn Fn(), alternate: &dyn Fn()) {
        if test().is_truthy() {
            consequent();
        } else {
            alternate();
        }
    }

    #[inline]
    fn control_branch_value(&mut self, test: &dyn Fn() -> Self::V, consequent: &dyn Fn() -> Self::V, alternate: &dyn Fn() -> Self::V) -> Self::V {
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
    fn control_coalesce(&mut self, left: &dyn Fn() -> Self::V, right: &dyn Fn() -> Self::V) -> Self::V {
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
    fn completion(&mut self) -> Option<runtime::Completion<Self::V>> {
        self.completion
    }

    #[inline]
    fn new_undefined() -> Self::V {
        JSValue::Undefined
    }

    #[inline]
    fn new_null() -> Self::V {
        JSValue::Null
    }

    #[inline]
    fn new_boolean(b: bool) -> Self::V {
        JSValue::Boolean(b)
    }

    #[inline]
    fn new_number(n: i64) -> Self::V {
        JSValue::Number(JSNumber::new(n))
    }

    #[inline]
    fn new_string(s: String) -> Self::V {
        JSValue::String(JSString::new(s))
    }

    #[inline]
    fn new_object() -> Self::V {
        JSValue::Reference(JSReference::new(JSObject::new()))
    }

    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: ast::FunctionExpression, captures: HashMap<String, Self::V>) -> Self::V {
        JSValue::Reference(JSReference::new_function(identifier, parameters, body, captures))
    }

    fn initialize_binding(&mut self, kind: ast::DeclarationKind, name: String, value: Option<Self::V>) {
        self.scope.declare(name, kind, value);
    }

    fn resolve_binding(&self, name: String) -> Option<Self::V> {
       self.scope.get(name) 
    }

    fn set_binding(&mut self, name: String, v: Self::V) -> Result<(), &str>{
        self.scope.set(name, &v)        
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Scope;
    use crate::interpreter::JSValue;
    use crate::interpreter::ast::DeclarationKind;
    
    #[test]
    fn scope_test_simple() {
        let scope = Scope::new();
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

        scope.declare("a".to_string(), DeclarationKind::Let, Some(JSValue::new_number(1)));
        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(1)));

        scope.declare("b".to_string(), DeclarationKind::Const, Some(JSValue::new_number(2)));
        assert_eq!(scope.get("b".to_string()), Some(JSValue::new_number(2)));
        
        scope.set("a".to_string(), &JSValue::new_number(3));
        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(3)));

        assert!(matches!(scope.set("b".to_string(), &JSValue::new_number(4)), Err(_)));

        scope.declare("c".to_string(), DeclarationKind::Let, None);
        assert_eq!(scope.get("c".to_string()), Some(JSValue::new_undefined()));

        scope.set("c".to_string(), &JSValue::new_number(4));
        assert_eq!(scope.get("c".to_string()), Some(JSValue::new_number(4)));

        scope.enter();

        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(1)));
        scope.declare("a".to_string(), DeclarationKind::Const, Some(JSValue::new_number(11)));
        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(11)));

        assert_eq!(scope.get("b".to_string()), Some(JSValue::new_number(2)));
        scope.declare("b".to_string(), DeclarationKind::Let, Some(JSValue::new_number(12)));
        assert_eq!(scope.get("b".to_string()), Some(JSValue::new_number(12)));

        assert_eq!(scope.get("x".to_string()), None);
        scope.declare("x".to_string(), DeclarationKind::Let, Some(JSValue::new_number(13)));
        assert_eq!(scope.get("x".to_string()), Some(JSValue::new_number(13)));

        assert_eq!(scope.get("c".to_string()), Some(JSValue::new_number(4)));
        scope.set("c".to_string(), &JSValue::new_number(14));
        assert_eq!(scope.get("c".to_string()), Some(JSValue::new_number(14)));

        scope.exit();

        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(1)));
        assert_eq!(scope.get("b".to_string()), Some(JSValue::new_number(2)));
        assert_eq!(scope.get("c".to_string()), Some(JSValue::new_number(14)));
        assert_eq!(scope.get("x".to_string()), None);

        scope.set("a".to_string(), &JSValue::new_number(5));
        assert_eq!(scope.get("a".to_string()), Some(JSValue::new_number(5)));

        assert!(matches!(scope.set("b".to_string(), &JSValue::new_number(6)), Err(_)));
    }
}