

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

pub trait Boolean {
    fn to_bool(&self) -> bool;
}

pub trait Number {
    fn add(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn sub(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn mul(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn div(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn modulo(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn pow(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitand(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitor(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitxor(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitnot(&mut self) -> Result<&mut Self, str>;
    fn lshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn rshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn urshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
}

pub trait Bigint {
    fn add(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn sub(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn mul(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn div(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn modulo(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn pow(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitand(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitor(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitxor(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn bitnot(&mut self) -> Result<&mut Self, str>;
    fn lshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn rshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn urshift(&mut self, other: &Self) -> Result<&mut Self, str>;
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
}

pub trait String {
    fn concat(&mut self, other: &Self) -> Result<&mut Self, str>;
}

pub trait Property {
    fn stringify(&self) -> str;
}

pub trait Reference {
    type V: Value;
    type P: Property;

    fn get(&self, prop: &Self::P) -> &Self::V;
    fn set(&self, prop: &Self::P, val: &Self::V);
    fn create_data_property(&self, prop: &Self::P, val: &Self::V);
    fn create_method_property(&self, prop: &Self::P, val: &Self::V);
}

pub trait Closure {
    type V: Value;

    fn call(&self, args: &[Self::V]) -> Self::V;
}

pub trait Value {
    type N: Number;
    type B: Bigint;
    type S: String;
    type R: Reference;
    type C: Closure;

    // Type switch
    fn is_null(&self) -> bool;
    fn is_undefined(&self) -> bool;
    fn as_boolean(&self) -> Option<bool>;
    fn as_number(&self) -> Option<&Self::N>;
    fn as_bigint(&self) -> Option<&Self::B>;
    fn as_string(&self) -> Option<&Self::S>;

    fn as_closure(&self) -> Option<&Self::C>;
    fn as_reference(&self) -> Option<&Self::R>;

    // Type coersion as defined in https://262.ecma-international.org/9.0/#sec-type-conversion
    fn to_boolean(&self) -> bool;
    fn to_integer(&self) -> &Self::N;
    fn to_string(&self) -> &Self::S;
    
    fn to_object(&self) -> &Self::R;
}

pub trait Context {
    type V: Value;

    fn new_undefined() -> Self::V;
    fn new_null() -> Self::V;
    fn new_boolean(b: bool) -> Self::V;
    fn new_number(n: i64) -> Self::V;
    fn new_bigint(n: &[i32]) -> Self::V;
    fn new_string(s: &str) -> Self::V;

    fn new_array(vs: &[Self::V]) -> Self::V;
    fn new_tuple(vs: &[Self::V]) -> Self::V;
}