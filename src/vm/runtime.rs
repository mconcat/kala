
// traits do not have runtime overhead(unless &dyn), so we use trait wrappers.

// ECMA boolean value
trait Boolean {
    pub fn new(value: bool) -> Self;

    fn and(&mut self, other: &Self) -> OpResult;
    fn or(&mut self, other: &Self) -> OpResult;
    fn not(&mut self) -> OpResult;
    fn xor(&mut self, other: &Self) -> OpResult;
}

struct MockBoolean {
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

enum OpResult {
    Ok,
    TypeError,
}

#[inline]
fn ok(value: ()) -> OpResult {
    OpResult::Ok
}

// ECMA number/bigint value
// Number values are represented as a 53-bit integer, without any fractional part.
// Bigint values are represented as a vector of 64-bit integers with a sign flag.
pub trait Numeric {
    fn add(&mut self, other: &Self) -> OpResult;
    fn sub(&mut self, other: &Self) -> OpResult;
    fn mul(&mut self, other: &Self) -> OpResult;
    fn div(&mut self, other: &Self) -> OpResult;
    fn modulo(&mut self, other: &Self) -> OpResult;
    fn pow(&mut self, other: &Self) -> OpResult;
    fn bitand(&mut self, other: &Self) -> OpResult;
    fn bitor(&mut self, other: &Self) -> OpResult;
    fn bitxor(&mut self, other: &Self) -> OpResult;
    fn bitnot(&mut self) -> OpResult;
    fn lshift(&mut self, other: &Self) -> OpResult;
    fn rshift(&mut self, other: &Self) -> OpResult;
    fn urshift(&mut self, other: &Self) -> OpResult;
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
    fn gt(&self, other: &Self) -> bool;
    fn le(&self, other: &Self) -> bool;
    fn ge(&self, other: &Self) -> bool;
}

pub enum MockNumeric {
    NaN,
    Infinity(bool),
    Number(i64),
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
            MockNumeric::Number(n) => {
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
            MockNumeric::Number(n) => {
                *n % 4294967296;
            },
            _ => 0, // TODO XXX
        }
    }
}

impl Numeric for MockNumeric {
    #[inline]
    fn add(&mut self, other: &Self) -> OpResult {
        match other {
            MockNumeric::NaN => self.assign(MockNumeric::NaN),
            _ => match self {
                MockNumeric::NaN => Ok,
                MockNumeric::Infinity(x) => match other {
                    MockNumeric::Infinity(y) => {
                        if *x == *y {
                            Ok
                        } else {
                            self.assign(MockNumeric::NaN)
                        }
                    }
                    _ => Ok // ignore other cases
                }
                MockNumeric::Number(x) => match other {
                    MockNumeric::Infinity(_) => self.assign(other),
                    MockNumeric::Number(y) => ok(*x = *x + *y),
                    MockNumeric::Bigint(_) => TypeError,
                },
                MockNumeric::Bigint(_, _) => match other {
                    MockNumeric::Infinity(_) => self.assign(other),
                    MockNumeric::Number(_) => TypeError,
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::Bigint(false, vec![])), // TODO XXX
                },
            }
        }
    }

    #[inline]
    fn sub(&mut self, other: &Self) -> OpResult {
        match other {
            MockNumeric::NaN => self.assign(MockNumeric::NaN),
            _ => match self {
                MockNumeric::NaN => Ok,
                MockNumeric::Infinity(x) => match other {
                    MockNumeric::Infinity(y) => {
                        if *x != *y {
                            Ok
                        } else {
                            self.assign(MockNumeric::NaN)
                        }
                    }
                    _ => Ok,
                }
                MockNumeric::Number(x) => match other {
                    MockNumeric::Infinity(y) => ok(*y = !*y),
                    MockNumeric::Number(y) => ok(*x = *x - *y),
                    MockNumeric::Bigint(_) => TypeError,
                },
                MockNumeric::Bigint(_, _) => match other {
                    MockNumeric::Infinity(y) => ok(*y = !*y),
                    MockNumeric::Number(_) => TypeError,
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::Bigint(false, vec![])), // TODO XXX
                },
            }
        }
    }

    #[inline]
    fn mul(&mut self, other: &Self) -> OpResult {
        match other {
            MockNumeric::NaN => self.assign(MockNumeric::NaN),
            _ => match self {
                MockNumeric::NaN => Ok,
                MockNumeric::Infinity(x) => match other {
                    MockNumeric::Infinity(y) => ok(*x = *x == *y),
                    MockNumeric::Number(0) => MockNumeric::NaN,
                    MockNumeric::Number(y) => ok(*x = *x == (y>=0)),
                    MockNumeric::Bigint(s, _) => ok(*x = *x == *s),
                }
                MockNumeric::Number(0) => match other {
                    MockNumeric::Infinity(_) => self.assign(MockNumeric::NaN),
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::NaN),
                    _ => Ok,
                },
                MockNumeric::Number(x) => match other {
                    MockNumeric::Infinity(y) => self.assign(MockNumeric::Infinity((x>=0) == y)),
                    MockNumeric::Number(y) => ok(*x = *x * *y),
                    _ => TypeError
                },
                MockNumeric::Bigint(s, _) => match other {
                    MockNumeric::Infinity(y) => self.assign(MockNumeric::Infinity(s == y)),
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::Bigint(false, vec![])), // TODO XXX
                    _ => TypeError,
                },
            }
        }
    }

    #[inline]
    fn div(&mut self, other: &Self) -> OpResult {
        match other {
            MockNumeric::NaN => self.assign(MockNumeric::NaN),
            _ => match self {
                MockNumeric::NaN => Ok,
                MockNumeric::Infinity(x) => match other {
                    MockNumeric::Infinity(y) => self.assign(MockNumeric::NaN),
                    MockNumeric::Number(y) => ok(*x = *x == (y>=0)),
                    MockNumeric::Bigint(s, _) => ok(*x = *x == *s),
                }
                MockNumeric::Number(0) => match other {
                    MockNumeric::Number(0) => self.assign(MockNumeric::NaN),
                    MockNumeric::BigInt(_, _) => TypeError,
                    _ => Ok,
                },
                MockNumeric::Number(x) => match other {
                    MockNumeric::Number(0) => self.assign(MockNumeric::Infinite(x>=0)),
                    MockNumeric::Infinity(y) => ok(*x = 0),
                    MockNumeric::Number(y) => ok(*x = *x / *y),
                    _ => TypeError
                },
                MockNumeric::Bigint(_, _) => match self {
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::Bigint(false, vec![])), // TODO XXX
                    _ => TypeError,
                },
            }
        }
    }

    #[inline]
    fn modulo(&mut self, other: &Self) -> OpResult {
        match other {
            MockNumeric::NaN => self.assign(MockNumeric::NaN),
            _ => match self {
                MockNumeric::NaN => Ok,
                MockNumeric::Infinity(x) => assign(MockNumeric::NaN),
                MockNumeric::Number(x) => match other {
                    MockNumeric::Infinity(y) => Ok,
                    MockNumeric::Number(0) => self.assign(MockNumeric::NaN),
                    MockNumeric::Number(y) => ok(*x = *x % *y),
                    _ => TypeError
                },
                MockNumeric::Bigint(_, _) => match self {
                    MockNumeric::Bigint(_, _) => self.assign(MockNumeric::Bigint(false, vec![])), // TODO XXX
                    _ => TypeError,
                },
            }
        }
    }

    #[inline]
    fn pow(&mut self, other: &Self) -> OpResult {} // TODO XXX

    #[inline]
    fn bitand(&mut self, other: &Self) -> OpResult {
        let v = self.to_int32() & other.to_int32();
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        }  
    }

    #[inline]
    fn bitor(&mut self, other: &Self) -> OpResult {
        let v = self.to_int32() | other.to_int32();
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        } 
    }

    #[inline]
    fn bitxor(&mut self, other: &Self) -> OpResult {
        let v = self.to_int32() ^ other.to_int32();
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        }    
    }

    #[inline]
    fn bitnot(&mut self) -> OpResult {
        let v = self.to_int32() ^ 0xFFFFFFFF;
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        }
    }

    #[inline]
    fn bitlshift(&mut self, other: &Self) -> OpResult {
        let v = self.to_int32() << other.to_int32();
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        } 
    }

    #[inline]
    fn bitrshift(&mut self, other: &Self) -> OpResult {
        let v = self.to_int32() >> other.to_int32();
        match self {
            MockNumeric::Number(x) => ok(*x = v),
            _ => self.assign(MockNumeric::Number(v)),
        }   
    }

    #[inline]
    fn biturshift(&mut self, other: &Self) -> OpResult {
        // TODO
    }

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self {
            MockNumeric::NaN => match other {
                MockNumeric::NaN => true,
                _ => false,
            },
            MockNumeric::Infinity(x) => match other {
                MockNumeric::Infinity(y) => *x == *y,
                _ => false,
            },
            MockNumeric::Number(x) => match other {
                MockNumeric::Number(y) => *x == *y,
                _ => false,
            },
            MockNumeric::Bigint(xs, xv) => match other {
                MockNumeric::Bigint(ys, yv) => xs == ys && xv == yv,
                _ => false,
            },
        }
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    // Copilot wrote, need to check
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        match self {
            MockNumeric::NaN => match other {
                MockNumeric::NaN => false,
                _ => false,
            },
            MockNumeric::Infinity(x) => match other {
                MockNumeric::Infinity(y) => *x < *y,
                _ => false,
            },
            MockNumeric::Number(x) => match other {
                MockNumeric::Number(y) => *x < *y,
                _ => false,
            },
            MockNumeric::Bigint(xs, xv) => match other {
                MockNumeric::Bigint(ys, yv) => xs < ys || (xs == ys && xv < yv),
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn le(&self, other: &Self) -> bool {
        match self {
            MockNumeric::NaN => match other {
                MockNumeric::NaN => false,
                _ => false,
            },
            MockNumeric::Infinity(x) => match other {
                MockNumeric::Infinity(y) => *x <= *y,
                _ => false,
            },
            MockNumeric::Number(x) => match other {
                MockNumeric::Number(y) => *x <= *y,
                _ => false,
            },
            MockNumeric::Bigint(xs, xv) => match other {
                MockNumeric::Bigint(ys, yv) => xs < ys || (xs == ys && xv <= yv),
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        match self {
            MockNumeric::NaN => match other {
                MockNumeric::NaN => false,
                _ => false,
            },
            MockNumeric::Infinity(x) => match other {
                MockNumeric::Infinity(y) => *x > *y,
                _ => false,
            },
            MockNumeric::Number(x) => match other {
                MockNumeric::Number(y) => *x > *y,
                _ => false,
            },
            MockNumeric::Bigint(xs, xv) => match other {
                MockNumeric::Bigint(ys, yv) => xs > ys || (xs == ys && xv > yv),
                _ => false,
            },
        }
    }

    // Copilot wrote, need to check
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        match self {
            MockNumeric::NaN => match other {
                MockNumeric::NaN => false,
                _ => false,
            },
            MockNumeric::Infinity(x) => match other {
                MockNumeric::Infinity(y) => *x >= *y,
                _ => false,
            },
            MockNumeric::Number(x) => match other {
                MockNumeric::Number(y) => *x >= *y,
                _ => false,
            },
            MockNumeric::Bigint(xs, xv) => match other {
                MockNumeric::Bigint(ys, yv) => xs > ys || (xs == ys && xv >= yv),
                _ => false,
            },
        }
    }
}





/* 
V128(i64, i64),
V256(i64, i64, i64, i64),
// VInf(bool, u8, Vec<i64>),
}
*/

trait String {
    fn concat(&mut self, other: &Self) -> OpResult;
}

struct MockString {
    value: str
}

impl String for MockString {
    fn concat(&mut self, other: &Self) -> OpResult {
        self.value.push_str(other.value);
        ok()
    }
}

/*
enum String {
    Short(i64), // ascii string less than length 8
    Vector(Vec<u16>), // UTF-16 string
    // GlobalConst(i64),
}
*/

trait Property {
    fn get() -> &Value;
    fn set(val: &Value);
}

trait Class {
    fn add_property(&mut self, name: &str, prop: Box<Property>);
}

trait Object {
    fn class(&self) -> &Class; // hiddenclass
    
    fn property(&self, name: &str) -> Option<&Property>;
    fn property_known(&self, id: &i32) -> Option<&Property>;
    fn has_property(&self, name: &str) -> bool;
    fn has_property_known(&self, id: &i32) -> bool;
    fn delete_property(&self, name: &str) -> bool;
    fn delete_property_known(&self, id: &i32) -> bool;
    
    fn array(&self) -> Option<&Array>;
}

struct MockObject {
    class: MockClass,
    properties: HashMap<String, Property>,
    array: Option<Array>,
}

// RuntimeValue is either
// - a primitive value (i.e. a number, string, boolean, null, etc)
// - a reference to an object in the heap
// - a reference to a function in the heap
// - a reference to a array in the heap
// TODO: flatten the enum hierarchy to reduce memory footprint
// TODO: use tagged pointer
enum Value {
    Undefined,
    Null,
    Boolean(Boolean),
    Integer(Integer),
    // Number(Number), // unused
    String(String),
    Bigint(Bigint),

    Object(Object),
    Property(Property),

    Closure(Closure),

    // Optimization for arrays
    // Should be cocered to object when becomes not a pure array
    Array(Array),
}

pub trait Value {
    fn is_undefined(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_boolean(&self) -> bool;
    fn is_integer(&self) -> bool;
    fn is_string(&self) -> bool;
    fn is_bigint(&self) -> bool;
    fn is_object(&self) -> bool;
    fn is_property(&self) -> bool;
    fn is_closure(&self) -> bool;
    fn is_array(&self) -> bool;

    fn as_undefined(&self) -> Option<&Undefined>;
    fn as_null(&self) -> Option<&Null>;
    fn as_boolean(&self) -> Option<&Boolean>;
    fn as_integer(&self) -> Option<&Integer>;
    fn as_string(&self) -> Option<&String>;
    fn as_bigint(&self) -> Option<&Bigint>;
    fn as_object(&self) -> Option<&Object>;
    fn as_property(&self) -> Option<&Property>;
    fn as_closure(&self) -> Option<&Closure>;
    fn as_array(&self) -> Option<&Array>;

    fn as_undefined_mut(&mut self) -> Option<&mut Undefined>;
    fn as_null_mut(&mut self) -> Option<&mut Null>;
    fn as_boolean_mut(&mut self) -> Option<&mut Boolean>;
    fn as_integer_mut(&mut self) -> Option<&mut Integer>;
    fn as_string_mut(&mut self) -> Option<&mut String>;
    fn as_bigint_mut(&mut self) -> Option<&mut Bigint>;
    fn as_object_mut(&mut self) -> Option<&mut Object>;
    fn as_property_mut(&mut self) -> Option<&mut Property>;
    fn as_closure_mut(&mut self) -> Option<&mut Closure>;
    fn as_array_mut(&mut self) -> Option<&mut Array>;
}