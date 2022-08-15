use core::panic;
use std::collections::BTreeMap;
use std::collections::HashMap;
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

#[derive(PartialEq, Eq, Debug, Clone)]
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
        JSValue::String(JSString::new(&value.to_string()))
    }

    pub fn new_object(obj: JSObject) -> JSValue {
        JSValue::Reference(JSReference::new(obj))
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
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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

    #[inline]
    fn to_int64(&self) -> Option<i64> {
        match self {
            &Integer(i) => Some(i),
            _ => None,
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
                Infinity(y) => { *self = Infinity(!*y) },
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
                Integer(y) => { *x = *x * *y },
                NaN => { *self = NaN },
                _ => panic!("should not reach here"), 
            },
        };

        self
    }

    #[inline]
    fn op_div(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => match other {
                Infinity(y) => { *self = NaN },
                Integer(y) => { *x = *x == (*y>=0) },
                NaN => { *self = NaN },
            }
            Integer(0) => match other {
                Integer(0) => { *self = NaN },
                NaN => { *self = NaN },
                _ => {},
            },
            Integer(x) => match other {
                Integer(0) => { *self = Infinity(*x>=0) },
                Infinity(y) => { *x = 0 },
                Integer(y) => { *x = *x / *y },
                NaN => { *self = NaN },
            },
        };

        self
    }

    #[inline]
    fn op_modulo(&mut self, other: &Self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => { *self = NaN },
            Integer(x) => match other {
                Infinity(y) => {},
                Integer(0) => { *self = NaN },
                Integer(y) => { *x = *x % *y },
                NaN => { *self = NaN },
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
            Integer(x) => { *x = v as i64 },
            _ => { *self = Integer(v as i64) },
        };
        
        self
    }

    #[inline]
    fn op_bitor(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() | other.to_int32();
        match self {
            Integer(x) => { *x = v as i64 },
            _ => { *self = Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_bitxor(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            Integer(x) => { *x = v as i64 },
            _ => { *self = Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_bitnot(&mut self) -> &mut Self {
        let v = !self.to_int32();
        match self {
            Integer(x) => { *x = v as i64 },
            _ => { *self = Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_lshift(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() << other.to_int32();
        match self {
            Integer(x) => { *x = v as i64 }, 
            _ => { *self = Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_rshift(&mut self, other: &Self) -> &mut Self {
        let v = self.to_int32() >> other.to_int32();
        match self {
            Integer(x) => { *x = v as i64 },
            _ => { *self = Integer(v as i64) },
        };

        self
    }

    #[inline]
    fn op_urshift(&mut self, other: &Self) -> &mut Self {
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
    fn op_neg(&mut self) -> &mut Self {
        match self {
            NaN => {},
            Infinity(x) => *x = !*x,
            Integer(x) => *x = -*x,
        };

        self
    }

    #[inline]
    fn op_inc(&mut self) -> &mut Self {
        match self {
            NaN => self,
            Infinity(x) => self,
            Integer(x) => { *x += 1; self },
        }.check_overflow()
    }

    #[inline]
    fn op_dec(&mut self) -> &mut Self {
        match self {
            NaN => self,
            Infinity(x) => self,
            Integer(x) => { *x -= 1; self },
        }.check_underflow()
    }
}

#[cfg(test)]
mod test_number {
    use crate::runtime::JSNumeric;
    use crate::interpreter::runtime::JSNumber;
    use crate::ast;
    
    #[test]
    fn simple_test() {
        let test_arithmetic = |mut ix: i64, iy: i64, op: ast::binary_expression::Operator| {
            let mut jsx = JSNumber::new(ix);
            let jsy = JSNumber::new(iy);
            assert_eq!(jsx.to_int64().unwrap(), ix);
            assert_eq!(jsy.to_int64().unwrap(), iy);
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
                    jsx.op_modulo(&jsy)
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

            assert_eq!(jsx.to_int64().unwrap(), ix);
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum JSString {
    Short(u64), // short ascii String, maximum length is 8 bytes
    Normal(String), // heap allocated arbitrary length UTF-8 String
}

impl JSString {
    #[inline]
    fn new(s: &String) -> Self {
        JSString::Normal(s.clone())
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

    #[inline]
    fn is_empty(&self) -> bool {
        match self {
            JSString::Short(x) => *x == 0,
            JSString::Normal(x) => x.is_empty(),
        }
    }

    #[inline]
    fn to_string(&self) -> String {
        match self {
           JSString::Short(x) => {
                let bytes: [u8; 8] = x.to_be_bytes();
                std::str::from_utf8(bytes.as_ref()).unwrap().to_string()
            }
            JSString::Normal(x) => x.clone()
        }
    }

}

impl runtime::JSString for JSString {
    #[inline]
    fn concat(&mut self, other: &Self) -> &mut Self {
        if let JSString::Normal(self_) = self.to_normal() {
            for c in other.to_string().chars() {
                self_.push(c);
            }
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
    // Primitive(), // primitive value wrapper
    // ForeignFunction(Box<dyn Fn(Vec<JSValue>) -> Result<JSValue, String>>), // foreign function object
    // State(String), // chain state object
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

impl Eq for Prototype {

}

#[derive(PartialEq, Eq, Debug)]
pub struct JSObject {
    prototype: Option<Prototype>, // none if simple Object
    properties: BTreeMap<String, JSValue>,
}

impl JSObject {
    #[inline]
    fn new(props: Vec<(runtime::PropName, JSValue)>) -> Self {
        let mut properties = BTreeMap::new();
        for (k, v) in props {
            properties.insert(k.to_string(), v);
        }
        JSObject {
            prototype: None,
            properties,
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
    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: &ast::FunctionExpression, captures: Vec<&ScopedVariable>) -> Self {
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

#[cfg(test)]
mod reference_test {
    use crate::interpreter::runtime::JSObject;
    use crate::interpreter::runtime::JSValue;
    use crate::runtime::JSProperty;
    use crate::runtime::PropName;
    use crate::runtime::JSReference;

    #[test]
    fn simple_object_test() {
        let mut obj = crate::interpreter::runtime::JSReference::new(JSObject::new(vec![
            (PropName::String("a".to_string()), JSValue::new_number(1)),
            (PropName::String("b".to_string()), JSValue::new_number(2)),
            (PropName::String("c".to_string()), JSValue::new_number(3)),
        ]));

        let mut prop_a = obj.property(PropName::String("a".to_string()));
        assert_eq!(prop_a.get(), JSValue::new_number(1));

        let mut prop_b = obj.property(PropName::String("b".to_string()));
        assert_eq!(prop_b.get(), JSValue::new_number(2));
        
        let mut prop_c = obj.property(PropName::String("c".to_string()));
        assert_eq!(prop_c.get(), JSValue::new_number(3));

        prop_a.set(JSValue::new_number(4));
        assert_eq!(prop_a.get(), JSValue::new_number(4));

        prop_b.set(JSValue::new_object(JSObject::new(vec![
            (PropName::String("x".to_string()), JSValue::new_number(5)),
        ])));

        assert_eq!(prop_b.get(), JSValue::new_object(JSObject::new(vec![
            (PropName::String("x".to_string()), JSValue::new_number(5)),
        ])));
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
// Reference is a pointer to a value in the heap.
// TODO: whenever the reference does not escape the current scope,
// it should be a pointer to the stack(without Rc<>).
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

    fn new_function(identifier: Option<String>, parameters: Vec<String>, body: &ast::FunctionExpression, captures: Vec<&ScopedVariable>) -> Self {
        JSReference {
            value: Rc::new(RefCell::new(JSObject::new_function(identifier, parameters, body, captures)))
        }
    }
}

impl runtime::JSReference for JSReference {
    type V = JSValue;
    type P = JSProperty;
//    type Iter = JSObjectIterator;

    fn property(&self, name: runtime::PropName) -> Self::P {
        JSProperty::new(self, name.to_string())
    }

    fn call(&self, _args: &[Self::V]) -> Self::V {
        unimplemented!()
    }
}
// JSProperty is meant to be ephemeral
pub struct JSProperty {
    value: Weak<RefCell<JSObject>>,
    propname: String,
}

impl JSProperty {
    fn new(refer: &JSReference, name: String) -> Self {
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
            JSObject { prototype: _, properties } => {
                let value = properties.get(&self.propname);
                value.unwrap_or(&JSValue::Undefined).clone()
            }
            _ => unimplemented!("ToObject"), // TODO
        }
    }

    fn set(&mut self, value: Self::V) {
        match &mut *(unsafe { &*self.value.as_ptr() }).borrow_mut() {
            JSObject { prototype: _, properties } => {
                let existing_value = properties.get_mut(&self.propname);
                *(existing_value.unwrap()) = value;
            }
            _ => unimplemented!("TypeError"), // TODO: return TypeError
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
            Self::Stack(value, kind, depth) => {
                *self = Self::Heap(
                    Rc::new(RefCell::new(value.clone())),
                    *kind,
                    *depth
                )
            },
            _ => {},
        };
        self
    }

    pub fn value(&self) -> JSValue {
        match self {
            Self::Stack(value, _, _) => value.clone(),
            Self::Heap(value, _, _) => (*(value.borrow_mut())).clone(),
        }
    }

    pub fn modify(&mut self, f: impl Fn(&mut JSValue)) {
        match self {
            Self::Stack(value, _, _) => f(value),
            Self::Heap(value, _, _) => f(&mut *(*value).borrow_mut()),
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
    // binding is a map from variable name to its ScopedVariable
    // the binding should hold the ScopedVariables in perspective of the innermost scope.
    // TODO: use a vector to emulate a stack
    binding: HashMap<String, ScopedVariable>, 
    recovery: Vec<BTreeMap<String, Option<ScopedVariable>>>,
}

impl Scope {
    pub fn new() -> Self {
        // XXX: construct binding from env
        Scope {
            binding: HashMap::new(),
            recovery: vec![BTreeMap::new()],
        }
    }

    #[inline]
    pub fn depth(&self) -> i16 {
        self.recovery.len() as i16
    }

    #[inline]
    pub fn is_parent_variable(&self, var: &ScopedVariable) -> bool {
        var.depth() < self.depth()
    }

    pub fn add_recover_variable(&mut self, name: String, value: Option<ScopedVariable>) {
        self.recovery.last_mut().unwrap().insert(name, value);
    }

    #[inline]
    pub fn variable(&self, name: &String) -> Result<&ScopedVariable, String> {
        self.binding.get(name).ok_or(format!("ReferenceError: {} is not defined", name))
    }

    // variable_mut should be de
    #[inline]
    pub fn variable_mut(&mut self, name: &String) -> Result<&mut ScopedVariable, String> {
        match self.binding.get_mut(name) {
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
    
    pub fn declare(&mut self, name: &String, kind: ast::DeclarationKind, value: Option<JSValue>) -> Result<(), String> {
        if kind == ast::DeclarationKind::Const && value.is_none() {
            panic!("const variable must be initialized");
        }
        
        let binding = self.binding.get(name).clone();

        // if there is no variable existing already, create a new one, and add it to the recovery list as to be discarded
        if binding.is_none() {
            self.add_recover_variable(name.clone(), None);
            let var = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
            self.binding.insert(name.clone(), var); 
            return Ok(())
        }
        
        let existing = binding.unwrap();
        let existing_kind = existing.kind();

        // if there is a variable existing already, check if it is a parent variable, 
        // if so, shadow it by adding it to the recovery list as to be recovered, and create a new one
        if self.is_parent_variable(existing) {
            let recover_value = Some(existing.clone());
            self.add_recover_variable(name.clone(), recover_value);
        }

        // at this point the variable is already declared in the current scope, so we just update it after checking if it is a let
        else if existing_kind != ast::DeclarationKind::Let {
            // cannot set to non-let variable
            return Err(format!("SyntaxError: redeclaration of formal parameter \"{}\"", name))
        }

        let var = ScopedVariable::new(value.unwrap_or(JSValue::Undefined), kind, self.depth());
        self.binding.insert(name.clone(), var);
        Ok(())
    }

    #[inline]
    pub fn enter(&mut self) {
        self.recovery.push(BTreeMap::new());
    }

    pub fn exit(&mut self) {
        if let Some(recovery) = self.recovery.pop() {
            for (key, entry) in recovery.iter() {
                match entry {
                    Some(existing) => {
                        self.binding.insert(key.clone(), existing.clone());
                    }
                    None => {
                        self.binding.remove(key);
                    }
                }
            }
        } else {
            panic!("exit scope without entering");
        }
    }
}
#[cfg(test)]
mod scope_tests {
    use crate::interpreter::runtime::Scope;
    use crate::interpreter::runtime::JSValue;
    use crate::ast::DeclarationKind;
    
    #[test]
    fn scope_test_simple() {
        let scope = &mut Scope::new();

        let declare_let = |scope: &mut Scope, name: &str, value: Option<JSValue>| {
            scope.declare(&name.to_string(), DeclarationKind::Let, value);
        };

        let declare_const = |scope: &mut Scope, name: &str, value: Option<JSValue>| {
            scope.declare(&name.to_string(), DeclarationKind::Const, value);
        };

        let set_var = |scope: &mut Scope, name: &str, value: JSValue| {
            scope.variable_mut(&name.to_string())
                .map(|v| v.set(value))
                .unwrap();
        };

        let assert_error = |scope: &mut Scope, name: &str| {
            assert!(scope.variable_mut(&name.to_string()).is_err());
        };

        let assert_var = |scope: &mut Scope, name: &str, value: JSValue| {
            assert_eq!(scope.variable(&name.to_string()).unwrap().value(), value);
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

        println!("1");
        declare_let(scope, "a", Some(JSValue::new_number(1)));
        assert_var(scope, "a", JSValue::new_number(1));

        println!("2");
        declare_const(scope ,"b", Some(JSValue::new_number(2)));
        assert_var(scope, "b", JSValue::new_number(2));

        println!("3");
        set_var(scope, "a", JSValue::new_number(3));
        assert_var(scope, "a", JSValue::new_number(3));

        println!("4");
        assert_error(scope, "b"); // error on const variable set

        println!("5");
        declare_let(scope, "c", None);
        assert_var(scope, "c", JSValue::new_undefined());

        println!("6");
        set_var(scope, "c", JSValue::new_number(4));
        assert_var(scope, "c", JSValue::new_number(4));

        println!("7");
        scope.enter();
        {
            println!("8");
            assert_var(scope, "a", JSValue::new_number(3));
            declare_const(scope, "a", Some(JSValue::new_number(11)));
            assert_var(scope, "a", JSValue::new_number(11));

            println!("9");
            assert_var(scope, "b", JSValue::new_number(2));
            declare_let(scope, "b", Some(JSValue::new_number(12)));
            assert_var(scope, "b", JSValue::new_number(12));

            println!("10");
            assert_error(scope, "x"); // error on undeclared variable
            declare_let(scope, "x", Some(JSValue::new_number(13)));
            assert_var(scope, "x", JSValue::new_number(13));

            println!("11");
            assert_var(scope, "c", JSValue::new_number(4));
            set_var(scope, "c", JSValue::new_number(14));
            assert_var(scope, "c", JSValue::new_number(14));
        }; 

        println!("11");
        scope.exit();

        println!("12");
        assert_var(scope, "a", JSValue::new_number(3));
        assert_var(scope, "b", JSValue::new_number(2));
        assert_var(scope, "c", JSValue::new_number(14));
        assert_error(scope, "x"); // error on undeclared variable

        set_var(scope, "a", JSValue::new_number(5));
        assert_var(scope, "a", JSValue::new_number(5));
        assert_error(scope, "b"); // error on const variable set
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

    #[inline]
    fn declare_const_variable(&mut self, name: &String, v: JSValue) -> Result<(), String> {
        self.scope.declare(name, ast::DeclarationKind::Const, Some(v))
    }

    #[inline]
    fn declare_let_variable(&mut self, name: &String, v: Option<JSValue>) -> Result<(), String> {
        self.scope.declare(name, ast::DeclarationKind::Let, v)
    }

}

impl runtime::JSContext for JSContext {
    type V = JSValue;

    fn check_early_errors(&self) -> bool {
        false // TODO
    }



    fn block_scope(&mut self, hoisted_fns: Vec<(String, Self::V)>, body: impl Fn(&mut Self)) {
        self.scope.enter();
        body(self);
        self.scope.exit();
    }

    fn extract_free_variables(&mut self, mut vars: HashSet<String>) -> HashSet<String> {
        unimplemented!()
        /*
        for var in vars.iter() {
            if self.scope.variable(var).is_ok() {
                vars.remove(var);
            }
        }
        
        vars
        */
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
        self.completion.clone()
    }

    #[inline]
    fn new_undefined(&mut self) -> Self::V {
        JSValue::Undefined
    }

    #[inline]
    fn new_null(&mut self) -> Self::V {
        JSValue::Null
    }

    #[inline]
    fn new_boolean(&mut self, b: bool) -> Self::V {
        JSValue::Boolean(b)
    }

    #[inline]
    fn new_number(&mut self, n: i64) -> Self::V {
        JSValue::Number(JSNumber::new(n))
    }

    #[inline]
    fn wrap_number(&mut self, n: &<<Self as runtime::JSContext>::V as runtime::JSValue>::N) -> Self::V {
        JSValue::Number(*n)
    }

    #[inline]
    fn new_string(&mut self, s: &String) -> Self::V {
        JSValue::String(JSString::new(s))
    }

    #[inline]
    fn wrap_string(&mut self, s: &<<Self as runtime::JSContext>::V as runtime::JSValue>::S) -> Self::V {
        JSValue::String(s.clone())
    }

    #[inline]
    fn new_array(&mut self, elements: Vec<Self::V>) -> Self::V {
        JSValue::Reference(JSReference::new_array(elements))
    }

    #[inline]
    fn new_object(&mut self, props: Vec<(runtime::PropName, Self::V)>) -> Self::V {
        JSValue::Reference(JSReference::new(JSObject::new(props)))
    }

    #[inline]
    fn new_function(&mut self, identifier: Option<String>, parameters: Vec<String>, body: &ast::FunctionExpression, captures: Vec<String>) -> Self::V {
        let mut captured_vars = Vec::with_capacity(captures.len());
        let scope = &mut self.scope;
       
        for capture in captures {
            let var = scope.variable(&capture).unwrap();
            captured_vars.push(var);
        }

        JSValue::Reference(JSReference::new_function(identifier, parameters, body, captured_vars))
    }

    fn initialize_binding(&mut self, kind: ast::DeclarationKind, name: &String, value: Option<Self::V>) -> Result<(), String> {
        self.scope.declare(name, kind, value)
    }

    fn resolve_binding(&mut self, name: &String) -> Result<JSValue, String> {
       self.scope.variable(name).map(|v| v.value())
    }

    fn set_binding(&mut self, name: &String, value: Self::V) -> Result<(), String>{
        self.scope.variable_mut(name).map(|v| v.set(value))
    }

    fn dup(&mut self, v: Self::V) -> Self::V {
        v.clone()
    }
}

#[cfg(test)]
mod context_test {
    use std::cell::RefCell;
    use crate::runtime::{JSContext, JSValue, JSNumeric};

    #[test]
    fn simple_test() {
        let mut ctx = super::JSContext::new();

        // test bindings

        let name = "name".to_string();
        let value = ctx.new_string(&"John".to_string());
        let result = ctx.initialize_binding(crate::ast::DeclarationKind::Let, &name, Some(value));
        assert!(result.is_ok());

        let stored_value = ctx.resolve_binding(&name);
        assert!(stored_value.is_ok());
        assert!(stored_value.unwrap().as_string().unwrap().to_string() == "John".to_string());

        let stored_value = ctx.resolve_binding(&"unknown".to_string());
        assert!(stored_value.is_err());

        let value = ctx.new_string(&"Jane".to_string());
        let result = ctx.set_binding(&name, value);
        assert!(result.is_ok());
        assert!(ctx.resolve_binding(&name).unwrap().as_string().unwrap().to_string() == "Jane".to_string());

        // test controls

        let result = RefCell::new(ctx.new_undefined());
        ctx.control_branch(
            |ctx| ctx.new_boolean(true),
            |ctx| *result.borrow_mut() = ctx.new_string(&"consequent".to_string()),
            |ctx| *result.borrow_mut() = ctx.new_string(&"alternate".to_string()),
        );

        assert!(result.borrow().as_string().unwrap().to_string() == "consequent".to_string());

        let result = RefCell::new(ctx.new_undefined());
        ctx.control_branch(
            |ctx| ctx.new_boolean(false),
            |ctx| *result.borrow_mut() = ctx.new_string(&"consequent".to_string()),
            |ctx| *result.borrow_mut() = ctx.new_string(&"alternate".to_string()),
        );

        assert!(result.borrow().as_string().unwrap().to_string() == "alternate".to_string());

        // for (let i = 0; i < 10; i++) { }
        let condition = RefCell::new(ctx.new_boolean(true));
        let count = RefCell::new(ctx.new_number(0));
        let ten = super::JSNumber::new(10);
        ctx.control_loop(
            |ctx| condition.borrow().clone(),
            |ctx| { 
                if let super::JSValue::Number(value) = &mut *count.borrow_mut() {
                    println!("count: {}", value.to_int64().unwrap());
                    if !value.op_lt(&ten) {
                        *condition.borrow_mut() = ctx.new_boolean(false);
                        return
                    } 
                    value.op_inc();
                } else {
                    panic!("count is not a number");
                }
            },
        );

        assert!(count.borrow().as_number().unwrap().to_int64().unwrap() == 10);
    }
}