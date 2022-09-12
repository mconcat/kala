use kala_ast::ast;
use kala_context::{environment_record::{EnvironmentRecord}};
use kala_lexical::eval::eval_function;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign, Not, Neg, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};
use std::cmp::{PartialEq, PartialOrd, Eq, Ord};




use std::collections::BTreeMap;



#[cfg(test)]
mod reference_test {
    use crate::runtime::JSObject;
    use crate::runtime::JSValue;

    #[test]
    fn simple_object_test() {
        fn get(obj: &JSValue, name: &str) -> JSValue {
            obj.get_property(JSValue::string(name.to_string())).unwrap().clone()
        }

        fn set(obj: &mut JSValue, name: &str, value: JSValue) {
            obj.set_property(&JSValue::string(name.to_string()), value)
        }

        let mut obj = JSValue::object(vec![
            ("a".to_string(), JSValue::number(1)),
            ("b".to_string(), JSValue::number(2)),
            ("c".to_string(), JSValue::number(3)),
        ]);

        assert_eq!(get(&obj, "a"), JSValue::number(1));

        assert_eq!(get(&obj, "b"), JSValue::number(2));
        
        assert_eq!(get(&obj, "c"), JSValue::number(3));

        set(&mut obj, "a", JSValue::number(4));
        assert_eq!(get(&obj, "a"), JSValue::number(4));

        set(&mut obj, "b", JSObject::new(vec![(JSValue::string("x".to_string()), JSValue::number(5))]));
        assert_eq!(get(&get(&obj, "b"), "x"), JSValue::number(5));
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, PartialEq)]
pub enum JSValue<N: JSNumber> {
    Undefined,
    Null,
    Boolean(JSBoolean),
    Number(N),
    String(JSString),
    Object(Rc<RefCell<JSObject>>),
}

impl JSValue {
    pub fn undefined() -> Self {
        JSValue::Undefined
    }

    pub fn null() -> Self {
        JSValue::Null
    }

    pub fn boolean(b: bool) -> Self {
        JSValue::Boolean(JSBoolean(b))
    }

    pub fn as_boolean(&self) -> Option<&JSBoolean> {
        if let JSValue::Boolean(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn number(n: i64) -> Self {
        JSValue::Number(JSNumber::Integer(n))
    }

    pub fn as_number(&self) -> Option<&JSNumber> {
        if let JSValue::Number(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn string(s: String) -> Self {
        JSValue::String(JSString(s))
    } 

    pub fn as_string(&self) -> Option<&JSString> {
        if let JSValue::String(val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn array(elems: Vec<JSValue>) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Array(elems)),
            properties: BTreeMap::new(),
        })))
    }

    pub fn object(props: Vec<(String, JSValue)>) -> Self {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert(k.clone(), v);
        }
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: None,
            properties: properties,
        })))
    }

    pub fn function(env: EnvironmentRecord<JSValue>, code: ast::FunctionExpression) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Function(PrototypeFunction::new(env, code.clone()))),
            properties: BTreeMap::new(),
        })))
    }


    
    pub fn get_property(&self, key: JSValue) -> Option<&JSValue> {
        match self {
            JSValue::Object(obj) => {
                let mut obj = obj.borrow_mut();
                match key {
                    JSValue::Undefined => return obj.properties.get("undefined"),
                    JSValue::String(s) => return obj.properties.get(&s.0),
                    JSValue::Boolean(JSBoolean(b)) => return obj.properties.get(&b.to_string()),
                    JSValue::Number(n) => {
                        if let Some(Prototype::Array(elems)) = obj.prototype {
                            if let Some(index) = n.to_i64() {
                                if 0 <= index && index < elems.len() as i64 {
                                    return Some(&elems[index as usize])
                                } 
                            }
                        }
                        return obj.properties.get(&n.to_string())
                    }
                    _ => unimplemented!("object key")
                }
            }
            _ => None,
        }
    }

    pub fn set_property(&mut self, key: &JSValue, value: JSValue) {
        match self {
            JSValue::Object(obj) => {
                match key {
                    JSValue::String(s) => {
                        let mut obj = obj.borrow_mut();
                        obj.properties.insert(s.0, value);
                    }
                    _ => {
                        panic!("set_property: key must be a string");
                    }
                }
            }
            _ => (),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            JSValue::Undefined => false,
            JSValue::Null => false,
            JSValue::Boolean(b) => b.0,
            JSValue::Number(JSNumber::NaN) => false,
            JSValue::Number(JSNumber::Integer(0)) => false,
            JSValue::Number(_) => true,
            JSValue::String(s) => s.0.len() > 0,
            JSValue::Object(obj) => true,
        }
    }
}

impl Default for JSValue {
    fn default() -> Self {
        JSValue::Undefined
    }
}

impl runtime::JSValue for JSValue {
    fn error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn range_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn reference_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn type_error(s: String) -> Self {
        unimplemented!("asdf")
    }

    fn is_reference(&self) -> bool {
        match self {
            JSValue::Object(_) => true,
            _ => false,
        }
    }
}

enum CompletionRecord {
    Normal,
    Return(Option<JSValue>),
    Throw(JSValue),
    Break,
    Continue,
}

pub struct JSContext {
    environment_record: EnvironmentRecord<JSValue>,
    completion: CompletionRecord,
}

impl JSContext {
    fn with_env(&mut self, env: EnvironmentRecord<JSValue>, f: impl Fn(&mut Self)) {
        let current_context = JSContext {
            environment_record: env,
            completion: CompletionRecord::Normal,
        };

        f(&mut current_context);

        self.completion = current_context.completion;

        // env drops here
    }
}

impl runtime::JSContext for JSContext {
    type V = JSValue;

    fn undefined() -> Self::V {
        Self::V::undefined()
    }

    fn null() -> Self::V {
        Self::V::null()
    }

    fn boolean(b: bool) -> Self::V {
        Self::V::boolean(b)
    }

    fn coerce_boolean(&mut self, v: &Self::V) -> Self::V {
        match v {
            Self::V::Boolean(b) => Self::V::boolean(b.0),
            _ => unimplemented!("coerce boolean"),
        }
    }

    fn number(n: i64) -> Self::V {
        Self::V::number(n)
    }

    fn coerce_number(&mut self, v: &Self::V) -> Self::V {
        match v {
            Self::V::Number(n) => v.clone(),
            _ => unimplemented!("coerce number"),
        }
    }

    fn bigint(n: i64) -> Self::V {
        Self::V::bigint(n)
    }

    fn string(s: String) -> Self::V {
        Self::V::string(s)
    }

    fn coerce_string(&mut self, v: &Self::V) -> Self::V {
        match v {
            Self::V::String(s) => s.clone(),
            _ => unimplemented!("coerce string"),
        }
    }

    fn array(&mut self, v: Vec<Self::V>) -> Self::V {
        Self::V::array(v)
    }

    fn object(&mut self, props: Vec<(String, Self::V)>) -> Self::V {
        Self::V::object(props)
    }

    fn function(&mut self, captures: Vec<String>, code: ast::FunctionExpression) -> Self::V {
        let mut captured_vars = Vec::with_capacity(captures.len());
        
        let scope = self.environment_record.function_scope(captures);


        Self::V::function(captured_vars, code)
    }

    fn block_scope(&mut self, hoisted_funcs: Vec<(String, Self::V)>, body: impl Fn(&mut Self)) {
        self.environment_record.enter();
        for (name, func) in hoisted_funcs {
            current_env.initialize_immutable_binding(&name, &func)
        }  
        self.environment_record.exit()
    }

    fn call(&mut self, func: &Self::V, args: Vec<Self::V>) -> Result<(), String> {
        if let JSValue::Object(ptr) = func {
            let mut obj = ptr.borrow_mut();
            if let Some(Prototype::Function(env, f)) = obj.prototype {
                let params = args.iter().enumerate().map(|(i, arg)| (f.parameters[i], arg));
                let local_env = env.enter(params);
                self.with_env(local_env, |ctx| { lexical::eval_function(ctx, &f); });
                return Ok(())
            } 
        }

        Err("cannot call non function".to_string())
    }

    #[inline]
    fn control_loop(&mut self, test: impl Fn(&mut Self) -> Self::V, body: impl Fn(&mut Self)) {
        while test(self).is_truthy() {
            body(self);
        }
    }

    #[inline]
    fn control_branch(&mut self, test: impl Fn(&mut Self) -> Self::V, consequent: impl Fn(&mut Self), alternate: impl Fn(&mut Self)) {
        if test(self).is_truthy() {
            consequent(self);
        } else {
            alternate(self);
        }
    }

    #[inline]
    fn control_branch_value(&mut self, test: impl Fn(&mut Self) -> Self::V, consequent: impl Fn(&mut Self) -> Self::V, alternate: impl Fn(&mut Self) -> Self::V) -> Self::V {
        if test(self).is_truthy() {
            consequent(self)
        } else {
            alternate(self)
        }
    }

     #[inline]
    fn control_switch(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn control_coalesce(&mut self, left: impl Fn(&mut Self) -> Self::V, right: impl Fn(&mut Self) -> Self::V) -> Self::V {
        let result = left(self);
        match result {
            JSValue::Undefined => right(self),
            JSValue::Null => right(self),
            _ => result,
        }
    }

    #[inline]
    fn complete_break(&mut self) {
        self.completion = CompletionRecord::Break;
    }

    fn is_break(&self) -> bool {
        match self.completion {
            CompletionRecord::Break => true,
            _ => false,
        }
    }

    #[inline]
    fn complete_continue(&mut self) {
        self.completion = CompletionRecord::Continue;
    }

    fn is_continue(&self) -> bool {
        match self.completion {
            CompletionRecord::Continue => true,
            _ => false,
        }
    }

    fn complete_return(&mut self) {
        self.completion = CompletionRecord::Return(None);
    }

    #[inline]
    fn complete_return_value(&mut self, value: Self::V) {
        self.completion = CompletionRecord::Return(Some(value));
    }

    fn is_return(&self) -> bool {
        match self.completion {
            CompletionRecord::Return(_) => true,
            _ => false,
        }
    }

    fn consume_return(&mut self) -> Option<Self::V> {
        let result = match self.completion {
            CompletionRecord::Return(Some(value)) => Some(value),
            _ => None,
        };
        self.completion = CompletionRecord::Normal;
        result
    }

    fn complete_throw(&mut self, val: Self::V) {
        self.completion = CompletionRecord::Throw(val);
    }

    fn is_throw(&self) -> bool {
        match self.completion {
            CompletionRecord::Throw(_) => true,
            _ => false,
        }
    }

    fn consume_throw(&mut self) -> Self::V {
        let result = match self.completion {
            CompletionRecord::Throw(value) => value,
            _ => unreachable!(),
        };
        self.completion = CompletionRecord::Normal;
        result
    }

    fn initialize_mutable_binding(&mut self, name: &String, v: Option<Self::V>) -> Result<(), String> {
        self.environment_record.initialize_mutable_binding(name, v);
        Ok(())
    }

    fn initialize_immutable_binding(&mut self, name: &String, v: &Self::V) -> Result<(), String> {
        self.environment_record.initialize_immutable_binding(name, v)
    }

    fn resolve_binding(&mut self, name: &String) -> Self::V {
        self.environment_record.resolve_binding(name)
    }

    fn set_binding(&mut self, name: &String, v: Self::V) -> Result<(), String> {
        self.environment_record.set_mutable_binding(name, v)
    }

    // operations

    fn op_add(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V {
        match (left, other) {
            (JSValue::String(l), JSValue::String(r)) => l.op_concat(r),
            (JSValue::String(_), _) | (_, JSValue::String(_)) => unimplemented!("string coersion concat"),
            (l, r) => self.coerce_number(l).op_add(self.coerce_number(r)),
        }
    }

    fn op_sub(&mut self, left: &mut Self::V, other: &Self::V) -> &mut Self::V {
        self.coerce_number(left).as_number().and_then(|left| 
        self.coerce_number(other).as_number().and_then(|right|
        left.op_sub(right))).unwrap_or_else(JSValue::type_error)
    }
}