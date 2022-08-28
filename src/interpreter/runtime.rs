use crate::{runtime, environment_record::{EnvironmentRecord, Variable}, ast, lexical};

#[derive(Clone, Debug, PartialEq)]
pub struct JSBoolean(bool);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum JSNumber {
    NaN,
    Infinity(bool), // true = positive, false = negative
    Integer(i64),
}

impl JSNumber {
    
    #[inline]
    fn to_int32(&self) -> i32 {
        match self {
            &Self::Integer(i) => i as i32, // TODO
            _ => 0,
        }
    }

    const MAX_SAFE_INTEGER: i64 = 9007199254740991;
    const MIN_SAFE_INTEGER: i64 = -9007199254740991;

    fn check_overflow(&mut self) -> &mut Self {
        match self {
            Self::Integer(i) => {
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
            Self::Integer(i) => {
                if *i < Self::MIN_SAFE_INTEGER  {
                    *i = Self::MIN_SAFE_INTEGER;
                }
            }
            _ => {}
        }
        self
    }
    
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        match self {
            &Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    #[inline]
    fn op_add(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Self::Infinity(x) => match other {
                Self::Infinity(y) => {
                    if *x != *y {
                        *self = Self::NaN;
                    }
                }
                _ => {}, // ignore other cases
            }
            Self::Integer(x) => match other {
                Self::Infinity(_) => { *self = *other },
                Self::Integer(y) => { *x += *y; self.check_overflow(); },
                NaN => { *self = Self::NaN },
            },
        };

        self
    }

    #[inline]
    fn op_sub(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Self::Infinity(x) => match other {
                Self::Infinity(y) => {
                    if *x == *y {
                        *self = Self::NaN
                    }
                }
                _ => {}, // ignore other cases
            }
            Self::Integer(x) => match other {
                Self::Infinity(y) => { *self = Self::Infinity(!*y) },
                Self::Integer(y) => { *x -= *y; self.check_underflow(); },
                NaN => { *self = Self::NaN },
            },
        };

        self
    }

    #[inline]
    fn op_mul(&mut self, other: &Self) -> &mut Self {
        match self {
            Self::NaN => {},
            Self::Infinity(x) => match other {
                Self::Infinity(y) => { *x = *x == *y },
                Self::Integer(0) => { *self = Self::NaN },
                Self::Integer(y) => { *x = *x == (*y>=0) },
                Self::NaN => { *self = Self::NaN },
            }
            Self::Integer(0) => match other {
                Self::Infinity(_) => { *self = Self::NaN },
                Self::Integer(_) => {},
                Self::NaN => { *self = Self::NaN }, 
            },
            Self::Integer(x) => match other {
                Self::Infinity(y) => { *self = Self::Infinity((*x>=0) == *y) },
                Self::Integer(y) => { *x = *x * *y },
                Self::NaN => { *self = Self::NaN },
                _ => panic!("should not reach here"), 
            },
        };

        self
    }

    #[inline]
    fn op_div(&mut self, other: &Self) -> &mut Self {
        match self {
            Self::NaN => {},
            Self::Infinity(x) => match other {
                Self::Infinity(y) => { *self = Self::NaN },
                Self::Integer(y) => { *x = *x == (*y>=0) },
                Self::NaN => { *self = Self::NaN },
            }
            Self::Integer(0) => match other {
                Self::Integer(0) => { *self = Self::NaN },
                Self::NaN => { *self = Self::NaN },
                _ => {},
            },
            Self::Integer(x) => match other {
                Self::Integer(0) => { *self = Self::Infinity(*x>=0) },
                Self::Infinity(y) => { *x = 0 },
                Self::Integer(y) => { *x = *x / *y },
                NaN => { *self = Self::NaN },
            },
        };

        self
    }

    #[inline]
    fn op_mod(&mut self, other: &Self) -> &mut Self {
        match self {
            Self::NaN => {},
            Self::Infinity(x) => { *self = Self::NaN },
            Self::Integer(x) => match other {
                Self::Infinity(y) => {},
                Self::Integer(0) => { *self = Self::NaN },
                Self::Integer(y) => { *x = *x % *y },
                Self::NaN => { *self = Self::NaN },
            },
        };

        self
    }

    #[inline]
    fn op_pow(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    } // TODO XXX

    #[inline]
    fn op_bitand(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() & other.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 },
            _ => { *self = Self::Integer(v as i64) },
        };
        
        self
    }

    #[inline]
    fn op_bitor(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() | other.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 },
            _ => { *self = Self::Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_bitxor(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 },
            _ => { *self = Self::Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_bitnot(&mut self) -> &mut Self {
        let v = !self.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 },
            _ => { *self = Self::Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_lshift(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() << other.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 }, 
            _ => { *self = Self::Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_rshift(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() >> other.to_int32();
        match self {
            Self::Integer(x) => { *x = v as i64 },
            _ => { *self = Self::Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_urshift(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdfasdf")
    }


    // Copilot wrote, need to check
    #[inline]
    fn op_lt(&self, other: &Self) -> bool {
        match self {
            Self::NaN => match other {
                Self::NaN => false,
                _ => false,
            },
            Self::Infinity(x) => match other {
                Self::Infinity(y) => *x < *y,
                _ => false,
            },
            Self::Integer(x) => match other {
                Self::Integer(y) => *x < *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_lte(&self, other: &Self) -> bool {
        match self {
            Self::NaN => match other {
                Self::NaN => false,
                _ => false,
            },
            Self::Infinity(x) => match other {
                Self::Infinity(y) => *x <= *y,
                _ => false,
            },
            Self::Integer(x) => match other {
                Self::Integer(y) => *x <= *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_gt(&self, other: &Self) -> bool {
        match self {
            Self::NaN => match other {
                Self::NaN => false,
                _ => false,
            },
            Self::Infinity(x) => match other {
                Self::Infinity(y) => *x > *y,
                _ => false,
            },
            Self::Integer(x) => match other {
                Self::Integer(y) => *x > *y,
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn op_gte(&self, other: &Self) -> bool {
        match self {
            Self::NaN => match other {
                Self::NaN => false,
                _ => false,
            },
            Self::Infinity(x) => match other {
                Self::Infinity(y) => *x >= *y,
                _ => false,
            },
            Self::Integer(x) => match other {
                Self::Integer(y) => *x >= *y,
                _ => false,
            },
        }
    }

    #[inline]
    fn op_neg(&mut self) -> &mut Self {
        match self {
            Self::NaN => {},
            Self::Infinity(x) => *x = !*x,
            Self::Integer(x) => *x = -*x,
        };

        self
    }

    #[inline]
    fn op_inc(&mut self) -> &mut Self {
        match self {
            Self::NaN => self,
            Self::Infinity(x) => self,
            Self::Integer(x) => { *x += 1; self },
        }.check_overflow()
    }

    #[inline]
    fn op_dec(&mut self) -> &mut Self {
        match self {
            Self::NaN => self,
            Self::Infinity(x) => self,
            Self::Integer(x) => { *x -= 1; self },
        }.check_underflow()
    }
}

#[cfg(test)]
mod test_number {
    use crate::ast;
    
    #[test]
    fn simple_test() {
        let test_arithmetic = |mut ix: i64, iy: i64, op: ast::binary_expression::Operator| {
            let mut jsx = crate::interpreter::runtime::JSNumber::Integer(ix);
            let jsy = crate::interpreter::runtime::JSNumber::Integer(iy);
            assert_eq!(jsx.to_i64().unwrap(), ix);
            assert_eq!(jsy.to_i64().unwrap(), iy);
            match op {
                ast::binary_expression::Operator::Add => {
                    ix += iy;
                    jsx.op_add(&jsy)
                }
                ast::binary_expression::Operator::Sub => {
                    ix -= iy;
                    jsx.op_sub(&jsy)
                }
                ast::binary_expression::Operator::Mul => {
                    ix *= iy;
                    jsx.op_mul(&jsy)
                }
                ast::binary_expression::Operator::Div => {
                    ix /= iy;
                    jsx.op_div(&jsy)
                }
                ast::binary_expression::Operator::Mod => {
                    ix %= iy;
                    jsx.op_mod(&jsy)
                }
                /*
                ast::BinaryOp::BitAnd => jsx.op_bitand(&jsy),
                ast::BinaryOp::BitOr => jsx.op_bitor(&jsy),
                ast::BinaryOp::BitXor => jsx.op_bitxor(&jsy),
                ast::BinaryOp::LShift => jsx.op_lshift(&jsy),
                ast::BinaryOp::RShift => jsx.op_rshift(&jsy),
                ast::BinaryOp::URShift => jsx.op_urshift(&jsy),
                */
                _ => unimplemented!("Invalid op"),
            };

            assert_eq!(jsx.to_i64().unwrap(), ix);
        };

        // copilot wrote, add more edge cases later
        test_arithmetic(1, 2, ast::binary_expression::Operator::Add);
        test_arithmetic(1, 2, ast::binary_expression::Operator::Sub);
        test_arithmetic(1, 2, ast::binary_expression::Operator::Mul);
        test_arithmetic(1, 2, ast::binary_expression::Operator::Div);
        test_arithmetic(1, 2, ast::binary_expression::Operator::Mod);

        test_arithmetic(1, -2, ast::binary_expression::Operator::Add);
        test_arithmetic(1, -2, ast::binary_expression::Operator::Sub);
        test_arithmetic(1, -2, ast::binary_expression::Operator::Mul);
        test_arithmetic(1, -2, ast::binary_expression::Operator::Div);
        test_arithmetic(1, -2, ast::binary_expression::Operator::Mod);

        test_arithmetic(-1, 2, ast::binary_expression::Operator::Add);
        test_arithmetic(-1, 2, ast::binary_expression::Operator::Sub);
        test_arithmetic(-1, 2, ast::binary_expression::Operator::Mul);
        test_arithmetic(-1, 2, ast::binary_expression::Operator::Div);
        test_arithmetic(-1, 2, ast::binary_expression::Operator::Mod);

        test_arithmetic(-1, -2, ast::binary_expression::Operator::Add);
        test_arithmetic(-1, -2, ast::binary_expression::Operator::Sub);
        test_arithmetic(-1, -2, ast::binary_expression::Operator::Mul);
        test_arithmetic(-1, -2, ast::binary_expression::Operator::Div);
        test_arithmetic(-1, -2, ast::binary_expression::Operator::Mod);

        test_arithmetic(0, 2, ast::binary_expression::Operator::Add);
        test_arithmetic(0, 2, ast::binary_expression::Operator::Sub);
        test_arithmetic(0, 2, ast::binary_expression::Operator::Mul);
        test_arithmetic(0, 2, ast::binary_expression::Operator::Div);
        test_arithmetic(0, 2, ast::binary_expression::Operator::Mod);
    } 
}

#[derive(Clone, Debug, PartialEq)]
pub struct JSBigint {
    // TODO
}

impl JSBigint {
    fn wrap(self) -> JSValue {
        JSValue::Bigint(self)
    }

    fn op_add(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_sub(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_mul(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_div(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_mod(&mut self, other: &Self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_neg(&mut self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_inc(&mut self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_dec(&mut self) -> &mut Self {
        unimplemented!("asdf")
    }

    fn op_lt(&self, other: &Self) -> bool {
        unimplemented!("asdf")
    }

    fn op_lte(&self, other: &Self) -> bool {
        unimplemented!("asdf")
    }

    fn op_gt(&self, other: &Self) -> bool {
        unimplemented!("asdf")
    }

    fn op_gte(&self, other: &Self) -> bool {
        unimplemented!("asdf")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct JSString(String);

impl JSString {
    fn wrap(self) -> JSValue {
        JSValue::String(self)
    }

    fn op_concat(&mut self, other: &Self) -> &mut Self {
        self.0.push_str(&other.0);
        self
    }
}

#[derive(Debug, Clone)]
pub enum Prototype {
    Function(EnvironmentRecord<JSValue>, ast::FunctionExpression), // function object
    Array(Vec<JSValue>), // object with array storage
    // TypedArray(Vec<u8>),
    Error(ErrorType, String), // error object
    // Primitive(), // primitive value wrapper
    // ForeignFunction(Box<dyn Fn(Vec<JSValue>) -> Result<JSValue, String>>), // foreign function object
    // ForeignObject(Box<fdsafdsa>) // foreign object
    // Struct(), // known type struct object, inferred or from type annotation
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
use std::collections::BTreeMap;

#[derive(PartialEq, Debug, Clone)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl ToString for JSObject {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("{");
        for (k, v) in &self.properties {
            s.push_str(&format!("{}:{},", k, v.to_string()));
        }
        s.push_str("}");
        s
    }
}

#[cfg(test)]
mod reference_test {
    use crate::interpreter::runtime::JSObject;
    use crate::interpreter::runtime::JSValue;

    #[test]
    fn simple_object_test() {
        fn get(obj: &JSValue, name: &str) -> JSValue {
            obj.get_property(JSValue::string(name.to_string())).unwrap()
        }

        fn set(obj: &mut JSValue, name: &str, value: JSValue) {
            obj.set_property(&JSValue::string(name.to_string()), value)
        }

        let mut obj = JSValue::object(vec![
            (&"a".to_string(), JSValue::number(1)),
            (&"b".to_string(), JSValue::number(2)),
            (&"c".to_string(), JSValue::number(3)),
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
pub enum JSValue {
    Undefined,
    Null,
    Boolean(JSBoolean),
    Number(JSNumber),
    Bigint(JSBigint),
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

    pub fn bigint(n: i64) -> Self {
        JSValue::Bigint(JSBigint { })
    }

    pub fn as_bigint(&self) -> Option<&JSBigint> {
        if let JSValue::Bigint(val) = self {
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

    pub fn object(props: Vec<(&String, JSValue)>) -> Self {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert(k.clone(), v);
        }
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: None,
            properties: properties,
        })))
    }

    pub fn function(env: EnvironmentRecord<JSValue>, code: &ast::FunctionExpression) -> Self {
        JSValue::Object(Rc::new(RefCell::new(JSObject{
            prototype: Some(Prototype::Function(env, code.clone()),),
            properties: BTreeMap::new(),
        })))
    }


    
    pub fn get_property(&self, key: JSValue) -> Option<JSValue> {
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
            JSValue::Number(_) = true,
            JSValue::Bigint(n) => true, // TODO
            JSValue::String(s) => s.0.len() > 0,
            JSValue::Object(obj) => true,
        }
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
        
        for capture in captures {
            let var = self.environment_record.access_variable_by_name(&capture).unwrap();
            captured_vars.push((capture, var));
        }

        Self::V::function(code, captured_vars)
    }

    fn block_scope(&mut self, hoisted_funcs: Vec<(String, Self::V)>, body: impl Fn(&mut Self)) {
        let current_env = self.environment_record.enter(vec![]);
        for (name, func) in hoisted_funcs {
            current_env.initialize_immutable_binding(&name, &func)
        }  

        self.with_env(current_env, body);
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

        Err("cannot call non function".to_string()),
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

    fn resolve_binding(&mut self, name: &String) -> Result<Self::V, String> {
        self.environment_record.get_binding_value(name)
    }

    fn set_binding(&mut self, name: &String, v: &Self::V) -> Result<(), String> {
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